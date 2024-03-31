use leptos::*;
use leptos_router::*;

use crate::{layouts::default::DefaultLayout, pages::quick::QuickRequest};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes>
                <Route path="/" view=DefaultLayout>
                    <Route path="/" view=QuickRequest/>
                </Route>
            </Routes>
        </Router>
    }
}
