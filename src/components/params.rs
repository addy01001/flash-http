use std::collections::HashMap;

use stylance::import_crate_style;
use leptos::*;

use crate::{models::http_models::HttpHashMapData, utils::url::{add_query_params, get_base_url}};
import_crate_style!(style, "src/components/params.module.scss");



#[component]
pub fn Params(
    http_params: RwSignal<Vec<HttpHashMapData>>, url: RwSignal<String>
)->impl IntoView {
    let add_column = move|| {
        let mut new_value = http_params.get().clone();
        new_value.push(HttpHashMapData::new());
        http_params.set(new_value);
    };

    let handle_vector_update = move||{
        let mut query_params: HashMap<String, String> = HashMap::new();
        http_params.get_untracked().clone().iter_mut()
            .for_each(|e|{
                query_params.insert(e.key.get(), e.value.get());
            });

        let base_url = get_base_url(url.get());
        url.set(add_query_params(base_url.as_str(), &query_params))
    };

    let handle_delete = move|index: usize|{
        let mut new_value = http_params.get().clone();
        if new_value.len() == 1 {
            http_params.set(vec![HttpHashMapData::new()])
        } else {
            new_value.remove(index-1);
            http_params.set(new_value);
        }
        handle_vector_update();
    };


    let dynamic_component = move|| {
        let mut index = 0;
        http_params.get().clone().into_iter()
            .map(|e|{
                index+=1;
                return view!{
                    <tr>
                        <td><input on:input = move|ev|{ e.key.set(event_target_value(&ev)); handle_vector_update(); } prop:value=e.key /></td>
                        <td><input on:input = move|ev|{ e.value.set(event_target_value(&ev)); handle_vector_update(); } prop:value=e.value /></td>
                        <td><div class=style::desc_div><input></input><button on:click = move|_|{ handle_delete(index) }>Delete</button></div></td>
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