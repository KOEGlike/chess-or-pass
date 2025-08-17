use leptos::logging::*;
use leptos::prelude::*;

#[component]
pub fn ButtonPrimary(children:Children) -> impl IntoView {
    view! {
        <button class="rounded-2xl bg-primary flex items-center justify-center text-white">
            {children()}
        </button>
    }
}