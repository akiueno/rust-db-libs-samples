use chrono::{DateTime, Local};
use domain::model::{product::Product, Id};

#[derive(sqlx::FromRow)]
pub(crate) struct ProductTable {
    id: i64,
    name: String,
    price: i32,
    category_id: i64,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
}

impl From<ProductTable> for Product {
    fn from(source: ProductTable) -> Self {
        Self::new(
            Id::new(source.id),
            source.name,
            source.price,
            Id::new(source.category_id),
        )
    }
}
