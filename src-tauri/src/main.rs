// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::{collections::HashMap, str::FromStr};

use serde::{Deserialize, Serialize};
use tauri::http::{header::{HeaderMap, HeaderValue}, Method};
use tauri_plugin_http::reqwest;

#[derive(Serialize, Deserialize)]
struct HttpResponse<'a> {
    headers: &'a str,
    body: &'a str,
    code: u16,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn request(
    method: String,
    url: String,
    body: String,
    headers: HashMap<String, String>,
) -> String {
    let client = reqwest::Client::new();

    let mut header = HeaderMap::new();

    headers.into_iter().for_each(|(k, v)| {
        let static_key: &'static str = Box::leak(k.into_boxed_str());
        let static_val: &'static str = Box::leak(v.into_boxed_str());

        header.insert(static_key, HeaderValue::from_str(static_val).unwrap());
    });

    header.append("content-type", HeaderValue::from_str("application/json").unwrap());

    let res = client.request(Method::from_str(method.as_str()).unwrap(), url)
        .body(body)
        .headers(header);


    if let Ok(response) = res.send().await {
        let headers = format!("{:?}", response.headers());
        let status = response.status();
        let body = response.text().await.expect("Parse error");
        let response_struct = HttpResponse {
            headers: &headers.as_str(),
            body: &body,
            code: status.as_u16(),
        };
        serde_json::to_string_pretty(&response_struct).unwrap()
    } else {
        format!("Something went wrong!")
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, request])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
