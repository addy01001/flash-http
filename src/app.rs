use leptos::*;
use leptos_router::*;

use crate::{layouts::{default::DefaultLayout, history::History}, pages::quick::QuickRequest};

#[component]
pub fn App() -> impl IntoView {
    // Global states
    let (dark_mode, set_dark_mode) = create_signal(false);
    let (cdr, set_cdr) = create_signal(false);

    provide_context(dark_mode);
    provide_context(cdr);

    view! {
        <Router>
            <Routes>
                <Route path="/" view=DefaultLayout>
                    <Route path="/" view=History>
                        <Route path="/" view=move|| view! { <QuickRequest set_cdr/>  }/>
                    </Route>
                </Route>
            </Routes>
        </Router>
    }
}
