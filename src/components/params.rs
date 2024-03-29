use stylance::import_crate_style;
use leptos::*;
use crate::quick::HttpHeaders;
import_crate_style!(style, "src/components/params.module.scss");

#[component]
pub fn Params(
    http_params: ReadSignal<Vec<HttpHeaders>>, set_http_params: WriteSignal<Vec<HttpHeaders>>
)->impl IntoView {
    let add_column = move|| {
        let mut new_value = http_params.get_untracked().clone();
        new_value.push(HttpHeaders::new());
        set_http_params.set(new_value);
    };

    let handle_vector_update = move|ev: ev::Event, index: usize|{
        if index == http_params.get().len() {
            add_column();
        }

        let mut new_value = http_params.get_untracked().clone();
        let v = event_target_value(&ev);
        new_value[index-1].key = v;
        set_http_params.set(new_value);
    };

    let handle_vector_update_value = move|ev: ev::Event, index: usize|{
        if index == http_params.get().len() {
            add_column();
        }

        let mut new_value = http_params.get_untracked().clone();
        let v = event_target_value(&ev);
        new_value[index-1].value = v;
        set_http_params.set(new_value);
    };

    let dynamic_component = move|| {
        let mut index = 0;
        http_params.get().clone().into_iter()
            .map(|e|{
                index+=1;
                return view!{
                    <tr>
                        <td><input on:input = move|ev|{ handle_vector_update(ev, index) } prop:value=e.key /></td>
                        <td><input on:input = move|ev|{ handle_vector_update_value(ev, index) } prop:value=e.value /></td>
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