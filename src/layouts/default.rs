use leptos::{component, view, IntoView};
use leptos_router::Outlet;
use stylance::import_crate_style;
import_crate_style!(style, "src/layouts/default.module.scss");

#[component]
pub fn DefaultLayout() -> impl IntoView {
    let handle_close=move|| {
        
    };
    view! {
        <div class=style::app>
            <div class=style::app_bar>
              <div>Flash HTTP</div>
                <div data-tauri-drag-region class=style::titlebar>
                    Build in progress
                    <div class="titlebar-button" id="titlebar-minimize">
                        <img
                        src="https://api.iconify.design/mdi:window-minimize.svg"
                        alt="minimize"
                        />
                    </div>
                    <div class="titlebar-button" id="titlebar-maximize">
                        <img
                        src="https://api.iconify.design/mdi:window-maximize.svg"
                        alt="maximize"
                        />
                    </div>
                    <div class="titlebar-button" id="titlebar-close">
                        <img src="https://api.iconify.design/mdi:close.svg" alt="close" />
                    </div>
                </div>
            </div>
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
        </div>
    }
}