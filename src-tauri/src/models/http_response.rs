use serde::{Deserialize, Serialize};
use tauri_plugin_http::reqwest::Error;

#[derive(Serialize, Deserialize)]
pub struct HttpResponse {
    pub headers: String,
    pub body: String,
    pub code: u16,
    pub timing: f64,
    pub err: String,
}

impl HttpResponse {
    pub fn with_error(err: Error, timing: f64)-> HttpResponse {
        HttpResponse {
            headers: String::new(),
            body: String::new(),
            code: 0,
            timing,
            err: format!("{:?}", err)
        }
    }
}