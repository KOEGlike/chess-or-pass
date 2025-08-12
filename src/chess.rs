use leptos::either::Either;
use leptos::ev::select;
use leptos::logging::*;
use leptos::prelude::*;
use shakmaty::*;

#[component]
pub fn ChessBoard() -> impl IntoView {
    let (chess, set_chess) = signal(Chess::default());
    let current_color = move || chess.read().turn();

    let (selected_piece, set_selected_piece) = signal::<Option<(Square, Piece)>>(None);
    Effect::new(move |_| {
        chess.read();
        set_selected_piece(None)
    });

    let move_piece = move |_| {
        let first_legal = chess.read().legal_moves()[0];
        set_chess.set(chess.get().play(first_legal).expect("lol no legal move"));
    };

    let pieces = move || {
        chess
            .read()
            .board()
            .iter()
            .map(|(square, piece)| {
                let square = square.rotate_270();
                let file = square.file().to_u32() + 1;
                let rank = square.rank().to_u32() + 1;

                let on_click = move |_| {
                    log!("clicked on {square:?}");
                    match selected_piece.get() {
                        Some((selected_square, selected_piece)) => {
                            if selected_square == square {
                                set_selected_piece.set(None);
                                return;
                            }
                            set_selected_piece(Some((square, piece)))
                        }
                        None => set_selected_piece(Some((square, piece))),
                    }
                };

                view! {
                    <img
                        class="transition-transform duration-300 ease-in-out"
                        style:grid-column=rank.to_string()
                        style:grid-row=file.to_string()
                        class=("rotate-180", move || current_color().is_black())
                        src=piece_to_img(&piece)
                        on:click=on_click
                    />
                }
            })
            .collect_view()
    };

    let move_indicators = move || {
        log!("selected: {:?}", selected_piece.get());

        let (square, piece) = match selected_piece.get() {
            Some(s) => s,
            None => return Either::Left(()),
        };

        log!("selected square: {square}");

        let indicators = chess
            .get()
            .legal_moves()
            .into_iter()
            .filter_map(|m| {
                log!("legal move {m:?}");
                match m {
                    Move::Normal { from, to, .. } | Move::EnPassant { from, to } => {
                        if from == square {
                            Some(to)
                        } else {
                            None
                        }
                    }

                    Move::Castle { king, rook } => {
                        if king == square {
                            Some(rook)
                        } else {
                            None
                        }
                    }
                    Move::Put { .. } => None,
                }
            })
            .map(|s| {
                log!("sq: {s:?}");
                let s = s.rotate_270();
                let file = s.file().to_u32() + 1;
                let rank = s.rank().to_u32() + 1;
                view! {
                    <div
                        class="m-4 rounded-4xl bg-amber-50"
                        style:grid-column=rank.to_string()
                        style:grid-row=file.to_string()
                    />
                }
            })
            .collect_view();

        Either::Right(indicators)
    };

    view! {
        <div
            class="bg-[url(/board.png)] w-160 h-160 grid-cols-8 grid-rows-8 grid bg-contain transition-transform duration-300 ease-in-out rounded-md"
            class=("rotate-180", move || current_color().is_black())
        >
            {pieces}
            {move_indicators}
        </div>
        <button on:click=move_piece>"lololol"</button>
    }
}

fn piece_to_img(piece: &Piece) -> &'static str {
    let color = piece.color;
    let role = piece.role;

    match (color, role) {
        (Color::Black, Role::Pawn) => "/pieces/bp.png",
        (Color::Black, Role::Knight) => "/pieces/bn.png",
        (Color::Black, Role::Bishop) => "/pieces/bb.png",
        (Color::Black, Role::Rook) => "/pieces/br.png",
        (Color::Black, Role::Queen) => "/pieces/bq.png",
        (Color::Black, Role::King) => "/pieces/bk.png",
        (Color::White, Role::Pawn) => "/pieces/wp.png",
        (Color::White, Role::Knight) => "/pieces/wn.png",
        (Color::White, Role::Bishop) => "/pieces/wb.png",
        (Color::White, Role::Rook) => "/pieces/wr.png",
        (Color::White, Role::Queen) => "/pieces/wq.png",
        (Color::White, Role::King) => "/pieces/wk.png",
    }
}
