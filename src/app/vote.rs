use std::ops::Range;

use chrono::{DateTime, Utc};
use leptos::either::Either;
use leptos::logging::*;
use leptos::prelude::*;
use leptos::reactive::spawn_local;
use leptos_router::hooks::use_navigate;
use leptos_router::NavigateOptions;
use shakmaty::Piece;

use crate::app::chess::piece_to_img;
use crate::app::feed::VotePiece;
use crate::types::{Error, Pieces, Vote as VoteType};

use strum::IntoEnumIterator;

#[component]
pub fn VotePage() -> impl IntoView {
    let (vote_count, set_vote_count) = signal(0);

    let vote_candidates = Resource::new(move || vote_count.get(), move |_| get_vote_candidates());

    let (user_id, set_user_id) = signal(None::<String>);
    let (selected_first, set_selected_first) = signal(None::<bool>);
    let (reason, set_reason) = signal(String::new());

    Effect::new(move || {
        use gloo::storage::{LocalStorage, Storage};

        let res = LocalStorage::get("id");

        if let Err(e) = &res {
            log!("No user_id found in local storage: {e:?}");
            use_navigate()("/login-register", NavigateOptions::default());
        }

        set_user_id.set(res.ok());
    });

    let vote_local = move |first: Pieces, second: Pieces| {
        spawn_local(async move {
            let user_id = match user_id.get_untracked() {
                Some(id) => id,
                None => return,
            };

            let selected_first = match selected_first.get_untracked() {
                Some(s) => s,
                None => return,
            };

            let reason = if reason.with_untracked(String::is_empty) {
                None
            } else {
                Some(reason.get_untracked())
            };

            vote(user_id, first, second, selected_first, reason)
                .await
                .unwrap_or_else(|e| error!("Error voting: {e}"));

            set_vote_count.update(|u| *u += 1);
        })
    };

    let suspense = move || {
        Suspend::new(async move {
            let (first, second) = match vote_candidates.await {
                Err(e) => {
                    return Either::Left(
                        view! { <div class="text-red-700">"Error loading vote candidates: " {e.to_string()}</div> },
                    );
                }
                Ok(c) => c,
            };

            Either::Right(view! {
                <div class="flex flex-row gap-4 justify-center items-center p-4 rounded-2xl w-fit h-fit bg-secondary">
                    <VotePiece
                        piece=first.into()
                        voted_for=selected_first
                        on:click=move |_| set_selected_first.set(Some(true))
                        hoverable=true
                    />
                    <div class="z-40 p-4 -m-9 bg-white rounded-full rotate-12 w-fit h-fit text-background">
                        "OR"
                    </div>
                    <VotePiece
                        piece=second.into()
                        voted_for=Signal::derive(move || selected_first.get().map(|b| !b))
                        on:click=move |_| set_selected_first.set(Some(false))
                        hoverable=true
                    />
                </div>
                <input
                    placeholder="Reason (optional)"
                    class="mx-8 w-100"
                    type="text"
                    bind:value=(reason, set_reason)
                />
                <button
                    class="text-3xl rounded-md button-primary w-70 h-25"
                    on:click=move |_| { vote_local(first, second) }
                >
                    "Submit!"
                </button>
            })
        })
    };

    view! {
        <div class="flex flex-col gap-6 justify-center items-center p-4 w-full h-full">
            {suspense}
        </div>
    }
}

#[server]
async fn get_vote_candidates() -> Result<(Pieces, Pieces), Error> {
    use rand::seq::IndexedRandom;

    let all_possibilities = Pieces::iter().collect::<Vec<_>>();

    let mut rng = rand::rng();

    let first = all_possibilities.choose(&mut rng).cloned();
    let second = all_possibilities
        .into_iter()
        .filter(move |p| *p != first.unwrap())
        .collect::<Vec<_>>()
        .choose(&mut rng)
        .cloned();

    Ok((first.unwrap(), second.unwrap()))
}

#[server]
async fn vote(
    user_id: String,
    first_piece: Pieces,
    second_piece: Pieces,
    voted_for_first: bool,
    reason: Option<String>,
) -> Result<(), Error> {
    use crate::types::AppState;
    let app_state = expect_context::<AppState>();

    let mut transaction = app_state.db.pool.begin().await?;

    let current_time = Utc::now();
    let vote_id = cuid2::cuid();

    sqlx::query!(
        r#"
        INSERT INTO votes (id, user_id, first_piece, second_piece, voted_for_first, reason, created_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
        vote_id,
        user_id,
        first_piece as Pieces,
        second_piece as Pieces,
        voted_for_first,
        reason,
        current_time
    ).execute(&mut *transaction)
    .await?;

    transaction.commit().await?;

    Ok(())
}
