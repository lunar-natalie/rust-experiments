use leptos::*;

mod components;
use components::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <List initial_length=8/>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> });
}