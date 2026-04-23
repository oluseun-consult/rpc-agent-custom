use std::ffi::OsStr;

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
    pub model_dir: String,
}

#[derive(serde::Serialize)]
struct Prompt<'a> {
    inputs: &'a str,
}

#[derive(serde::Serialize)]
struct LocalInferenceResult {
    label: String,
    score: f64,
}

impl<'a> AsRef<OsStr> for Prompt<'a> {
    fn as_ref(&self) -> &OsStr {
        OsStr::new(self.inputs)
    }
}

impl LocalInferenceAI {
    /// Sets up a new LocalInferenceAI client using the provided model directory.
    pub async fn setup(model_dir: &str, script_name: String, function: String) -> Self {
        Self {
            function,
            script_name,
            model_dir: model_dir.to_string(),
        }
    }

    /// Invokes the local inference model with the given prompt and returns the response.
    async fn invoke(&self, prompt: &str) -> Result<String, Error> {
        let p: PyResult<String> = Python::with_gil(|py| {
            let inference = PyModule::import_bound(py, self.script_name.as_str())?;
            // Get the predict function and call it with a tuple argument
            let result = inference
                .getattr(self.function.as_str())?
                .call1((Prompt { inputs: prompt }.as_ref(), &self.model_dir))?;
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
    async fn chat(&self, prompt: &str) -> Result<String, Error> {
        self.invoke(prompt).await
    }
}
