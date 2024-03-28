use leptos::*;
use serde_wasm_bindgen::to_value;
use stylance::import_crate_style;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, Map, Value};
use wasm_bindgen::prelude::*;

use crate::components::{header::Header, response::Response};

import_crate_style!(style, "src/quick.module.scss");

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct RequestArgs<'a> {
    url: &'a str,
    method: &'a str,
    body: &'a str,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HttpResponse {
    pub headers: String,
    pub body: String,
    pub code: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HttpHeaders {
    pub value: String,
    pub key: String
}

impl HttpHeaders {
    pub fn new()->HttpHeaders {
        HttpHeaders{ value: String::new(), key: String::new() }
    }
}

#[component]
pub fn QuickRequest() -> impl IntoView {
    let (http_headers, set_http_headers) = create_signal(vec![HttpHeaders::new()]);
    let (url, set_url) = create_signal(String::new());
    let (method, set_method) = create_signal(String::from("POST"));
    let (body, set_body) = create_signal(String::new());
    let (menu, set_menu) = create_signal(String::from("Body"));
    let (loader, set_loader) = create_signal(false);
    let (response, set_response) = create_signal(HttpResponse { headers: String::new(), body: String::new(), code: 0 });

    let change_menu = move |val: String| {
        set_menu.set(val);
    };

    let update_url = move |ev| {
        let v = event_target_value(&ev);
        set_url.set(v);
    };

    let update_method = move |ev| {
        let v = event_target_value(&ev);
        set_method.set(v);
    };

    let update_body = move |ev: ev::Event| {
        let v = event_target_value(&ev);
        set_body.set(v);
    };

    let handle_submit = move |_| {
        set_loader.set(true);
        spawn_local(async move {
            let name = url.get_untracked();
            if name.is_empty() {
                return;
            }

            let args = to_value(&RequestArgs { url: &name, method: method.get().as_str(), body: body.get().as_str() }).unwrap();
            // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
            let new_msg = invoke("request", args).await.as_string().expect("Something went wrong");
            // set_result.set(new_msg);

            let res_struct: HttpResponse = from_str(&new_msg).unwrap();
            set_response.set(res_struct);
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
                    <textarea class=style::textarea on:input=update_body value=body />
                </div>
            }
        } else if menu.get().eq("Headers") {
            view! {
                <div>
                    <Header http_headers=http_headers set_http_headers=set_http_headers/>
                </div>
            }
        } 
        else {
            view! {
                <div>Build in progress</div>
            }
        }
    };

    view! {
        <div class=style::quick_container>
            <div class=style::top_input>
                <select value=method on:input=update_method id="methods" name="methods">
                    <option value="POST">Post</option>
                    <option value="PUT">Put</option>
                    <option value="GET">Get</option>
                    <option value="PATCH">Patch</option>
                    <option value="DELETE">Delete</option>
                </select>
                <input on:input=update_url value=url placeholder="Enter URL"></input>
                <button on:click=handle_submit>{message}</button>
            </div>
            <div class=style::field_nav>
                <div on:click = move |_|{ change_menu(String::from("Params")); }>Params</div>
                <div on:click = move |_|{ change_menu(String::from("Headers")); }>Headers</div>
                <div on:click = move |_|{ change_menu(String::from("Body")); }>Body</div>
            </div>
            {dynamic_component}
            <div>
                <Response response= response/>
            </div>
        </div>
    }
}