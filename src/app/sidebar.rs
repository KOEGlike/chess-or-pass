use leptos::either::Either;
use leptos::ev::error;
use leptos::logging::*;
use leptos::prelude::*;
use leptos_router::components::{Outlet, A};
use leptos_router::hooks::use_navigate;
use leptos_router::NavigateOptions;

#[component]
pub fn Sidebar() -> impl IntoView {
    let (user_id, set_user_id) = signal(None::<String>);

    Effect::new(move || {
        use gloo::storage::{LocalStorage, Storage};

        let res = LocalStorage::get("id");

        if let Err(e) = &res {
            log!("No user_id found in local storage: {e:?}");
        }

        set_user_id.set(res.ok());
    });

    let bottom = move || match user_id.get() {
        Some(user_id) => {
            let name = OnceResource::new(get_user_name(user_id));
            let on_logout = move |_| {
                use gloo::storage::{LocalStorage, Storage};

                LocalStorage::delete("id");

                if let Some(w) = leptos::web_sys::window() {
                    if let Err(e) = w.location().reload() {
                        error!("Error reloading page after logout: {e:?}");
                    }
                }
            };
            let suspense = move || {
                Suspend::new(async move {
                    match name.await {
                        Ok(name) => Either::Left(view! {
                            <div class="flex flex-col items-start p-4">
                                <span class="font-sans text-lg font-light">"Logged in as"</span>
                                <span class="text-2xl font-bold">{name}</span>
                            </div>
                        }),
                        Err(e) => Either::Right(
                            view! { <div class="text-red-700">"Error loading username: " {e.to_string()}</div> },
                        ),
                    }
                })
            };
            Either::Left(view! {
                {suspense}
                <Section on:click=on_logout text="Log Out" image_src="/logout-icon.png" />
            })
        }
        None => Either::Right(view! {
            <button
                on:click=move |_| { use_navigate()("/register", NavigateOptions::default()) }
                class="p-2 mx-2 w-auto text-lg button-primary"
            >
                "Sign Up"
            </button>
            <button
                on:click=move |_| { use_navigate()("/login", NavigateOptions::default()) }
                class="p-2 mx-2 w-auto text-lg button-secondary bg-background hover:bg-secondary-hover"
            >
                "Log In"
            </button>
        }),
    };

    view! {
        <div class="flex flex-row w-full h-full">
            <div class="flex flex-col justify-between w-auto h-full bg-secondary">
                <div class="flex flex-col w-full h-fit">
                    <a href="/" class="p-2 text-3xl">
                        "Chess Or Pass"
                    </a>

                    <A href="/play">
                        <Section text="Play" image_src="/play-icon.png" />
                    </A>
                    <A href="/feed">
                        <Section text="Feed" image_src="/feed-icon.png" />
                    </A>
                </div>

                <div class="flex flex-col gap-4 pb-8 mx-0.5 w-full h-fit">{bottom}</div>
            </div>

            <Outlet />
        </div>
    }
}

#[component]
fn Section(#[prop(into)] text: String, #[prop(into)] image_src: String) -> impl IntoView {
    view! {
        <div class="flex flex-row gap-3 items-center p-4 hover:bg-secondary-hover">
            <img src=image_src class="w-8 h-8" />
            {text}
        </div>
    }
}

#[server]
async fn get_user_name(user_id: String) -> Result<String, ServerFnError> {
    use crate::types::AppState;
    let app_state = expect_context::<AppState>();

    let mut transaction = app_state.db.pool.begin().await?;

    let row = sqlx::query!("SELECT username FROM users WHERE id = $1", user_id)
        .fetch_one(&mut *transaction)
        .await?;

    transaction.commit().await?;

    Ok(row.username)
}
