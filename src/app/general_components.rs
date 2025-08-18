use leptos::logging::*;
use leptos::prelude::*;

#[component]
pub fn ButtonPrimary(children:Children) -> impl IntoView {
    view! {
        <ButtonBare {..} class="bg-primary hover:bg-primary-hover">
            {children()}
        </ButtonBare>
    }
}

#[component]
pub fn ButtonSecondary(children:Children) -> impl IntoView {
    view! {
        <ButtonBare {..} class="bg-secondary hover:bg-secondary-hover">
            {children()}
        </ButtonBare>
    }
}

#[component]
pub fn ButtonBare(children:Children) -> impl IntoView {
    view! {
        <button class="rounded-sm flex items-center justify-center text-white shadow-[inset_0px_-0.35rem_0px_0px] shadow-overlay-shadow">
            {children()}
        </button>
    }
}