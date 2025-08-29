use leptos::{children, either::Either, ev::MouseEvent, prelude::*};
use shakmaty::{Color, KnownOutcome};

#[component]
pub fn GameEndModal(
    #[prop(into)] ended: Signal<Option<KnownOutcome>>,
    on_continue: impl FnMut(MouseEvent) + 'static,
) -> impl IntoView {
    view! {
        <GameModal
            visible=Signal::derive(move || ended.get().is_some())
            main_text=Signal::derive(move || {
                match ended.get() {
                    Some(KnownOutcome::Decisive { winner: Color::White }) => "White Wins",
                    Some(KnownOutcome::Decisive { winner: Color::Black }) => "Black Wins",
                    Some(KnownOutcome::Draw) => "Draw",
                    None => "",
                }
                    .to_string()
            })
            sub_text="Game Over".to_string()
            button_text="Continue"
            on_click=on_continue
        />
    }
}

#[component]
pub fn GameModal(
    #[prop(into)] visible: Signal<bool>,
    #[prop(into)] main_text: Signal<String>,
    #[prop(optional)]
    #[prop(into)]
    sub_text: Signal<Option<String>>,
    #[prop(into)] button_text: Signal<String>,
    on_click: impl FnMut(MouseEvent) + 'static,
) -> impl IntoView {
    view! {
        <dialog class="absolute" class:hidden=move || !visible.get()>
            <div class="flex overflow-hidden flex-col justify-start items-center text-3xl text-white rounded-lg bg-background">
                <div class="flex flex-col justify-start items-center m-5 w-fit h-fit">
                    <span class="">{main_text}</span>
                    {if let Some(sub) = sub_text.get() {
                        Either::Left(
                            view! { <span class="font-sans text-xl font-light">{sub}</span> },
                        )
                    } else {
                        Either::Right(())
                    }}
                </div>
                <div class="p-5 w-full h-full bg-secondary">
                    <button on:click=on_click class="p-4 w-full text-2xl button-primary">
                        {button_text}
                    </button>
                </div>
            </div>
        </dialog>
    }
}
