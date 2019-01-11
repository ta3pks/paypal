use reqwest::{
    header::HeaderMap,
    Client,
    Response,
};
use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;
#[derive(Debug)]
pub struct ResponseData
{
    pub status: String,
    pub headers: Vec<(String, String)>,
    pub body: String,
}
impl ResponseData //{{{
{
    pub fn status_code(&self) -> i64
    {
        self.status
            .split_whitespace()
            .next()
            .unwrap()
            .parse()
            .unwrap()
    }
} //}}}
pub fn post(
    url: &str,
    headers: &HashMap<String, String>,
    form: &HashMap<String, String>,
) -> Result<ResponseData, String>
{
    let client = Client::new();
    let client = client.post(url);
    let resp = client.headers(*_build_headers(headers)).form(form).send();
    _build_response(resp)
}
pub fn get(url: &str, headers: &HashMap<String, String>) -> Result<ResponseData, String>
{
    let client = Client::new();
    let client = client.get(url);
    let resp = client.headers(*_build_headers(headers)).send();
    _build_response(resp)
}

pub fn post_json<T>(
    url: &str,
    headers: &mut HashMap<String, String>,
    form: &T,
) -> Result<ResponseData, String>
where
    T: serde::Serialize + ?Sized,
{
    let client = Client::new();
    let client = client.post(url);
    headers.insert("Content-Type".into(), "application/json".into());
    let resp = client.headers(*_build_headers(headers)).json(form).send();
    _build_response(resp)
}

fn _build_headers<'a>(map: &'a HashMap<String, String>) -> Box<HeaderMap> //{{{
{
    let mut header_map = Box::new(HeaderMap::new());
    (*map).iter().for_each(|(k, v)| {
        (*header_map).insert(
            reqwest::header::HeaderName::from_str(k.as_str()).unwrap(),
            v.parse().unwrap(),
        );
    });
    header_map
} //}}}
fn _build_response(r: reqwest::Result<Response>) -> Result<ResponseData, String> //{{{
{
    if let Ok(mut r) = r
    {
        let mut _headers = vec![];
        r.headers()
            .iter()
            .for_each(|(k, v)| _headers.push((k.to_string(), v.to_str().unwrap().to_string())));
        Ok(ResponseData {
            status: r.status().to_string(),
            headers: _headers,
            body: r.text().unwrap(),
        })
    }
    else
    {
        Err(r.unwrap_err().description().to_string())
    }
} //}}}
