// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::{collections::HashMap, str::FromStr, time::Instant};
use diesel::{sql_types::Json, RunQueryDsl};
use model::{History, NewHistory};
use schema::histories;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tauri_plugin_http::reqwest::{self, header::{HeaderMap, HeaderName, HeaderValue}, Method};

use crate::db::estabilish_connection;
mod db;
mod model;
mod schema;

#[derive(Serialize, Deserialize)]
struct HttpResponse<'a> {
    headers: &'a str,
    body: &'a str,
    code: u16,
    timing: f64
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn get_history() -> String {
    use self::schema::histories::dsl::histories;
    let connection = &mut estabilish_connection();
    
    let list = histories.load::<History>(connection)
        .expect("Error loading users");

    serde_json::to_string_pretty(&list).unwrap()
}

#[tauri::command]
async fn get_history_by_id(id: i128) -> String {
    use self::schema::histories::dsl::histories;
    let connection = &mut estabilish_connection();
    
    // let history = histories.find(id)
    //     .first(connection)
    //     .expect("Error loading users");

    // serde_json::to_string_pretty(&history).unwrap()
    unimplemented!()
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
    headers.clone().into_iter()
        .for_each(|(k,v)|{
            header.append(HeaderName::from_str(k.as_str()).unwrap(), HeaderValue::from_str(v.as_str()).unwrap());
        });


    header.append("content-type", HeaderValue::from_str("application/json").unwrap());

    let res = client.request(Method::from_str(method.as_str()).unwrap(), url.clone())
        .body(body.clone())
        .headers(header);

    let now = Instant::now();
    if let Ok(response) = res.send().await {
        let timing = now.elapsed();
        let history = NewHistory {url: url.clone(), method: method.clone(), body: body.clone(), headers: serde_json::to_string(&headers).unwrap() };
        let connection = &mut estabilish_connection();

        diesel::insert_into(histories::table)
            .values(&history)
            .execute(connection)
            .expect("Insert failed");

        let headers = format!("{:?}", response.headers());
        let header_map = response.headers();

        let content_type= header_map.get("content-type").unwrap().to_str().unwrap();
        let status = response.status();

        let mut body =String::new();
        if content_type.contains("json") {
            let body_json = json!(response.text().await.expect("Parse error"));
            body = serde_json::to_string_pretty(&body_json).unwrap();
        } else {
            body = response.text().await.expect("Parse error");
        }
        

        let response_struct = HttpResponse {
            headers: &headers.as_str(),
            body: &body,
            code: status.as_u16(),
            timing: timing.as_secs_f64()
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
        .plugin(tauri_plugin_sql::Builder::default().build())
        .setup(|_app| {
            // Initialize the database.
            db::init();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, request, get_history])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
