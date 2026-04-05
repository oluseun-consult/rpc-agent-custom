use crate::error::{ApiError, Error};

#[test]
fn api_error_from_provider_error() {
    let err = Error::ProviderError("fail".to_string());
    let api: ApiError = err.into();
    assert_eq!(api.to_string(), "500: fail");
}

#[test]
fn api_error_from_auth_error() {
    let err = Error::AuthenticationError("bad auth".to_string());
    let api: ApiError = err.into();
    assert_eq!(api.to_string(), "401: bad auth");
}
