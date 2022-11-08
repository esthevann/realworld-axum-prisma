use prisma::PrismaClient;
pub use ::prisma_client_rust::{QueryError, NewClientError, prisma_errors::query_engine::{RecordNotFound, UniqueKeyViolation}};

pub mod prisma;
pub mod query;
pub mod mutation;

#[derive(Debug)]
pub enum DbErr {
    NotFound,
    QueryError(QueryError),
    Unauthorized
}

impl From<QueryError> for DbErr {
    fn from(value: QueryError) -> Self {
        Self::QueryError(value)
    }
}

pub async fn get_client() -> Result<PrismaClient, NewClientError> {
    PrismaClient::_builder().build().await
}