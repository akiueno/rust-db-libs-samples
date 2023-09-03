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
    fn from(product_table: ProductTable) -> Self {
        Self::new(
            Id::new(product_table.id),
            product_table.name,
            product_table.price,
            Id::new(product_table.category_id),
        )
    }
}
