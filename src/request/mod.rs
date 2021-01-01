use reqwest::{
    header::HeaderMap, header::HeaderName, header::HeaderValue, Client, Error, Response,
};

use serde_json::{json, Value};

#[derive(Debug, Clone)]
pub struct Request {
    pub url: String,
    pub body: Option<Value>,
    pub headers: Option<Value>,
    pub method: Method,
}

#[derive(Debug, Copy, Clone)]
pub enum Method {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

impl Default for Request {
    fn default() -> Self {
        Request {
            url: "".to_string(),
            body: None,
            headers: None,
            method: Method::Get,
        }
    }
}

impl Request {
    pub fn new<S: Into<String>>(
        url: S,
        body: Option<Value>,
        headers: Option<Value>,
        method: Method,
    ) -> Self {
        Request {
            url: url.into(),
            body,
            headers,
            method,
        }
    }

    pub async fn make(&mut self) -> std::result::Result<Response, Error> {
        // create client
        let client = reqwest::Client::new(); // client

        // start request
        match self.method {
            Method::Get => self.get(client).await,
            Method::Post => self.post(client).await,
            Method::Put => self.put(client).await,
            Method::Patch => self.patch(client).await,
            Method::Delete => self.delete(client).await,
        }
    }

    /// PRIVATE ///
    pub async fn get(&self, client: Client) -> Result<Response, Error> {
        let mut headers = HeaderMap::new();

        // add headers
        if let Some(value) = &self.headers {
            for (key, value) in value.as_object().unwrap() {
                let name = HeaderName::from_lowercase(&key.as_ref());
                let val = HeaderValue::from_str(&value.as_str().unwrap());

                headers.insert(&name.unwrap(), val.unwrap());
            }
        }

        client
            .post(self.url.as_str())
            .headers(headers)
            .json(&self.body.clone().unwrap_or(json!({})))
            .send()
            .await
    }

    pub async fn post(&self, client: Client) -> Result<Response, Error> {
        let mut headers = HeaderMap::new();

        // add headers
        if let Some(value) = &self.headers {
            for (key, value) in value.as_object().unwrap() {
                let name = HeaderName::from_lowercase(&key.as_ref());
                let val = HeaderValue::from_str(&value.as_str().unwrap());

                headers.insert(&name.unwrap(), val.unwrap());
            }
        }

        client
            .post(self.url.as_str())
            .headers(headers)
            .json(&self.body.clone().unwrap_or(json!({})))
            .send()
            .await
    }

    pub async fn put(&self, client: Client) -> Result<Response, Error> {
        let mut headers = HeaderMap::new();

        // add headers
        if let Some(value) = &self.headers {
            for (key, value) in value.as_object().unwrap() {
                let name = HeaderName::from_lowercase(&key.as_ref());
                let val = HeaderValue::from_str(&value.as_str().unwrap());

                headers.insert(&name.unwrap(), val.unwrap());
            }
        }

        client
            .put(self.url.as_str())
            .headers(headers)
            .json(&self.body.clone().unwrap_or(json!({})))
            .send()
            .await
    }

    pub async fn patch(&self, client: Client) -> Result<Response, Error> {
        let mut headers = HeaderMap::new();

        // add headers
        if let Some(value) = &self.headers {
            for (key, value) in value.as_object().unwrap() {
                let name = HeaderName::from_lowercase(&key.as_ref());
                let val = HeaderValue::from_str(&value.as_str().unwrap());

                headers.insert(&name.unwrap(), val.unwrap());
            }
        }

        client
            .patch(self.url.as_str())
            .headers(headers)
            .json(&self.body.clone().unwrap_or(json!({})))
            .send()
            .await
    }

    pub async fn delete(&self, client: Client) -> Result<Response, Error> {
        let mut headers = HeaderMap::new();

        // add headers
        if let Some(value) = &self.headers {
            for (key, value) in value.as_object().unwrap() {
                let name = HeaderName::from_lowercase(&key.as_ref());
                let val = HeaderValue::from_str(&value.as_str().unwrap());

                headers.insert(&name.unwrap(), val.unwrap());
            }
        }

        client
            .delete(self.url.as_str())
            .headers(headers)
            .json(&self.body.clone().unwrap_or(json!({})))
            .send()
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::request::{Method, Request};
    use serde_json::json;

    #[actix_rt::test]
    async fn test_request() {
        println!("EEEEEEEE");

        let mut request = Request {
            url: "http://localhost:9898/test".to_string(),
            body: Some(json!({
                "test": "hi"
            })),
            headers: Some(json!({
                "test": "hi"
            })),
            method: Method::Get,
        };

        let res = request.make().await;

        if !res.is_err() {
            let e = res.unwrap().text().await;
            println!("res: {}", e.unwrap());
        }
    }
}
