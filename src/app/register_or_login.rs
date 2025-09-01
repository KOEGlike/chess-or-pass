use leptos::logging::*;
use leptos::prelude::*;
use leptos::tachys::view;
use leptos_router::hooks::use_navigate;
use leptos_router::NavigateOptions;

#[component]
pub fn RegisterOrLoginPage() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-8 justify-center items-center w-full h-full">
            <button
                on:click=move |_| { use_navigate()("/register", NavigateOptions::default()) }
                class="p-2 mx-2 text-2xl w-30 button-primary"
            >
                "Sign Up"
            </button>
            <button
                on:click=move |_| { use_navigate()("/login", NavigateOptions::default()) }
                class="p-2 mx-2 text-2xl w-30 button-secondary"
            >
                "Log In"
            </button>
        </div>
    }
}
