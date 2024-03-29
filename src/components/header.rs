use stylance::import_crate_style;
use leptos::*;
use crate::quick::HttpHeaders;
import_crate_style!(style, "src/components/header.module.scss");


#[component]
pub fn Header(
    http_headers: ReadSignal<Vec<HttpHeaders>>,  set_http_headers: WriteSignal<Vec<HttpHeaders>>
)->impl IntoView {
    let add_column = move|| {
        let mut new_value = http_headers.get().clone();
        new_value.push(HttpHeaders::new());
        set_http_headers.set(new_value);
    };

    let handle_update = move|ev: ev::Event, index: usize|{
        if index == http_headers.get_untracked().len() {
            add_column();
        }
    };

    let dynamic_component = move|| {
        let mut index = 0;
        http_headers.get().into_iter()
            .map(|e|{
                index+=1;
                return view!{
                    <tr>
                        <td><input on:input = move|ev|{ e.key.set(event_target_value(&ev)) } prop:value=e.key /></td>
                        <td><input on:input = move|ev|{ e.value.set(event_target_value(&ev)) } prop:value=e.value /></td>
                        <td><input></input></td>
                    </tr>
                };
            })
            .collect_view()
    };

    view! {
        <table class=style::headers>
            <tr>
                <th>Key</th>
                <th>Value</th>
                <th>Description</th>
            </tr>
            
            {move|| dynamic_component()}
        </table>
        <button on:click=move|_|{ add_column() }>Add</button>
    }
}