use leptos::{component, view, IntoView};
use leptos_router::Outlet;
use stylance::import_crate_style;
import_crate_style!(style, "src/layouts/default.module.scss");

#[component]
pub fn DefaultLayout() -> impl IntoView {
    view! {
        <div class=style::main_container>
            <div class=style::side_nav>
                // <div>Quick</div>
                <div>History</div>
                // <div>Collections</div>
            </div>
            <div class=style::child_div>
                <Outlet/>    
            </div>
        </div>
    }
}