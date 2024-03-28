use std::collections::HashMap;

use stylance::import_crate_style;
use leptos::*;
use crate::quick::HttpHeaders;
import_crate_style!(style, "src/components/header.module.scss");


#[component]
pub fn Header(
    http_headers: ReadSignal<Vec<HttpHeaders>>, set_http_headers: WriteSignal<Vec<HttpHeaders>>
)->impl IntoView {
    let add_column = move|| {
        let mut new_value = http_headers.get_untracked().clone();
        new_value.push(HttpHeaders::new());
        set_http_headers.set(new_value);
    };

    let handle_vector_update = move|ev: ev::Event, index: usize|{
        if index == http_headers.get().len() {
            add_column();
        }

        let mut new_value = http_headers.get_untracked().clone();
        let v = event_target_value(&ev);
        new_value[index].key = v;
    };

    let handle_vector_update_value = move|ev: ev::Event, index: usize|{
        if index == http_headers.get().len() {
            add_column();
        }

        let mut new_value = http_headers.get_untracked().clone();
        let v = event_target_value(&ev);
        new_value[index].value = v;
    };

    let dynamic_component = move|| {
        let mut index = -1;
        http_headers.get().clone().into_iter()
            .map(|e|{
                index+=1;
                return view!{
                    <tr>
                        <td><input value=e.key /></td>
                        <td><input value=e.value /></td>
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
    }
}