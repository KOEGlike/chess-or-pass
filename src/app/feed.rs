use std::ops::Range;

use chrono::{DateTime, Utc};
use leptos::logging::*;
use leptos::prelude::*;
use shakmaty::Piece;

use crate::app::chess::piece_to_img;
use crate::types::{Error, Pieces, Vote};

const PAGE_SIZE: i64 = 20;

#[component]
pub fn Feed() -> impl IntoView {
    let (length, set_length) = signal::<i64>(0);

    let votes = Resource::new(move || 0..length.get(), fetch_feed);

    let suspense = move || {
        Suspend::new(async move {
            let votes = votes.await.unwrap_or_default();

            votes
                .into_iter()
                .map(|vote| view! {<VoteComponent vote />})
                .collect_view()
        })
    };

    view! {
        <div class="h-full w-full flex flex-col justify-start gap-4 p-4">
            <span class="w-full h-fit text-3xl">"Feed"</span>
            {suspense}
        </div>

    }
}

#[component]
fn VoteComponent(vote: Vote) -> impl IntoView {
    view! {
        <div class="w-full h-fit p-2 rounded-2xl bg-secondary ">
            <div class="flex flex-col justify-between items-center">
                <VotePiece piece=vote.first_piece.into() voted_for=vote.voted_for_first/>
                <div class="rounded-full -m-1.5 w-5 h-5 bg-secondary">"OR"</div>
                <VotePiece piece=vote.second_piece.into() voted_for=!vote.voted_for_first/>
            </div>
        </div>
    }
}

#[component]
fn VotePiece(piece: Piece, voted_for: bool) -> impl IntoView {
    let img_src = piece_to_img(&piece);
    view! {
        <img class="w-16 h-16 rounded-lg border-2 " class:border-green-700=voted_for class:border-red-700=!voted_for src=img_src alt=""/>
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
