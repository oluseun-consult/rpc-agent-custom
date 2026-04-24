use pyo3::{
    PyResult, Python,
    types::{PyAnyMethods as _, PyDict, PyModule},
};

use crate::{error::Error, providers::CompletionProvider};

#[derive(Clone)]
/// A custom SageMaker AI client that wraps the AWS SDK for SageMaker.
pub struct LocalInferenceAI {
    pub function: String,
    pub script_name: String,
}

#[derive(serde::Serialize)]
struct LocalInferenceResult {
    label: String,
    score: f64,
}

impl TryFrom<LocalInferenceResult> for String {
    type Error = Error;

    fn try_from(value: LocalInferenceResult) -> Result<Self, Self::Error> {
        serde_json::to_string(&value).map_err(Self::Error::SerializationError)
    }
}

impl LocalInferenceAI {
    /// Sets up a new LocalInferenceAI client using the provided model directory.
    pub async fn setup(script_name: String, function: String) -> Self {
        Self {
            function,
            script_name,
        }
    }

    /// Invokes the local inference model with the given prompt and returns the response.
    async fn invoke(&self, prompt: &str) -> Result<String, Error> {
        let p: PyResult<LocalInferenceResult> = Python::with_gil(|py| {
            let inference = PyModule::import_bound(py, self.script_name.as_str())?;
            let result = inference.getattr(self.function.as_str())?.call1((prompt,));
            match result {
                Ok(obj) => {
                    let result_dict = obj.downcast::<PyDict>()?;
                    let label: String = result_dict.get_item("label")?.extract()?;
                    let score: f64 = result_dict.get_item("score")?.extract()?;
                    Ok(LocalInferenceResult { label, score })
                }
                Err(e) => Err(e),
            }
        });

        // Now handle conversion to your Error type outside the GIL closure
        let res = match p {
            Ok(val) => val.try_into()?,
            Err(e) => {
                // Try to extract the error string from the PyErr
                Python::with_gil(|py| {
                    let msg: String = e
                        .value_bound(py)
                        .extract()
                        .unwrap_or_else(|_| e.to_string());
                    Err(Error::ProviderError(msg))
                })?
            }
        };

        Ok(res)
    }
}

#[async_trait::async_trait]
impl CompletionProvider for LocalInferenceAI {
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(name = "local_inference.chat", skip(self, prompt))
    )]
    async fn chat(&self, prompt: &str) -> Result<String, Error> {
        self.invoke(prompt).await
    }
}
