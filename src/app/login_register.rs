use leptos::prelude::*;
use shakmaty::san::San;
use crate::app::chess::ChessBoard;

#[component]
pub fn LoginRegisterPage() -> impl IntoView {

    let on_finished=|_|{};
    let notation: RwSignal<Vec<San>>=RwSignal::new(vec![]);

    view! { <ChessBoard on_finished notation /> }
}