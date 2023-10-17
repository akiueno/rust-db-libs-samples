use chrono::{DateTime, Local};
use domain::model::product::Product;

#[derive(sqlx::FromRow)]
pub(crate) struct ProductTable {
    id: String,
    name: String,
    price: i32,
    category_id: String,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
}

impl TryFrom<ProductTable> for Product {
    type Error = anyhow::Error;
    fn try_from(source: ProductTable) -> Result<Self, Self::Error> {
        Ok(Self::new(
            source.id.try_into()?,
            source.name,
            source.price,
            source.category_id.try_into()?,
        ))
    }
}
