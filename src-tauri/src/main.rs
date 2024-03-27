// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use serde::{Deserialize, Serialize};
use tauri::api::http::{ ClientBuilder, HttpRequestBuilder, ResponseType };

#[derive(Serialize, Deserialize)]
struct HttpResponse<'a> {
    headers: &'a str,
    body: &'a str,
    code: i32,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn request(method: String, url: String) -> String {
    let client = ClientBuilder::new()
        .max_redirections(3)
        .build()
        .unwrap();

    let request_instance = HttpRequestBuilder::new(method, url).expect("Unreachable runtime");
    let request = request_instance.response_type(ResponseType::Text);

    if let Ok(response) = client.send(request).await {
        let body= response.read().await.expect("Something went wrong");
        let headers = format!("{:?}", body.headers);
        let response_struct = HttpResponse {
            headers: &headers.as_str(),
            body: body.data.as_str().unwrap(),
            code: 200
        };
        
        format!("{:?}", serde_json::to_string(&response_struct).unwrap())
    } else {
        format!("Hello, Rust! You've been greeted from Rust!")
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, request])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
