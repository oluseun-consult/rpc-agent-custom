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
        let p: PyResult<String> = Python::with_gil(|py| {
            let inference = PyModule::import_bound(py, self.script_name.as_str())?;
            // Get the predict function and call it with a tuple argument
            let result = inference
                .getattr(self.function.as_str())?
                .call1((prompt,))?;
            let result_dict = result.downcast::<PyDict>()?;
            let label: String = result_dict.get_item("label")?.extract()?;
            let score: f64 = result_dict.get_item("score")?.extract()?;

            let result = LocalInferenceResult { label, score };
            let serialized: String = serde_json::to_string(&result).unwrap();
            Ok(serialized)
        });

        let res = p?;

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
