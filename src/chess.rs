use leptos::either::Either;
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
        set_selected_piece.set(None)
    });

    let move_chess = move |m: Move| {
        let next = match chess.get().play(m) {
            Err(e) => {
                error!("got error from chess: {e}");
                return;
            }
            Ok(c) => c,
        };
        set_chess.set(next);
    };

    let pieces = move || {
        chess
            .read()
            .board()
            .iter()
            .map(|(square, piece)| {
                let on_click = move |_| {
                    log!("clicked on {square:?}");
                    match selected_piece.get() {
                        Some((selected_square, _selected_piece)) => {
                            if selected_square == square {
                                log!("set none");
                                set_selected_piece.set(None);
                                return;
                            }
                            set_selected_piece.set(Some((square, piece)))
                        }
                        None => set_selected_piece.set(Some((square, piece))),
                    }
                };

                let rotated_square = square.rotate_270();
                let file = rotated_square.file().to_u32() + 1;
                let rank = rotated_square.rank().to_u32() + 1;

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
            .filter(|m| {
                log!("legal move {m:?}");
                match m {
                    Move::Normal { from, .. } | Move::EnPassant { from, .. } => *from == square,

                    Move::Castle { king, .. } => *king == square,
                    Move::Put { .. } => false,
                }
            })
            .map(|m| {
                let s = match m {
                    Move::Normal { to, .. } | Move::EnPassant { to, .. } => to,

                    Move::Castle { rook, .. } => rook,

                    _ => return Either::Left(()),
                };

                log!("sq: {s:?}");

                let on_click = move |_| {
                    move_chess(m);
                };

                let s_rotated = s.rotate_270();
                let file = s_rotated.file().to_u32() + 1;
                let rank = s_rotated.rank().to_u32() + 1;
                Either::Right(view! {
                    <div
                        class="m-4 rounded-4xl bg-zinc-700/50 z-30"
                        style:grid-column=rank.to_string()
                        style:grid-row=file.to_string()
                        on:click=on_click
                    />
                })
            })
            .collect_view();

        Either::Right(indicators)
    };

    view! {
        <div
            class="bg-[url(/board.png)] w-160 h-160 grid-cols-8 grid-rows-8 grid bg-contain transition-transform duration-300 ease-in-out rounded-md"
            class=("rotate-180", move || current_color().is_black())
        >
            {move_indicators}
            {pieces}

        </div>
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
