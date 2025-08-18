use crate::app::{chess::ChessBoard, general_components::*};
use leptos::logging::*;
use leptos::prelude::*;
use shakmaty::san::San;

use crate::types::Error;

#[component]
pub fn LoginRegisterPage() -> impl IntoView {
    let on_finished = |_| {};
    let notation: RwSignal<Vec<San>> = RwSignal::new(vec![]);
    Effect::new(move |_| {
        let s = notation
            .get()
            .into_iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        log!("san {s}");
    });

    view! {
        <div class="bg-background flex h-full w-full items-center justify-center">
            <ChessBoard on_finished notation />
            <ButtonPrimary {..} class="h-12 w-36 rounded-3xl">
                "d"
            </ButtonPrimary>
        </div>
    }
}

#[server]
async fn test() -> Result<(), Error> {
    use crate::types::AppState;
    let app_state = expect_context::<AppState>();
    let mut transaction =
        app_state.db.pool.begin().await?;

    let res=sqlx::query!("SELECT * FROM votes").fetch_one(&mut *transaction).await?;
    res.
    Ok(())
}