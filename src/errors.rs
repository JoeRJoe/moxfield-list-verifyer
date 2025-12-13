use rocket::Request;
use rocket::http::{ContentType, Status};
use rocket::response::{Responder, Response, Result};
use std::io::Cursor;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Moxfield API error: {0}")]
    MoxfieldApiError(#[from] reqwest::Error),
    #[error("Scryfall API error: {0}")]
    ScryfallApiError(String),
    #[error("Spellbook API error: {0}")]
    SpellbookApiError(String),
    #[error("Environment variable missing: {0}")]
    EnvVarMissing(#[from] std::env::VarError),
    #[error("Internal error: {0}")]
    Internal(String),
}

#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for AppError {
    fn respond_to(self, _: &'r Request<'_>) -> Result<'static> {
        let status = match self {
            AppError::MoxfieldApiError(_) => Status::BadGateway,
            AppError::ScryfallApiError(_) => Status::BadGateway,
            AppError::SpellbookApiError(_) => Status::BadGateway,
            AppError::EnvVarMissing(_) => Status::InternalServerError,
            AppError::Internal(_) => Status::InternalServerError,
        };

        let body = format!("{{\"error\": \"{}\"}}", self);

        Response::build()
            .status(status)
            .header(ContentType::JSON)
            .sized_body(body.len(), Cursor::new(body))
            .ok()
    }
}
