use std::collections::HashMap;

use leptos::{leptos_dom::logging::console_log, *};
use serde_wasm_bindgen::to_value;
use stylance::import_crate_style;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use wasm_bindgen::prelude::*;
use url::Url;

use crate::{components::{body::BodyComponent, header::Header, params::Params, response::Response}, utils::curl_parser};

import_crate_style!(style, "src/pages/quick.module.scss");

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct RequestArgs<'a> {
    url: &'a str,
    method: &'a str,
    body: &'a str,
    // form_encoded: HashMap<String, String>,
    headers: HashMap<String, String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HttpResponse {
    pub headers: String,
    pub body: String,
    pub code: i32,
    pub timing: f64
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HttpHeaders {
    pub value: RwSignal<String>,
    pub key: RwSignal<String>,
}

impl HttpHeaders {
    pub fn new()->HttpHeaders {
        HttpHeaders{ 
            value: create_rw_signal(String::new()), 
            key: create_rw_signal(String::new())
        }
    }
}

#[component]
pub fn QuickRequest(
    set_cdr: WriteSignal<bool>
) -> impl IntoView {
    let cdr = use_context::<ReadSignal<bool>>()
        .expect("there to be a `count` signal provided");

    let http_params = create_rw_signal(vec![HttpHeaders::new()]);
    let http_form_encoded = create_rw_signal(vec![HttpHeaders::new()]);
    let http_headers = create_rw_signal(vec![HttpHeaders::new()]);
    let url = create_rw_signal(String::new());
    let body_type: RwSignal<String> = create_rw_signal(String::from("raw"));
    let method: RwSignal<String> = create_rw_signal(String::from("POST"));
    let body = create_rw_signal(String::new());
    let (menu, set_menu) = create_signal(String::from("Body"));
    let (loader, set_loader) = create_signal(false);
    let response = create_rw_signal(HttpResponse { headers: String::new(), body: String::new(), code: 0, timing: 0.00 });
    
    let change_menu = move |val: String| {
        set_menu.set(val);
    };

    let update_url = move |ev| {
        let v = event_target_value(&ev);

        if v.contains("curl --location") {
            let new_url = curl_parser(&v);
            url.set(new_url);
            return;
        }
        
        url.set(v);
        let parsed_url = Url::parse(url.get().as_str()).expect("Failed to parse URL");

        let mut temp: Vec<HttpHeaders> = vec![];
        parsed_url.query_pairs()
            .for_each(|(key, value)| {
                if !key.to_string().is_empty() {
                    temp.push(HttpHeaders{ value: create_rw_signal(value.to_string()), key: create_rw_signal(key.to_string()) })
                }
            });
        http_params.set(temp);
    };

    let update_method = move |ev| {
        let v = event_target_value(&ev);
        method.set(v);
    };

    let handle_submit = move |_| {
        set_loader.set(true);
        let mut header_map: HashMap<String, String> = HashMap::new();
        let mut encoded_map: HashMap<String, String> = HashMap::new();
        http_headers.get().into_iter()
            .for_each(|v|{
                if !v.key.get().is_empty() {
                    header_map.insert(v.key.get(), v.value.get());
                }
            });

        http_form_encoded.get().into_iter()
            .for_each(|v|{
                if !v.key.get().is_empty() {
                    encoded_map.insert(v.key.get(), v.value.get());
                }
            });
            console_log("Started call");
        spawn_local(async move {
            let name = url.get_untracked();
            if name.is_empty() {
                return;
            }

            let args = to_value(&RequestArgs { 
                // form_encoded: encoded_map, 
                url: &name, 
                method: method.get().as_str(), 
                body: body.get().as_str(), 
                headers: header_map 
            }).unwrap();
            // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
            let new_msg = invoke("request", args).await.as_string().expect("Request invoke failed");
            // set_result.set(new_msg);

            let res_struct: HttpResponse = from_str(&new_msg).unwrap();
            response.set(res_struct);
            set_cdr.set(!cdr.get().clone());
        });
        set_loader.set(false);
    };

    let message = move || {
        if loader.get() {
            "Cancel"
        } else {
            "Send"
        }
    };

    let dynamic_component = move|| {
        if menu.get().eq("Body") {
            view! {
                <div>
                    <BodyComponent body http_form_encoded menu=body_type/>
                </div>
            }
        } else if menu.get().eq("Headers") {
            view! {
                <div>
                    <div>Headers</div>
                    <Header http_headers/>
                </div>
            }
        } 
        else {
            view! {
                <div>
                    <div>Query params</div>
                    <Params http_params url/>   
                </div>
            }
        }
    };

    view! {
        <div class=style::quick_container>
            <div class=style::top_input>
                <select prop:value=method on:input=update_method id="methods" name="methods">
                    <option value="POST">Post</option>
                    <option value="PUT">Put</option>
                    <option value="GET">Get</option>
                    <option value="PATCH">Patch</option>
                    <option value="DELETE">Delete</option>
                </select>
                <input on:input=update_url prop:value=url placeholder="Enter URL"></input>
                <button on:click=handle_submit>{move || message()}</button>
            </div>
            <div class=style::field_nav>
                <div on:click = move |_|{ change_menu(String::from("Params")); }>Params</div>
                <div on:click = move |_|{ change_menu(String::from("Headers")); }>Headers</div>
                <div on:click = move |_|{ change_menu(String::from("Body")); }>Body</div>
            </div>
            {move || dynamic_component()}
            <div>  
                <Show
                    when=move || { response.get().code != 0 }
                    fallback=|| view! { <div></div> }
                >
                    <Response response/>
                </Show>
            </div>
        </div>
    }
}