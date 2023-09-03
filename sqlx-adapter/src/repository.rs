mod product;

use derive_new::new;
use std::marker::PhantomData;

#[derive(new)]
struct RepositoryForSqlx<T> {
    pool: sqlx::PgPool,
    _marker: PhantomData<T>,
}
