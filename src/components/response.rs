use leptos::*;
use stylance::import_crate_style;

use crate::{models::http_models::HttpResponse, utils::http::{format_json_body, json_string_to_hashmap}};
import_crate_style!(style, "src/components/response.module.scss");

#[component]
pub fn Response(
    response: RwSignal<HttpResponse>
) -> impl IntoView {

    let (menu, set_menu) = create_signal(String::from("Body"));
    let get_result_body = move || {
        let formated_str = format_json_body(response.get().body);
        return formated_str;
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
            <div class=style::metrics>
                <div>Code: {move || get_code()}</div>
                <div>Time: {move || get_timing()}s</div>
            </div>
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