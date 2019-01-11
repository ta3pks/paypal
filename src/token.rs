use base64::encode;
pub fn get_token(id: &str, secret: &str) -> Result<super::types::ResponseAccessToken, String> //{{{
{
    let ep = super::_make_endpoint("/oauth2/token");
    let auth_header = prepare_auth_header(id, secret);
    let mut headers = std::collections::HashMap::<String, String>::new();
    headers.insert("Authorization".into(), auth_header);
    headers.insert(
        "content-type".into(),
        "application/x-www-form-urlencoded".into(),
    );
    let mut form = std::collections::HashMap::new();
    form.insert("grant_type".into(), "client_credentials".into());
    match super::request::post(&ep, &headers, &form)
    {
        Ok(resp) => Ok(serde_json::from_str(&resp.body[..]).unwrap()),
        Err(err) => Err(err),
    }
} //}}}
fn prepare_auth_header(id: &str, secret: &str) -> String //{{{
{
    format!("Basic {}", base64_credentials(id, secret))
} //}}}
fn base64_credentials(id: &str, secret: &str) -> String //{{{
{
    let mut key = String::from(id);
    key.push_str(":");
    key.push_str(secret);
    encode(&key)
} //}}}
#[cfg(test)] //{{{
mod tests
{
    use super::*;
    #[test] // base64 credentials {{{
    fn test_base64_credentials()
    {
        assert_eq!(
            "bmlrb3M6cGFzc3dvcmQ=",
            base64_credentials("nikos", "password")
        )
    } //}}}
    #[test] // prepare_auth_header {{{
    fn test_prepare_auth_header()
    {
        assert_eq!(
            "Basic bmlrb3M6cGFzc3dvcmQ=",
            prepare_auth_header("nikos", "password")
        )
    } //}}}
} //}}}
