use serde::Deserialize;

/// Denotes the way creating a payment can fail.
#[derive(Debug, Deserialize)]
pub struct Error {
    /// A error message with information about why the request failed
    message: String,
    /// If the cause of the error is a bad http reponse, this is the status code
    status: Option<u16>,
    /// If the cause of the error is a bad http reponse, this is the remote url
    /// (either starting with https://api.sandbox.paypal.com or https://api.paypal.com/v1)
    remote: Option<String>,
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self {
            message: err.to_string(),
            status: err.status().map(|status| status.as_u16()),
            remote: err.url().map(|url| url.to_string()),
        }
    }
}

impl From<reqwest::Response> for Error {
    fn from(mut response: reqwest::Response) -> Self {
        Self {
            message: response.text().unwrap(),
            status: Some(response.status().as_u16()),
            remote: Some(response.url().to_string()),
        }
    }
}
