use leptos::either::Either;
use leptos::html::P;
use leptos::logging::*;
use leptos::prelude::*;
use shakmaty::san::San;
use shakmaty::*;


#[component]
pub fn ChessBoard(on_finished: impl Fn(Outcome ) + 'static, notation:RwSignal<Vec<San>>) -> impl IntoView {
    let (chess, set_chess) = signal(Chess::default());
    let current_color = Signal::derive(move || chess.read().turn());

    let (selected_piece, set_selected_piece) = signal::<Option<(Square, Piece)>>(None);


    Effect::new(move |_| {
        let c =chess.get();
        on_finished(c.outcome());
        set_selected_piece.set(None);
    });

    
        
    let move_chess = {move |m: Move| {
        let c=chess.get();
        let next = match c.play(m) {
            
            Err(e) => {
                error!("got error from chess: {e}");
                return;
            }
            Ok(c) => {
                notation.write().push(San::from_move(&c,m));
                c},
        };
        set_chess.set(next);
    }};
    

    let pieces = move || {
        let board=chess.read().board().clone();

        let on_click=move|square, piece|{
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

       view! { <Pieces board current_color on_click /> }
    };

    let move_indicators = move || {
        log!("selected: {:?}", selected_piece.get());

        let (square, _piece) = match selected_piece.get() {
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
                    Move::Normal {
                        from, promotion, ..
                    } => *from == square && promotion.is_none(),
                    Move::EnPassant { from, .. } => *from == square,
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

                Either::Right(view! { <Indicator square=s on:click=on_click /> })
            })
            .collect_view();

        let promotion_indicators = chess
            .get()
            .promotion_moves()
            .into_iter()
            .filter(|m| match m {
                Move::Normal {
                    role,
                    from,
                    capture,
                    to,
                    promotion,
                } => *from == square && promotion.expect("lol, this cant happen") == Role::Knight,
                _ => false,
            })
            .map(|m| {
                let (visible, set_visible) = signal(false);

                let s = match m {
                    Move::Normal { to, .. } => to,
                    _ => return Either::Left(()),
                };

                let pieces = chess
                    .read()
                    .promotion_moves()
                    .iter()
                    .filter_map(|m| match m {
                        Move::Normal {
                            from,
                            to,
                            promotion,
                            ..
                        } => {
                            if *from == square && *to == s {
                                *promotion
                            } else {
                                None
                            }
                        }
                        _ => None,
                    })
                    .map(|r| Piece {
                        color: current_color.get(),
                        role: r,
                    })
                    .collect::<Vec<_>>();

                let on_click = move |_| {set_visible.set(true)};

                let on_selected = move |r: Role| {
                    if let Move::Normal {
                        role,
                        from,
                        capture,
                        to,
                        promotion: _,
                    } = m
                    {
                        move_chess(Move::Normal {
                            role,
                            from,
                            capture,
                            to,
                            promotion: Some(r),
                        });
                    }
                };

                Either::Right(view! {
                    <Indicator square=s on:click=on_click />
                    <ChooseRole
                        position=m.to()
                        pieces
                        on_selected
                        style:display=move || {
                            match visible.get() {
                                true => "",
                                false => "none",
                            }
                        }
                        {..}
                        class:rotate-180=move || current_color.read().is_white()
                    />
                })
            }).collect_view();

        Either::Right(view! {
            {indicators}
            {promotion_indicators}
        })
    };

    view! {
        <div
            class="bg-[url(/board.png)] w-160 h-160 grid-cols-8 grid-rows-8 grid bg-contain transition-transform duration-300 ease-in-out rounded-md"
            class:rotate-180=move || current_color.read().is_white()
        >
            {move_indicators}
            {pieces}

        </div>
    }
}

#[component]
fn Pieces(board:Board, current_color:Signal<Color>, on_click:impl Fn(Square, Piece) + 'static + std::marker::Send + std::marker::Sync)->impl IntoView{
    let on_click=move|(square, piece)|on_click(square,piece);
    let on_click=Callback::new(on_click);

    board            .into_iter()
            .map(|(square, piece)| {
                let on_click = move |_| {
                    on_click.run((square,piece));
                };

                view! {
                    <ChessPiece
                        piece
                        position=square
                        on:click=on_click
                        class:rotate-180=move || { current_color.read().is_white() }
                    />
                }
            })
            .collect_view()
}


#[component]
fn Indicator(square: Square) -> impl IntoView {
    let file = square.file().to_u32() + 1;
    let rank = square.rank().to_u32() + 1;
    view! {
        <div class="p-4" style:grid-column=file.to_string() style:grid-row=rank.to_string()>
            <div class="rounded-4xl bg-zinc-700/50 z-30 rotate-180 h-full w-full" />
        </div>
    }
}

#[component]
fn ChooseRole(
    position: Square,
    pieces: Vec<Piece>,
    on_selected: impl Fn(Role) + 'static + std::marker::Sync + std::marker::Send,
) -> impl IntoView {
    let file = position.file().to_u32() + 1;
    let rank = position.rank().to_u32() + 1;

    let file_start = file as f32 - pieces.len() as f32 / 2.0;

    let on_selected=Callback::new(on_selected);

    let pieces = pieces
        .into_iter()
        .map(| piece| {
            let on_click = move |_| on_selected.run(piece.role);
            view! { <ChessPiece piece class:z-50=true on:click=on_click /> }
        })
        .collect_view();

    let end = { file_start + pieces.len() as f32 }.to_string();
    view! {
        <div
            class="bg-white rounded-md p-1 z-40 flex justify-between content-center"
            style:grid-column-start=file_start.to_string()
            style:grid-column-end=end
            style:grid-row=rank.to_string()
        >
            {pieces}
        </div>
    }
}

#[component]
fn ChessPiece(piece: Piece, #[prop(optional)] position: Option<Square>) -> impl IntoView {
    view! {
        <img
            class="transition-transform duration-300 ease-in-out rotate-180"
            style:grid-column=position
                .map(|p| p.file().to_u32() + 1)
                .map(|f| f.to_string())
                .unwrap_or_default()
            style:grid-row=position
                .map(|p| p.rank().to_u32() + 1)
                .map(|r| r.to_string())
                .unwrap_or_default()
            src=piece_to_img(&piece)
        />
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
