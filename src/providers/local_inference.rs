use std::ffi::OsStr;

use crate::{error::Error, providers::CompletionProvider};

#[derive(Clone)]
/// A custom SageMaker AI client that wraps the AWS SDK for SageMaker.
pub struct LocalInferenceAI {
    model_dir: String,
    python_path: String,
}

#[derive(serde::Serialize)]
struct Prompt<'a> {
    inputs: &'a str,
}

impl<'a> AsRef<OsStr> for Prompt<'a> {
    fn as_ref(&self) -> &OsStr {
        OsStr::new(self.inputs)
    }
}

impl LocalInferenceAI {
    /// Sets up a new LocalInferenceAI client using the provided model directory.
    pub async fn setup(model_dir: &str, python_path: String) -> Self {
        Self {
            model_dir: model_dir.to_string(),
            python_path,
        }
    }

    /// Invokes the local inference model with the given prompt and returns the response.
    async fn invoke(&self, prompt: &str) -> Result<String, Error> {
        let output = std::process::Command::new(&self.python_path)
            .arg("local_inference.py")
            .arg("--text")
            .arg(Prompt { inputs: prompt })
            .arg("--model_dir")
            .arg(&self.model_dir)
            .output()
            .map_err(Error::Io)?;

        if !output.status.success() {
            return Err(Error::ProviderError(
                String::from_utf8_lossy(&output.stderr).into_owned(),
            ));
        }

        let response = String::from_utf8_lossy(&output.stdout);
        Ok(response.into_owned())
    }
}

#[async_trait::async_trait]
impl CompletionProvider for LocalInferenceAI {
    async fn chat(&self, prompt: &str) -> Result<String, Error> {
        self.invoke(prompt).await
    }
}
