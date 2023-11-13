use derive_new::new;
use std::marker::PhantomData;
use thiserror::Error;

use crate::persistence::postgres::Db;

mod product;

#[derive(new)]
struct RepositoryForSqlx<T> {
    pub pool: Db,
    _marker: PhantomData<T>,
}

#[derive(Debug, Error)]
enum RepositoryError {
    #[error("Unexpected Error: {0}")]
    Unexpected(String),
    #[error("NotFound, id id {0}")]
    NotFound(String),
}
