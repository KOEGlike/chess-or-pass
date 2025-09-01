use std::f32::consts::E;
use std::ops::Range;

use chrono::{DateTime, Utc};
use leptos::either::Either;
use leptos::logging::*;
use leptos::prelude::*;
use shakmaty::Piece;

use crate::app::chess::piece_to_img;
use crate::types::{Error, Pieces, Vote};

const PAGE_SIZE: i64 = 20;

#[component]
pub fn FeedPage() -> impl IntoView {
    let (length, set_length) = signal::<i64>(PAGE_SIZE);

    let votes = Resource::new(move || 0..length.get(), fetch_feed);

    let suspense = move || {
        Suspend::new(async move {
            let votes = votes.await.unwrap_or_default();

            votes
                .into_iter()
                .map(|vote| view! { <VoteComponent vote /> })
                .collect_view()
        })
    };

    let on_more = move |_| {
        set_length.update(|len| *len += PAGE_SIZE);
    };

    view! {
        <div class="flex overflow-scroll flex-col gap-4 justify-start p-4 w-full h-full">
            <span class="w-full text-3xl h-fit">"Feed"</span>
            {suspense}
            <div class="flex justify-center w-full h-fit">
                <button on:click=on_more class="p-4 button-primary w-fit">
                    "More!"
                </button>
            </div>
        </div>
    }
}

#[component]
fn VoteComponent(vote: Vote) -> impl IntoView {
    view! {
        <div class="flex flex-row justify-between items-center p-4 w-full h-fit rounded-4xl border-[#ffffff1a]">
            <div class="flex flex-row gap-4 justify-center items-center p-4 rounded-2xl w-fit h-fit bg-secondary">
                <VotePiece piece=vote.first_piece.into() voted_for=vote.voted_for_first />
                <div class="z-40 p-4 -m-9 bg-white rounded-full rotate-12 w-fit h-fit text-background">
                    "OR"
                </div>
                <VotePiece piece=vote.second_piece.into() voted_for=!vote.voted_for_first />
            </div>
            <div class="flex flex-col gap-4 justify-start items-center p-4 w-auto h-auto rounded-2xl bg-secondary">
                <span class="text-2xl">"Voted by: " {vote.username}</span>
                {if let Some(reason) = vote.reason {
                    Either::Left(view! { <span class="text-lg italic">"Reason: " {reason}</span> })
                } else {
                    Either::Right(())
                }}
                <span class="text-sm italic">"At: " {vote.created_at.to_rfc2822()}</span>
            </div>
        </div>
    }
}

#[component]
pub fn VotePiece(
    piece: Piece,
    #[prop(optional)]
    #[prop(into)]
    voted_for: Signal<Option<bool>>,
    #[prop(optional)] hoverable: Option<bool>,
) -> impl IntoView {
    let img_src = piece_to_img(&piece);
    view! {
        <img
            class="w-80 h-80 rounded-lg border-2 bg-secondary border-[#ffffff1a]"
            class:bg-secondary-hover=move || voted_for.get().unwrap_or(false)
            class:hover:bg-secondary-hover=hoverable.unwrap_or(false)
            src=img_src
            alt=""
        />
    }
}

#[server]
async fn fetch_feed(range: Range<i64>) -> Result<Vec<Vote>, Error> {
    use crate::types::AppState;
    let app_state = expect_context::<AppState>();

    let mut transaction = app_state.db.pool.begin().await?;

    let votes = sqlx::query_as!(Vote,
    r#"
    SELECT votes.id, users.username, votes.first_piece as "first_piece: Pieces", votes.second_piece as "second_piece: Pieces", votes.voted_for_first, votes.reason, votes.created_at
    FROM votes
    JOIN users ON votes.user_id = users.id
    ORDER BY votes.created_at DESC
    LIMIT $1 OFFSET $2
    "#,
    range.end - range.start,
    range.start
    )
    .fetch_all(&mut *transaction)
    .await?;

    transaction.commit().await?;

    Ok(votes)
}
