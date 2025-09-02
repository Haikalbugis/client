use isahc::{AsyncReadResponseExt, HttpClient};
use serde_json::Value;
use std::{collections::HashMap, error::Error};

#[derive(Clone)]
pub struct ApiClient {
    client: HttpClient,
    base_url: String,
}

impl ApiClient {
    pub fn new(
        base_url: String,
        extra_headers: Option<HashMap<String, String>>,
    ) -> Result<Self, Box<dyn Error>> {
        let mut builder = HttpClient::builder();

        if let Some(hmap) = extra_headers {
            let headers: Vec<(String, String)> = hmap.into_iter().map(|(k, v)| (k, v)).collect();
            builder = builder.default_headers(headers);
        }

        Ok(Self {
            client: builder.build()?,
            base_url: base_url.to_string(),
        })
    }

    pub async fn post_async_json(&self, path: &str, body: String) -> Result<Value, Box<dyn Error>> {
        let url = self.format_url(path);
        let mut res = self.client.post_async(url, body).await?;

        if res.status() == 200 {
            let json: Value = res.json().await?;
            Ok(json)
        } else {
            panic!("{:?}", res.text().await)
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
