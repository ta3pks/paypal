use crate::error::Error;
use crate::types::ResponseAccessToken;
use base64::encode;

/// This function uses your `client id` and `secret`, and creates a bearer token that
/// you must use to get access to the paypal endpoints
pub fn get_token(id: &str, secret: &str) -> Result<ResponseAccessToken, Error> {
    let ep = crate::_make_endpoint("/v1/oauth2/token");
    let auth_header = prepare_auth_header(id, secret);
    let mut headers = std::collections::HashMap::<String, String>::new();
    headers.insert("Authorization".into(), auth_header);
    headers.insert(
        "content-type".into(),
        "application/x-www-form-urlencoded".into(),
    );
    let mut form = std::collections::HashMap::new();
    form.insert("grant_type".into(), "client_credentials".into());
    crate::request::post(&ep, &headers, &form)
}

fn prepare_auth_header(id: &str, secret: &str) -> String {
    format!("Basic {}", base64_credentials(id, secret))
}

fn base64_credentials(id: &str, secret: &str) -> String {
    let mut key = String::from(id);
    key.push_str(":");
    key.push_str(secret);
    encode(&key)
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref CLIENT_ID: String = {
            dotenv::dotenv().ok();
            std::env::var("CLIENT_ID").expect("`CLIENT_ID` env parameter to be set")
        };
        static ref SECRET: String = {
            dotenv::dotenv().ok();
            std::env::var("SECRET").expect("`SECRET` env parameter to be set")
        };
    }

    #[test]
    fn test_base64_credentials() {
        assert_eq!(
            "bmlrb3M6cGFzc3dvcmQ=",
            base64_credentials("nikos", "password")
        )
    }

    #[test]
    fn test_prepare_auth_header() {
        assert_eq!(
            "Basic bmlrb3M6cGFzc3dvcmQ=",
            prepare_auth_header("nikos", "password")
        )
    }

    #[test]
    fn test_get_token() {
        get_token(&CLIENT_ID, &SECRET).unwrap();
    }
}
