use crate::types::*;
#[cfg(feature = "ssr")]
use axum::extract::FromRef;

#[cfg(feature = "ssr")]
#[derive(FromRef, Clone, Debug)]
pub struct AppState {
    pub db: Db,
    pub leptos_options: leptos::prelude::LeptosOptions,
}

#[cfg(feature = "ssr")]
impl AppState {
    pub async fn new(
        leptos_options: leptos::prelude::LeptosOptions,
        db_url: String,
    ) -> Result<Self, Error> {
        println!("Connecting to database...",);

        let db = Db::new(db_url).await?;
        println!("Connected to database...");

        Ok(Self { db, leptos_options })
    }
}
