use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
};

mod chess;
mod general_components;
mod login_register;

use chess::ChessBoard;
use login_register::LoginRegisterPage;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/chess-or-pass.css" />

        <Title text="Welcome to Leptos" />

        <Router>
            <main class="w-screen h-screen font-sans">
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=LoginRegisterPage />
                </Routes>
            </main>
        </Router>
    }
}
