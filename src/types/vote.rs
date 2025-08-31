use crate::types::pieces::Pieces;
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Vote {
    pub id: String,
    pub username: String,
    pub first_piece: Pieces,
    pub second_piece: Pieces,
    pub voted_for_first: bool,
    pub reason: Option<String>,
    pub created_at: DateTime<Utc>,
}
