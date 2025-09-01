use leptos::{
    prelude::*,
    server_fn::{codec::JsonEncoding, error::IntoAppError},
};

#[derive(thiserror::Error, Debug, serde::Serialize, serde::Deserialize, Clone)]
pub enum Error {
    #[error("Error from database: {0}")]
    Database(String),
    #[error("Error from serde decode: {0}")]
    Decode(String),
    #[error("Error from serde encode: {0}")]
    Encode(String),
    #[error("Error with web socket")]
    WebSocket(String),
    #[error("This action is not allowed for you: {0}")]
    Forbidden(String),
    #[error("There is something missing or something that is not allow with the file system: {0}")]
    FileSystem(String),
    #[error("A entry was not found: {0}")]
    DoesNotExist(String),
    #[error("Server fn error: {0}")]
    ServerFnError(ServerFnErrorErr),
    #[error("An impossible chess game was attempted")]
    ImpossibleChessGame,
    #[error("The password is incorrect")]
    WrongPassword,
    #[error("The password is too short")]
    PasswordTooShort,
}

#[cfg(feature = "ssr")]
impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        if let sqlx::Error::RowNotFound = e {
            return Error::DoesNotExist(format!("sqlx error: {:?}", e.to_string()));
        }
        Error::Database(format!("sqlx error: {:?}", e.to_string()))
    }
}

impl FromServerFnError for Error {
    type Encoder = JsonEncoding;

    fn from_server_fn_error(value: leptos::prelude::ServerFnErrorErr) -> Self {
        Error::ServerFnError(value)
    }
}
