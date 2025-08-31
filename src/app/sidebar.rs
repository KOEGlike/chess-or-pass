use leptos::logging::*;
use leptos::prelude::*;
use leptos_router::components::{Outlet, A};

#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <div class="flex flex-row w-full h-full">
            <div class="flex flex-col justify-start w-32 h-full bg-secondary">
                <A href="/play">
                    <Section text="Play" image_src="/play-icon.png" />
                </A>
                <A href="/feed">
                    <Section text="Feed" image_src="/feed-icon.png" />
                </A>
            </div>
            <Outlet />
        </div>
    }
}

#[component]
fn Section(#[prop(into)] text: String, #[prop(into)] image_src: String) -> impl IntoView {
    view! {
        <div class="flex flex-row gap-1 items-center p-4 hover:bg-secondary-hover">
            <img src=image_src class="w-8 h-8" />
            {text}
        </div>
    }
}
