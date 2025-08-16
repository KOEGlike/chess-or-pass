use leptos::logging::*;
use leptos::prelude::*;

#[component]
pub fn ButtonPrimary(children:Children) -> impl IntoView {
    view! { <button class="rounded-sm bg-primary">{children}</button> }
}