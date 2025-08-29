use crate::app::{chess::ChessBoard, };
use leptos::either::EitherOf4;
use leptos::logging::*;
use leptos::prelude::*;
use shakmaty::fen::*;
use shakmaty::san::*;

use crate::types::Error;

#[derive(Clone, Debug, PartialEq)]
enum State{
    Username,
    Password{user_name: String},
    PasswordConfirm{user_name: String,first: Vec<(San,Fen)>},
    Done{user_name: String,first: Vec<(San,Fen)>, second: Vec<(San,Fen)>},
}

#[component]
pub fn RegisterPage() -> impl IntoView {
    //let on_finished = |_| {};
    let notation: RwSignal<Vec<(San,Fen)>> = RwSignal::new(vec![]);

    let (state, set_state) = signal(State::Username);
    
    Effect::new(move |_| {
        let s = notation
            .get()
            .into_iter()
            .map(|e| format!("san {} fen {}", e.0, e.1))
            .collect::<Vec<String>>()
            .join(" \n");
        log!("{s}");
    });

    let current_view=move||match state.get(){
        State::Username => {let (user_name, set_username)=signal(String::new()); EitherOf4::A(view! { <input type="text" bind:value=(user_name, set_username) /> })},
        State::Password { user_name } => EitherOf4::B(()),
        State::PasswordConfirm { user_name, first } => EitherOf4::C(()),
        State::Done { user_name, first, second } => EitherOf4::D(()),
    };

    view! { <div class="flex h-full w-full items-center justify-center">{current_view}</div> }
}



#[server]
async fn test() -> Result<(), Error> {
    use crate::types::AppState;
    let app_state = expect_context::<AppState>();
    let mut transaction =
        app_state.db.pool.begin().await?;


    
    Ok(())
}