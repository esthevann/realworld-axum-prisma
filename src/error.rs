use std::{net::AddrParseError, env::VarError};

use prisma_client_rust::{
    prisma_errors::query_engine::{RecordNotFound, UniqueKeyViolation},
    QueryError, NewClientError,
};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},

};
use tracing::info;

// Error handling for the routes
pub enum AppError {
    PrismaError(QueryError),
    NotFound,
    Unathorized,
    HashingError
}

impl From<QueryError> for AppError {
    fn from(error: QueryError) -> Self {
        match error {
            e if e.is_prisma_error::<RecordNotFound>() => AppError::NotFound,
            e => AppError::PrismaError(e),
        }
    }
}

// This centralizes all differents errors from our app in one place
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::PrismaError(error) if error.is_prisma_error::<UniqueKeyViolation>() => {
                StatusCode::CONFLICT
            }
            AppError::PrismaError(e) => {
                info!("BAD REQUEST: {e}");
                StatusCode::BAD_REQUEST
            },
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::Unathorized => StatusCode::UNAUTHORIZED,
            AppError::HashingError => StatusCode::INTERNAL_SERVER_ERROR
        };

        status.into_response()
    }
}

// Error type for the main function
#[derive(Debug)]
pub enum MainError {
    NewClientError(NewClientError),
    AddrParseError(AddrParseError),
    HmacMissing(VarError),
    BindingError,
    
}


impl From<NewClientError> for MainError {
    fn from(e: NewClientError) -> Self {
        Self::NewClientError(e)
    }
}

impl From<AddrParseError> for MainError {
    fn from(e: AddrParseError) -> Self {
        Self::AddrParseError(e)
    }
}

impl From<VarError> for MainError {
    fn from(e: VarError) -> Self {
        Self::HmacMissing(e)
    }
}

