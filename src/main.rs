mod app;
mod pages;
mod layouts;
mod utils;
mod components;

use app::*;
use leptos::*;

fn main() {
    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}
