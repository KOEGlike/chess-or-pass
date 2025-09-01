use crate::app::chess::ChessBoard;
use gloo::storage::Storage;
use leptos::either::{Either, EitherOf4};
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

const MIN_PASSWORD_LENGTH: usize = 4;

#[derive(Clone, Debug, PartialEq)]
enum State {
    Username,
    Password {
        user_name: String,
    },
    PasswordConfirm {
        user_name: String,
        first_attempt: Vec<(San, Fen)>,
    },
    Done {
        user_name: String,
        password: Vec<(San, Fen)>,
    },
}

#[component]
pub fn RegisterPage() -> impl IntoView {
    let (state, set_state) = signal(State::Username);

    let current_view = move || match state.get() {
        State::Username => {
            let (user_name, set_username) = signal(String::new());

            let taken = Resource::new(
                move || user_name.get(),
                |name| async move {
                    if name.is_empty() {
                        Ok(false)
                    } else {
                        is_username_taken(name).await
                    }
                },
            );

            let suspense = move || {
                Suspend::new(async move {
                    let taken = taken.await.unwrap_or(false);

                    let on_click = move |_| {
                        if taken {
                            return;
                        }

                        set_state.set(State::Password {
                            user_name: user_name.get(),
                        })
                    };

                    view! {
                        {if taken {
                            Either::Left(
                                view! { <span class="text-red-500">"Username is taken"</span> },
                            )
                        } else {
                            Either::Right(())
                        }}
                        <button
                            on:click=on_click
                            class="p-4 w-full text-2xl"
                            class:button-primary=!taken
                            class:button-secondary=taken
                            class:bg-secondary-hover=taken
                        >
                            "Continue"
                        </button>
                    }
                })
            };

            EitherOf4::A(view! {
                <div class="flex flex-col gap-2.5 justify-center items-center">
                    <span class="text-5xl">"Register"</span>
                    <input
                        placeholder="Name"
                        class="w-full"
                        type="text"
                        bind:value=(user_name, set_username)
                    />
                    <Transition fallback=move || {
                        view! { <div>"Checking..."</div> }
                    }>{suspense}</Transition>
                </div>
            })
        }
        State::Password { user_name } => {
            let notation: RwSignal<Vec<(San, Fen)>> = RwSignal::new(vec![]);

            let over_min_moves = move || notation.read().len() >= MIN_PASSWORD_LENGTH;

            Effect::new(move |_| {
                let s = notation
                    .get()
                    .into_iter()
                    .map(|e| format!("san {} fen {}", e.0, e.1))
                    .collect::<Vec<String>>()
                    .join(" \n");
                log!("{s}");
            });

            let (ended, set_ended) = signal(Option::<KnownOutcome>::None);

            let on_finished = move |o: KnownOutcome| {
                log!("ended {o:?}");
                set_ended.set(Some(o));
            };

            let on_continue = {
                let user_name = user_name.clone();
                move |_| {
                    if !over_min_moves() {
                        return;
                    }

                    set_state.set(State::PasswordConfirm {
                        user_name: user_name.clone(),
                        first_attempt: notation.get(),
                    });
                }
            };

            EitherOf4::B(view! {
                <div class="flex flex-col gap-2.5 justify-start items-center w-full h-full">
                    <div class="flex flex-col justify-start items-start w-full text-2xl h-fit">
                        "Hi " {user_name}
                        <span class="font-sans font-light">
                            "Let's make a password! Play a game of chess with yourself until the game is over! Remember the game well!"
                        </span>
                    </div>
                    <div class="flex flex-row justify-around items-center w-full h-full">
                        <div class="flex flex-col justify-center items-center">
                            <ChessBoard on_finished notation />
                            <GameEndModal ended on_continue=on_continue.clone() />
                        </div>

                        <button
                            on:click=on_continue
                            class="p-10 text-2xl"
                            class:button-primary=over_min_moves
                            class:button-secondary=move || !over_min_moves()
                            class:bg-secondary-hover=move || !over_min_moves()
                        >
                            {move || {
                                if over_min_moves() {
                                    Either::Left("Continue")
                                } else {
                                    Either::Right(
                                        format!("Play at least {MIN_PASSWORD_LENGTH} moves"),
                                    )
                                }
                            }}
                        </button>
                    </div>
                </div>
            })
        }
        State::PasswordConfirm {
            user_name,
            first_attempt,
        } => {
            let notation: RwSignal<Vec<(San, Fen)>> = RwSignal::new(vec![]);

            Effect::new(move |_| {
                let s = notation
                    .get()
                    .into_iter()
                    .map(|e| format!("san {} fen {}", e.0, e.1))
                    .collect::<Vec<String>>()
                    .join(" \n");
                log!("{s}");
            });

            let matches = {
                let first = first_attempt.clone();
                move || {
                    let notation = notation.get();
                    let last_element = match notation.last() {
                        Some(e) => e,
                        None => return true,
                    };

                    let first_element = match first.get(notation.len() - 1) {
                        Some(e) => e,
                        None => return false,
                    };

                    last_element == first_element
                }
            };

            let completed = {
                let first_attempt = first_attempt.clone();
                let matches = matches.clone();
                move || notation.read().len() == first_attempt.len() && matches()
            };

            let (ended, set_ended) = signal(Option::<KnownOutcome>::None);

            let on_finished = move |o: KnownOutcome| {
                log!("ended {o:?}");
                set_ended.set(Some(o));
            };

            let on_continue = {
                let user_name = user_name.clone();
                let first_attempt = first_attempt.clone();

                move |_| {
                    if first_attempt == notation.get() {
                        set_state.set(State::Done {
                            user_name: user_name.clone(),
                            password: notation.get(),
                        });
                    } else {
                        error!("Passwords do not match");
                    }
                }
            };

            let on_restart = move |_| {
                set_state.set(state.get());
            };

            EitherOf4::C(view! {
                <div class="flex flex-col gap-2.5 justify-start items-center w-full h-full">
                    <div class="flex flex-col justify-start items-start w-full text-2xl h-fit">
                        "Hi " {user_name}
                        <span class="font-sans font-light">
                            "Now play the same game of chess again!"
                        </span>
                    </div>
                    <div class="flex flex-row justify-around items-center w-full h-full">
                        <div class="flex flex-col justify-center items-center w-full h-full">
                            <ChessBoard on_finished notation />
                            <GameEndModal ended on_continue=on_continue.clone() />
                            <GameModal
                                visible=Signal::derive(move || !matches())
                                main_text="Move doesn't match"
                                sub_text="Please try again"
                                button_text="Retry"
                                on_click=on_restart
                            />
                            <GameModal
                                visible=Signal::derive(completed)
                                main_text="Password Matches!!"
                                button_text="Continue!"
                                on_click=on_continue
                            />
                        </div>
                    </div>
                </div>
            })
        }
        State::Done {
            user_name,
            password,
        } => {
            let result =
                LocalResource::new(move || create_user(user_name.clone(), password.clone()));

            let on_home = move |_| {
                use_navigate()("/", NavigateOptions::default());
            };

            EitherOf4::D(view! {
                <Suspense>
                    {move || Suspend::new(async move {
                        let result = result.await;
                        let (main_text, sub_text) = match &result {
                            Ok(_) => ("User created".to_string(), "Yipeee 😁😁".to_string()),
                            Err(e) => ("Something went wrong".to_string(), e.to_string()),
                        };
                        if let Ok(user_id) = &result {
                            if let Err(e) = gloo::storage::LocalStorage::set("id", user_id) {
                                error!("Could not store user id in local storage: {e}");
                            }
                        }

                        view! {
                            <GameModal
                                visible=true
                                main_text=main_text
                                sub_text=sub_text
                                button_text="Home!"
                                on_click=on_home
                            />
                        }
                    })}
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
async fn is_username_taken(name: String) -> Result<bool, Error> {
    use crate::types::AppState;
    let app_state = expect_context::<AppState>();

    let rec = sqlx::query!(
        r#"
        SELECT id FROM users WHERE username = $1
        "#,
        name
    )
    .fetch_optional(&app_state.db.pool)
    .await?;

    Ok(rec.is_some())
}

#[server]
async fn create_user(name: String, password: Vec<(San, Fen)>) -> Result<String, Error> {
    use crate::types::AppState;
    let app_state = expect_context::<AppState>();

    if !check_chess_moves(&password) {
        return Err(Error::ImpossibleChessGame);
    }

    let mut transaction = app_state.db.pool.begin().await?;

    let user_id = cuid2::cuid();
    let salt = cuid2::CuidConstructor::new().with_length(5).create_id();

    let fen_hashed_vec =
        hash_fen_with_salt(password.into_iter().map(|(_san, fen)| fen).collect(), &salt);

    sqlx::query!(
        r#"
        INSERT INTO users (id, username, password, salt) VALUES ($1, $2, $3, $4)
        "#,
        user_id,
        name,
        &fen_hashed_vec,
        salt
    )
    .execute(&mut *transaction)
    .await?;

    transaction.commit().await?;

    Ok(user_id)
}

fn check_chess_moves(moves: &[(San, Fen)]) -> bool {
    let mut pos = shakmaty::Chess::default();
    for (san, fen) in moves {
        let mv = match san.to_move(&pos) {
            Ok(mv) => mv,
            Err(_) => return false,
        };
        pos = match pos.play(mv) {
            Ok(p) => p,
            Err(_) => return false,
        };
        let current_fen = Fen::from_position(&pos, EnPassantMode::Legal);
        if &current_fen != fen {
            return false;
        }
    }
    true
}

pub fn hash_fen_with_salt(fen: Vec<Fen>, salt: &str) -> Vec<Vec<u8>> {
    fen.into_iter()
        .map(|f| f.to_string() + salt)
        .map(|s| Sha256::digest(s.as_bytes()))
        .map(|d| d.to_vec())
        .collect::<Vec<_>>()
}
