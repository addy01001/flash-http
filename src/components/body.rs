use leptos::*;
use stylance::import_crate_style;

use crate::{components::{form_data::FormData, header::Header}, models::http_models::{HttpFormData, HttpHashMapData}};
import_crate_style!(style, "src/components/body.module.scss");

#[component]
pub fn BodyComponent(
    http_form_data: RwSignal<Vec<HttpFormData>>,
    body: RwSignal<String>,
    menu: RwSignal<String>,
    http_form_encoded: RwSignal<Vec<HttpHashMapData>>
)->impl IntoView {

    let update_body = move |ev: ev::Event| {
        let v: String = event_target_value(&ev);
        body.set(v);
    };

    let handle_change = move|ev| {
        let v: String = event_target_value(&ev);
        menu.set(v);
    };

    view! {
        <div>
            <div>
            //Implement body types nav
                <form class=style::form>
                    <input type="radio" id="none" name="body_type" value="none" on:input=handle_change/>
                    <label for="none">none</label><br/>
                    <input type="radio" id="form_data" name="body_type" value="form-data" on:input=handle_change/>
                    <label for="form_data">form-data</label><br/>
                    <input type="radio" id="url_encoded" name="body_type" value="form-url-encoded" on:input=handle_change/>
                    <label for="url_encoded">form-url-encoded</label><br/>
                    <input type="radio" id="binary" name="body_type" value="binary" on:input=handle_change/>
                    <label for="binary">binary</label><br/>
                    <input type="radio" id="raw" name="body_type" value="raw" on:input=handle_change />
                    <label for="raw">raw</label>
                </form>
            </div>

            <div>
                // TODO: Change to switch case
                {move || if menu.get().eq("raw") {
                    return view! {
                        <div>
                            <textarea class=style::textarea on:input=update_body prop:value=body />
                        </div>
                    };
                 } else if menu.get().eq("none") {
                    return view! {
                        <div>
                            This request has no body
                        </div>
                    };
                 } else if menu.get().eq("form-url-encoded") {
                    return view! {
                        <div>
                            <Header http_headers=http_form_encoded/>
                        </div>
                    };
                 } else if menu.get().eq("binary") {
                    return view! {
                        <div>
                            <input type="file" id="myFile" name="filename"/>
                            Build in progress
                        </div>
                    };
                 } else if menu.get().eq("form-data") {
                    return view! {<div>
                        Build in progress
                        <FormData http_params= http_form_data/>
                        </div>}
                 } else {
                        return view! {
                            <div>Build in progress</div>
                        };
                    }
                }
            </div>
        </div>
    }
}