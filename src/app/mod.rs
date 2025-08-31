use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes},
    hooks::use_navigate,
    path, NavigateOptions,
};

mod chess;
mod feed;
mod game_modal;
mod login;
mod register;
mod register_or_login;
mod sidebar;
mod vote;

use chess::ChessBoard;
use feed::FeedPage;
use login::LoginPage;
use register::RegisterPage;
use register_or_login::RegisterOrLoginPage;
use sidebar::Sidebar;
use vote::VotePage;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en" class="text-white">
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
            <main class="overflow-hidden w-screen h-screen font-chess-sans bg-background">
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=path!("/register") view=RegisterPage />
                    <Route path=path!("/login") view=LoginPage />
                    <ParentRoute path=path!("") view=Sidebar>
                        <Route path=path!("feed") view=FeedPage />
                        <Route path=path!("play") view=VotePage />
                        <Route path=path!("login-register") view=RegisterOrLoginPage />
                        <Route path=path!("") view=RedirectToFeed />
                    </ParentRoute>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn RedirectToFeed() -> impl IntoView {
    Effect::new(move || {
        let navigate = use_navigate();
        navigate("/feed", NavigateOptions::default());
    });
    view! {}
}
