use leptos::logging::*;
use leptos::prelude::*;
use shakmaty::*;

#[component]
pub fn ChessBoard() -> impl IntoView {
    let (chess, set_chess) = signal(Chess::default());
    let current_color = move || chess.read().turn();

    let x = set_chess.write().legal_moves();

    let pieces = move || {
        chess
            .read()
            .board()
            .iter()
            .map(|(square, piece)| {
                let square = square.rotate_270();
                let file = square.file().to_u32() + 1;
                let rank = square.rank().to_u32() + 1;
                view! {
                    <img
                        class="transition-transform duration-300 ease-in-out"
                        style:grid-column=rank.to_string()
                        style:grid-row=file.to_string()
                        class=("rotate-180", move || current_color().is_black())
                        src=piece_to_img(&piece)
                    />
                }
            })
            .collect_view()
    };

    let move_piece = move |_| {
        let first_legal = chess.read().legal_moves()[0];
        set_chess.set(chess.get().play(first_legal).expect("lol no legal move"));
    };

    view! {
        <div
            class="bg-[url(/board.png)] w-160 h-160 grid-cols-8 grid-rows-8 grid bg-contain transition-transform duration-300 ease-in-out"
            class=("rotate-180", move || current_color().is_black())
        >
            {pieces}
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
