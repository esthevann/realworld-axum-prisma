use prisma_client_rust::QueryError;

pub mod prisma;
pub mod query;
pub mod mutation;

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