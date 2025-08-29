use crate::app::chess::ChessBoard;
use leptos::either::EitherOf4;
use leptos::html;
use leptos::logging::*;
use leptos::prelude::*;
use shakmaty::KnownOutcome;
use shakmaty::fen::*;
use shakmaty::san::*;

use crate::types::Error;

#[derive(Clone, Debug, PartialEq)]
enum State {
    Username,
    Password {
        user_name: String,
    },
    PasswordConfirm {
        user_name: String,
        first: Vec<(San, Fen)>,
    },
    Done {
        user_name: String,
        first: Vec<(San, Fen)>,
        second: Vec<(San, Fen)>,
    },
}

#[component]
pub fn RegisterPage() -> impl IntoView {
    let (state, set_state) = signal(State::Password { user_name: "lol".to_string() });

    let current_view = move || match state.get() {
        State::Username => {
            let (user_name, set_username) = signal(String::new());

            let on_click = move |_| {
                set_state.set(State::Password {
                    user_name: user_name.get(),
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
                    <button on:click=on_click class="p-4 w-full text-2xl button-primary">
                        "Continue"
                    </button>
                </div>
            })
        }
        State::Password { user_name } => {
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

            

            let (ended, set_ended) = signal(Option::<KnownOutcome>::Some(KnownOutcome::Draw));

            let on_finished = move |o: KnownOutcome| {
                log!("ended {o:?}");
                set_ended.set(Some(o));
            };

            EitherOf4::B(view! {
                <div class="flex flex-col gap-2.5 justify-start items-center w-full h-full">
                    <div class="flex flex-col justify-start items-start w-full text-2xl h-fit">
                        "Hi " {user_name}
                        <span class="font-sans font-light">
                            "Let's make a password! Play a game of chess with yourself until the game is over"
                        </span>
                    </div>
                    <div class="flex flex-col justify-center items-center w-full h-full">
                        <ChessBoard on_finished notation />
                        <dialog open=move || ended.with(Option::is_some)>
                            <div class="flex flex-col gap-2.5 justify-start items-center p-5 text-3xl text-white rounded-lg bg-background/95">
                                "Game ended!"
                                <button class="p-4 w-full text-2xl button-primary">
                                    "Continue!"
                                </button>
                            </div>
                        </dialog>
                    </div>
                </div>
            })
        }
        State::PasswordConfirm { user_name, first } => EitherOf4::C(()),
        State::Done {
            user_name,
            first,
            second,
        } => EitherOf4::D(()),
    };

    view! {
        <div class="flex flex-col justify-center items-center p-5 w-full h-full">
            {current_view}
        </div>
    }
}

#[server]
async fn test() -> Result<(), Error> {
    use crate::types::AppState;
    let app_state = expect_context::<AppState>();
    let mut transaction = app_state.db.pool.begin().await?;

    Ok(())
}
