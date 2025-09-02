use isahc::{
    AsyncReadResponseExt, HttpClient, Request,
    http::{HeaderName, HeaderValue},
};
use serde_json::Value;
use std::{collections::HashMap, error::Error, str::FromStr};

#[derive(Clone)]
pub struct ApiClient {
    client: HttpClient,
    default_headers: HashMap<String, String>,
    base_url: String,
}

impl ApiClient {
    pub fn new(
        base_url: String,
        default_headers: Option<HashMap<String, String>>,
    ) -> Result<Self, Box<dyn Error>> {
        let client = HttpClient::builder().build()?;

        Ok(Self {
            client,
            default_headers: default_headers.unwrap_or_default(),
            base_url,
        })
    }

    pub async fn post_async_json(
        &self,
        path: &str,
        body: String,
        extra_headers: Option<HashMap<String, String>>,
    ) -> Result<Value, Box<dyn Error>> {
        let url = self.format_url(path);

        let mut req = Request::post(&url).body(body)?;

        {
            // insert default headers
            let headers = req.headers_mut();
            for (k, v) in &self.default_headers {
                headers.insert(HeaderName::from_str(k)?, HeaderValue::from_str(v)?);
            }

            // insert extra headers
            if let Some(extra) = extra_headers {
                for (k, v) in extra {
                    headers.insert(HeaderName::from_str(&k)?, HeaderValue::from_str(&v)?);
                }
            }
        }

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
        let mut res = self.client.get_async(url).await?;

        if res.status() == 403 {
            return Err("just a moment...".into());
        }

        let json: Value = res.json().await?;

        Ok(json)
    }

    fn format_url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }
}
