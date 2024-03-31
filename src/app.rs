use leptos::*;
use leptos_query::{ provide_query_client_with_options, DefaultQueryOptions};
use leptos_router::*;

use crate::{layouts::{default::DefaultLayout, history::History}, pages::quick::QuickRequest};

#[component]
pub fn App() -> impl IntoView {
    // provide_query_client_with_options(DefaultQueryOptions {
    //     resource_option: leptos_query::ResourceOption::Local,
    //     ..DefaultQueryOptions::default()
    // });
    let (dark_mode, set_dark_mode) = create_signal(false);
    provide_context(dark_mode);

    view! {
        <Router>
            <Routes>
                <Route path="/" view=DefaultLayout>
                    <Route path="/" view=History>
                        <Route path="/" view=QuickRequest/>
                    </Route>
                </Route>
            </Routes>
        </Router>
    }
}
