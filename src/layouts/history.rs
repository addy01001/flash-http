use leptos::{ component, create_resource, view, CollectView, IntoView, Transition};
use leptos_router::Outlet;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use stylance::import_crate_style;
use wasm_bindgen::prelude::*;
use serde_wasm_bindgen::to_value;
import_crate_style!(style, "src/layouts/history.module.scss");

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct RequestArgs {}

#[derive(Serialize, Deserialize)]
pub struct History {
    pub id: i32,
    pub method: String,
    pub url: String,
    pub body: String,
    pub headers: String,
    pub created_at: String
}

#[component]
pub fn History() ->impl IntoView {
    pub async fn get_histories() -> Vec<History> {
        let args = to_value(&RequestArgs { }).unwrap();
        let new_msg = invoke("get_history", args).await.as_string().expect("Something went wrong");
        let res_struct: Vec<History> = from_str(&new_msg).unwrap();
        return res_struct;
    }

    let formated_url=move|s: String| {
        match s.get(0..20) {
            Some(first_20_chars) => {
                let ret_str= first_20_chars.to_string() + "...";
                return ret_str;
            }
            None => {
                return s;
            }
        }
    };

    let histories= create_resource(|| (), |_| async move { get_histories().await });
    
    view! {
        <div class=style::main_container>
            <div class=style::side_nav>
                <Transition
                fallback=move || {
                    view! { <h2>"Loading..."</h2> }
                }>
                {move || {
                        histories.map(|e|{
                            e.into_iter()
                                .map(|ele|{
                                    view! {
                                        <div class=style::history_item>
                                            <div>{ele.headers.clone()}</div>
                                            <div>{formated_url(ele.url.clone())}</div>
                                        </div>
                                    }
                                })
                                .collect_view()
                        })
                    }}
                </Transition>
            </div>
            <div class=style::child_div>
                <Outlet/>
            </div>
        </div>
    }
}