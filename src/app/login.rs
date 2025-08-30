use crate::app::chess::ChessBoard;
use gloo::storage::Storage;
use leptos::either::EitherOf3;
use leptos::either::EitherOf4;
use leptos::logging::*;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use leptos_router::NavigateOptions;
use sha2::Digest;
use sha2::Sha256;
use shakmaty::fen::*;
use shakmaty::san::*;
use shakmaty::EnPassantMode;
use shakmaty::KnownOutcome;
use shakmaty::Position;

use crate::app::game_modal::*;

use crate::types::Error;

#[derive(Clone, Debug, PartialEq)]
enum State {
    Username,
    Password {
        user_name: String,
    },
    Done {
        user_name: String,
        password: Vec<(San, Fen)>,
    },
}

#[component]
pub fn LoginPage() -> impl IntoView {
    let (state, set_state) = signal(State::Username);

    let current_view = move || match state.get() {
        State::Username => {
            let (user_name, set_username) = signal(String::new());

            let on_click = move |_| {
                set_state.set(State::Password {
                    user_name: user_name.get(),
                })
            };

            EitherOf3::A(view! {
                <div class="flex flex-col gap-2.5 justify-center items-center">
                    <span class="text-5xl">"Register"</span>
                    <input
                        placeholder="Name"
                        class="w-full"
                        type="text"
                        bind:value=(user_name, set_username)
                    />
                    <button on:click=on_click class="p-4 w-full text-2xl button-primary">
                        "Continue"
                    </button>
                </div>
            })
        }
        State::Password { user_name } => {
            let notation: RwSignal<Vec<(San, Fen)>> = RwSignal::new(vec![]);

            let (ended, set_ended) = signal(Option::<KnownOutcome>::None);

            let on_finished = move |o: KnownOutcome| {
                log!("ended {o:?}");
                set_ended.set(Some(o));
            };

            let on_continue = {
                let user_name = user_name.clone();
                move |_| {
                    set_state.set(State::Done {
                        user_name: user_name.clone(),
                        password: notation.get(),
                    });
                }
            };

            EitherOf3::B(view! {
                <div class="flex flex-col gap-2.5 justify-start items-center w-full h-full">
                    <div class="flex flex-col justify-start items-start w-full text-2xl h-fit">
                        "Hi " {user_name}
                        <span class="font-sans font-light">
                            "Let's make a password! Play a game of chess with yourself until the game is over! Remember the game well!"
                        </span>
                    </div>
                    <div class="flex flex-col justify-center items-center w-full h-full">
                        <ChessBoard on_finished notation />
                        <GameEndModal ended on_continue />
                    </div>
                </div>
            })
        }
        State::Done {
            user_name,
            password,
        } => {
            let result = {
                let user_name = user_name.clone();
                LocalResource::new(move || login(user_name.clone(), password.clone()))
            };

            let suspense = {
                let user_name = user_name.clone();
                move || {
                    let user_name = user_name.clone();
                    Suspend::new(async move {
                        let result = result.await;

                        let (main_text, sub_text, button_text) = match &result {
                            Ok(_) => (
                                "Logged in".to_string(),
                                "Yipeee 😁😁😁😁".to_string(),
                                "Home!".to_string(),
                            ),
                            Err(Error::WrongPassword) => (
                                "Wrong password".to_string(),
                                "You messed up".to_string(),
                                "Try again".to_string(),
                            ),
                            Err(e) => (
                                "Something went wrong".to_string(),
                                e.to_string(),
                                "Home".to_string(),
                            ),
                        };

                        if let Ok(user_id) = &result {
                            if let Err(e) = gloo::storage::LocalStorage::set("id", user_id) {
                                error!("Could not store user id in local storage: {e}");
                            }
                        }

                        let on_click = move |_| match result {
                            Err(Error::WrongPassword) => set_state.set(State::Password {
                                user_name: user_name.clone(),
                            }),
                            _ => use_navigate()("/", NavigateOptions::default()),
                        };

                        view! {
                            <GameModal
                                visible=true
                                main_text
                                sub_text
                                button_text
                                on_click
                            />
                        }
                    })
                }
            };

            EitherOf3::C(view! {
                <Suspense>
                    {suspense}
                </Suspense>
            })
        }
    };

    view! {
        <div class="flex flex-col justify-center items-center p-5 w-full h-full">
            {current_view}
        </div>
    }
}

#[server]
async fn login(name: String, password: Vec<(San, Fen)>) -> Result<String, Error> {
    use crate::types::AppState;
    let app_state = expect_context::<AppState>();

    let mut transaction = app_state.db.pool.begin().await?;
    transaction.commit().await?;
    Ok("    user_id".to_string())
}
