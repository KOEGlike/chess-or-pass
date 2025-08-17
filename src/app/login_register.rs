use leptos::prelude::*;
use leptos::logging::*;
use shakmaty::san::San;
use crate::app::{chess::ChessBoard, general_components::*};

#[component]
pub fn LoginRegisterPage() -> impl IntoView {

    let on_finished=|_|{};
    let notation: RwSignal<Vec<San>>=RwSignal::new(vec![]);
    Effect::new(move |_|{
        let s=notation.get().into_iter().map(|e|e.to_string()).collect::<Vec<String>>().join(" ");
        log!("san {s}");
    });

    view! {
        <div class="bg-background flex h-full w-full items-center justify-center">
            <ChessBoard on_finished notation />
            <ButtonPrimary>"d"</ButtonPrimary>
        </div>
    }
}