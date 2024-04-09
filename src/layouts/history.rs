use std::collections::HashMap;

use leptos::{ component, create_node_ref, create_resource, create_rw_signal, html::Div, use_context, view, CollectView, IntoView, ReadSignal, SignalGet, Transition};
use leptos_router::Outlet;
use leptos_use::{use_infinite_scroll_with_options, UseInfiniteScrollOptions};
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use stylance::import_crate_style;
use wasm_bindgen::prelude::*;
use serde_wasm_bindgen::to_value;
use crate::models::history::History;
import_crate_style!(style, "src/layouts/history.module.scss");

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct RequestArgs {
    page: i64,
    size: i64
}


#[component]
pub fn History() ->impl IntoView {
    let cdr = use_context::<ReadSignal<bool>>()
        .expect("there to be a `count` signal provided");

    let history_grouped = create_rw_signal(HashMap::<String, Vec<History>>::new());
    let history_ungrouped = create_rw_signal(Vec::<History>::new());

    pub async fn get_histories(page: i64) -> Vec<History> {
        let args = to_value(&RequestArgs { page: page, size: 50 }).unwrap();
        let new_msg = invoke("get_history", args).await.as_string().expect("Something went wrong");
        let res_struct: Vec<History> = from_str(&new_msg).unwrap();
        return res_struct;
    }

    let formated_url=move|s: String| {
        match s.get(0..25) {
            Some(first_20_chars) => {
                let ret_str= first_20_chars.to_string() + "...";
                return ret_str;
            }
            None => {
                return s;
            }
        }
    };

    // let _ = use_infinite_scroll_with_options(
    //     history_grouped,
    //     move |_| async move {
            
    //         // let len = data.with(|d| d.len());
    //         // set_data.update(|data| *data = (1..len+6).collect());
    //     },
    //     UseInfiniteScrollOptions::default().distance(50.0),
    // );

    let el = create_node_ref::<Div>();
    let load_value = move|data: HashMap<String, Vec<History>>| {
        use leptos::SignalSet;
        history_grouped.set(data);
    };

    let histories= create_resource(move|| cdr.get(), move |_| async move { 
        let resource = get_histories(1).await;
        let grouped = History::group_histories(resource.clone(), Option::None);
        load_value(grouped);
        return resource;
    });
    
    view! {
        <div class=style::main_container>
            <div class=style::side_nav>
                <Transition
                fallback=move || {
                    view! { <h2>"Loading..."</h2> }
                }>
                {move || {
                            history_grouped.get().into_iter()
                                .map(|(key, ele)|{
                                    view! {
                                        <div class=style::history_item_head>
                                            {key}
                                        </div>
                                        { 
                                            ele.into_iter()
                                                .map(|e| {
                                                    view!{ 
                                                        <div class=style::history_item>
                                                            <div>{e.headers.clone()}</div>
                                                            <div>{formated_url(e.url.clone())}</div>
                                                        </div>
                                                    }
                                                })
                                                .collect_view()
                                        }}
                                        
                                    
                                })
                                .collect_view()
                    }}
                </Transition>
            </div>
            <div class=style::child_div>
                <Outlet/>
            </div>
        </div>
    }
}