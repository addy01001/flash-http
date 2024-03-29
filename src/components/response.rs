use leptos::*;
use stylance::import_crate_style;
use crate::quick::HttpResponse;
import_crate_style!(style, "src/components/response.module.scss");

#[component]
pub fn Response(
    response: RwSignal<HttpResponse>
) -> impl IntoView {

    let (menu, set_menu) = create_signal(String::from("Body"));
    let get_result_body = move || {
        return response.get().body;
    };

    let get_result_header = move || {
        return response.get().headers;
    };

    let get_code = move || {
        if response.get().code == 0{
            return String::new(); 
        } else {
            return response.get().code.to_string();
        };
    };

    let change_menu = move |val: String| {
        set_menu.set(val);
    };

    let to_render = move|| {
        return response.get().code!=0;  
    };

    let dynamic_component = move|| {
        if menu.get().eq("Headers") {
            view! {
                <div>{move || get_result_header()}</div>
            }
        } else {
            view! {
                <div>{move || get_result_body()}</div>
            }
        }
    };

    view! {
        <div class=style::response_container>
            <h5>Response {move || get_code()}</h5>
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