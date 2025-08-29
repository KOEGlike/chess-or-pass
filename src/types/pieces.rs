#[cfg(feature = "ssr")]
#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, serde::Deserialize, serde::Serialize)]
#[sqlx(type_name = "pieces", rename_all = "lowercase")]
enum Pieces {
    BB,
    BK,
    BN,
    BP,
    BQ,
    BR,
    WB,
    WK,
    WN,
    WP,
    WQ,
    WR,
}

#[cfg(feature = "ssr")]
impl From<shakmaty::Piece> for Pieces {
    fn from(piece: shakmaty::Piece) -> Self {
        match (piece.color, piece.role) {
            (shakmaty::Color::Black, shakmaty::Role::Bishop) => Pieces::BB,
            (shakmaty::Color::Black, shakmaty::Role::King) => Pieces::BK,
            (shakmaty::Color::Black, shakmaty::Role::Knight) => Pieces::BN,
            (shakmaty::Color::Black, shakmaty::Role::Pawn) => Pieces::BP,
            (shakmaty::Color::Black, shakmaty::Role::Queen) => Pieces::BQ,
            (shakmaty::Color::Black, shakmaty::Role::Rook) => Pieces::BR,
            (shakmaty::Color::White, shakmaty::Role::Bishop) => Pieces::WB,
            (shakmaty::Color::White, shakmaty::Role::King) => Pieces::WK,
            (shakmaty::Color::White, shakmaty::Role::Knight) => Pieces::WN,
            (shakmaty::Color::White, shakmaty::Role::Pawn) => Pieces::WP,
            (shakmaty::Color::White, shakmaty::Role::Queen) => Pieces::WQ,
            (shakmaty::Color::White, shakmaty::Role::Rook) => Pieces::WR,
        }
    }
}
