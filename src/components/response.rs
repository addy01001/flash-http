use std::collections::HashMap;

use leptos::*;
use serde_json::Value;
use stylance::import_crate_style;

use crate::pages::quick::HttpResponse;
import_crate_style!(style, "src/components/response.module.scss");

fn json_string_to_hashmap(json_str: &str) ->HashMap<String, String> {
    let json_value: Value = serde_json::from_str(json_str).unwrap();
    
    let mut hashmap: HashMap<String, String> = HashMap::new();

    if let Value::Object(obj) = json_value {
        for (key, value) in obj {
            hashmap.insert(key, value.to_string());
        }
    }

    hashmap
}

#[component]
pub fn Response(
    response: RwSignal<HttpResponse>
) -> impl IntoView {

    let (menu, set_menu) = create_signal(String::from("Body"));
    let get_result_body = move || {
        let formated_str = response.get().body
            .replace("\\n", "\r\n")
            .replace("\\\"", "\"");
        let trimmed = if formated_str.starts_with('"') && formated_str.ends_with('"') {
            formated_str.trim_matches('"').to_string()
        } else {
            formated_str
        };
        return trimmed;
    };

    let get_result_header = move || {
        let header_map = json_string_to_hashmap(&response.get().headers);
        header_map
    };

    let get_code = move || {
        if response.get().code == 0{
            return String::new(); 
        } else {
            return response.get().code.to_string();
        };
    };

    let get_timing = move || {
        if response.get().code == 0{
            return String::new(); 
        } else {
            return response.get().timing.to_string();
        };
    };


    let change_menu = move |val: String| {
        set_menu.set(val);
    };

    // let to_render = move|| {
    //     return response.get().code!=0;  
    // };

    let dynamic_component = move|| {
        if menu.get().eq("Headers") {
            view! {
                <div>
                    // <div>{move || get_result_header()}</div>
                    <table class=style::headers>
                        <tr>
                            <th>Key</th>
                            <th>Value</th>
                        </tr>
                        {move || get_result_header()
                            .into_iter()
                            .map(|(k,v)|{
                                view! {
                                    <tr>
                                        <td>{k}</td>
                                        <td>{v}</td>
                                    </tr>
                                }
                            })
                            .collect_view()}
                        // {move|| dynamic_component()}
                    </table>
                </div>
            }
        } else {
            view! {
                <div class=style::body_container><pre>{move || get_result_body()}</pre></div>
            }
        }
    };

    view! {
        <div class=style::response_container>
            <h5>Response</h5>
            <div>Code: {move || get_code()}</div>
            <div>Time: {move || get_timing()}</div>
            
            <div class=style::response_nav>
                <div on:click = move|_|{ change_menu(String::from("Headers")); }>Header</div>
                <div on:click = move|_|{ change_menu(String::from("Body")); }>Body</div>
            </div>
            <div class=style::response_section>
                {move|| dynamic_component()}
            </div>
        </div>
    }
}