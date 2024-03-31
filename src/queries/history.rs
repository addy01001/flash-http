use leptos::*;
use leptos_query::*;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use wasm_bindgen::prelude::*;
use serde_wasm_bindgen::to_value;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

 // Query for a track.
pub fn history_query() -> QueryScope<Limit, Vec<History>> {
    create_query(
        get_histories,
        QueryOptions::default(),
    )
}
#[derive(Serialize, Deserialize)]
struct RequestArgs {}

// Make a key type.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Limit(pub i128);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct History {
    pub id: i32,
    pub url: String,
    pub body: String,
    pub headers: String,
    pub created_at: String
}

// Query fetcher.
pub async fn get_histories(id: Limit) -> Vec<History> {
    let args = to_value(&RequestArgs { }).unwrap();
    let new_msg = invoke("get_history", args).await.as_string().expect("Something went wrong");
    let res_struct: Vec<History> = from_str(&new_msg).unwrap();
    return res_struct;
}