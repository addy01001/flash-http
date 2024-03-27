use leptos::*;
use leptos_router::*;

use crate::quick::QuickRequest;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes>
                <Route path="/" view=QuickRequest></Route>
            </Routes>
        </Router>
    }
}
