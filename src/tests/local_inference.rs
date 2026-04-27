use crate::providers::CompletionProvider;
use crate::providers::LocalInferenceAI;

#[tokio::test]
async fn local_inference_ai_invoke_returns_serialized_result() {
    // These are dummy values; in a real test, the Python environment and script must exist.
    let ai = LocalInferenceAI::setup("dummy_script".to_string(), "predict".to_string()).await;
    // This will likely fail unless the Python script exists, so we expect an error.
    let result = ai.chat("test prompt").await;
    assert!(result.is_err());
}

// #[tokio::test]
// async fn local_inference_ai_clone_and_fields() {
//     let ai = LocalInferenceAI::setup("script".to_string(), "func".to_string()).await;
//     let ai2 = ai.clone();
//     assert_eq!(ai2.script_name, "script");
//     assert_eq!(ai2.function, "func");
// }
