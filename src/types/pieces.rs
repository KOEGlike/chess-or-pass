use strum_macros::EnumIter;

#[derive(
    Copy, Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize, EnumIter,
)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "pieces", rename_all = "lowercase"))]
pub enum Pieces {
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

impl From<Pieces> for shakmaty::Piece {
    fn from(val: Pieces) -> Self {
        match val {
            Pieces::BB => shakmaty::Piece {
                color: shakmaty::Color::Black,
                role: shakmaty::Role::Bishop,
            },
            Pieces::BK => shakmaty::Piece {
                color: shakmaty::Color::Black,
                role: shakmaty::Role::King,
            },
            Pieces::BN => shakmaty::Piece {
                color: shakmaty::Color::Black,
                role: shakmaty::Role::Knight,
            },
            Pieces::BP => shakmaty::Piece {
                color: shakmaty::Color::Black,
                role: shakmaty::Role::Pawn,
            },
            Pieces::BQ => shakmaty::Piece {
                color: shakmaty::Color::Black,
                role: shakmaty::Role::Queen,
            },
            Pieces::BR => shakmaty::Piece {
                color: shakmaty::Color::Black,
                role: shakmaty::Role::Rook,
            },
            Pieces::WB => shakmaty::Piece {
                color: shakmaty::Color::White,
                role: shakmaty::Role::Bishop,
            },
            Pieces::WK => shakmaty::Piece {
                color: shakmaty::Color::White,
                role: shakmaty::Role::King,
            },
            Pieces::WN => shakmaty::Piece {
                color: shakmaty::Color::White,
                role: shakmaty::Role::Knight,
            },
            Pieces::WP => shakmaty::Piece {
                color: shakmaty::Color::White,
                role: shakmaty::Role::Pawn,
            },
            Pieces::WQ => shakmaty::Piece {
                color: shakmaty::Color::White,
                role: shakmaty::Role::Queen,
            },
            Pieces::WR => shakmaty::Piece {
                color: shakmaty::Color::White,
                role: shakmaty::Role::Rook,
            },
        }
    }
}
