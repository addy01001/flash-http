use leptos::*;
use stylance::import_crate_style;
use crate::{components::{form_data::FormData, header::Header}, models::http_models::{HttpFormData, HttpHashMapData}};
import_crate_style!(style, "src/components/body.module.scss");


#[component]
pub fn BodyComponent(
    http_form_data: RwSignal<Vec<HttpFormData>>,
    body: RwSignal<String>,
    menu: RwSignal<String>,
    binary: RwSignal<String>,
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

    let handle_upload = move|ev| {
        let v: String = event_target_value(&ev);
        binary.set(v);
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
                {move || match menu.get().as_str() {
                    "raw"=>{
                        view! {
                            <div>
                                <textarea class=style::textarea on:input=update_body prop:value=body />
                            </div>
                        }
                    }
                    "none"=>{
                        view! {
                            <div>
                                This request has no body
                            </div>
                        }
                    }
                    "form-url-encoded"=>{
                        view! {
                            <div>
                                <Header http_headers=http_form_encoded/>
                            </div>
                        }
                    }
                    "binary"=>{
                        view! {
                            <div>
                                <input on:input=handle_upload type="file" id="myFile" name="filename"/>
                                Build in progress
                            </div>
                        }
                    }
                    "form-data"=>{
                        view! {
                            <div>
                                Build in progress
                                <FormData http_params= http_form_data/>
                                </div>
                            }
                    }
                    _=>{
                        view! {
                            <div>Build in progress</div>
                        }
                    }
                }}
            </div>
        </div>
    }
}