use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

mod chess;
mod register;

use chess::ChessBoard;
use register::RegisterPage;

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
            <main class="w-screen h-screen font-sans bg-background">
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=RegisterPage />
                </Routes>
            </main>
        </Router>
    }
}
