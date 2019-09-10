use crate::error::Error;
use reqwest::{header::HeaderMap, Client, Response};
use std::collections::HashMap;
use std::str::FromStr;

pub type Resp<T> = Result<T, Error>;

pub fn post<T>(
    url: &str,
    headers: &HashMap<String, String>,
    form: &HashMap<String, String>,
) -> Resp<T>
where
    T: serde::de::DeserializeOwned,
{
    let client = Client::new();
    let client = client.post(url);
    let resp = client.headers(_build_headers(headers)).form(form).send();
    _build_response(resp)
}

pub fn get<T>(url: &str, headers: &HashMap<String, String>) -> Resp<T>
where
    T: serde::de::DeserializeOwned,
{
    let client = Client::new();
    let client = client.get(url);
    let resp = client.headers(_build_headers(headers)).send();
    _build_response(resp)
}

pub fn post_json<F, T>(url: &str, headers: &mut HashMap<String, String>, form: &F) -> Resp<T>
where
    F: serde::Serialize + ?Sized,
    T: serde::de::DeserializeOwned,
{
    let client = Client::new();
    let client = client.post(url);
    headers.insert("Content-Type".into(), "application/json".into());
    let resp = client.headers(_build_headers(headers)).json(form).send();
    _build_response(resp)
}

fn _build_headers(map: &HashMap<String, String>) -> HeaderMap {
    let mut header_map = HeaderMap::new();
    map.iter().for_each(|(k, v)| {
        header_map.insert(
            reqwest::header::HeaderName::from_str(k.as_str()).unwrap(),
            v.parse().unwrap(),
        );
    });
    header_map
}

fn _build_response<T>(mut r: reqwest::Result<Response>) -> Resp<T>
where
    T: serde::de::DeserializeOwned,
{
    match r {
        Ok(ref mut r) if r.status().is_success() => Ok(r.json().unwrap()),
        Ok(r) => Err(r.into()),
        Err(err) => Err(err.into()),
    }
}
