use isahc::{
    AsyncReadResponseExt, HttpClient, Request,
    http::{HeaderName, HeaderValue},
};
use serde_json::Value;
use std::{
    collections::HashMap,
    error::Error,
    str::FromStr,
    sync::{Arc, RwLock},
};

#[derive(Clone, Debug)]
pub struct ApiClient {
    client: HttpClient,
    headers: Arc<RwLock<HashMap<String, String>>>,
    base_url: String,
}

impl ApiClient {
    pub fn new(base_url: String) -> Result<Self, Box<dyn Error>> {
        let client = HttpClient::builder().build()?;

        Ok(Self {
            client,
            base_url,
            headers: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    pub async fn post_async_json(&self, path: &str, body: String) -> Result<Value, Box<dyn Error>> {
        let url = self.format_url(path);

        let mut req = Request::post(&url).body(body)?;
        self.apply_headers(&mut req)?;
        let mut res = self.client.send_async(req).await?;

        if res.status().is_success() {
            let json: Value = res.json().await?;
            Ok(json)
        } else {
            Err(format!("Request failed: {:?}", res.text().await?).into())
        }
    }

    pub async fn get_async_json(&self, path: &str) -> Result<Value, Box<dyn Error>> {
        let url = self.format_url(path);

        let mut req = Request::get(&url).body(())?;
        self.apply_headers(&mut req)?;
        let mut res = self.client.send_async(req).await?;

        if res.status().is_success() {
            let json: Value = res.json().await?;
            Ok(json)
        } else {
            Err(format!("Request failed: {:?}", res.text().await?).into())
        }
    }

    pub fn set_header(&self, key: &str, value: &str) {
        let mut headers = self.headers.write().unwrap();
        headers.insert(key.to_string(), value.to_string());
    }

    fn apply_headers<T>(&self, req: &mut Request<T>) -> Result<(), Box<dyn Error>> {
        let headers_map = self.headers.read().unwrap();
        let req_headers = req.headers_mut();

        for (k, v) in headers_map.iter() {
            req_headers.insert(HeaderName::from_str(k)?, HeaderValue::from_str(v)?);
        }

        Ok(())
    }

    fn format_url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }
}
