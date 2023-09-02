use crate::model::product::{NewProduct, Product, UpdateProduct};
use async_trait::async_trait;

#[async_trait]
pub trait ProductRepository {
    async fn create(&self, source: NewProduct) -> anyhow::Result<Product>;
    async fn find(&self, id: i32) -> anyhow::Result<Product>;
    async fn all(&self) -> anyhow::Result<Vec<Product>>;
    async fn update(&self, id: i32, source: UpdateProduct) -> anyhow::Result<Product>;
    async fn delete(&self, id: i32) -> anyhow::Result<()>;
}
