use crate::model::product::{NewProduct, Product, UpdateProduct};
use async_trait::async_trait;

#[async_trait]
#[cfg_attr(test, mockall::automock)]
pub trait ProductRepository {
    async fn create(&self, source: NewProduct) -> anyhow::Result<Product>;
    async fn find(&self, id: String) -> anyhow::Result<Product>;
    async fn all(&self) -> anyhow::Result<Vec<Product>>;
    async fn update(&self, source: UpdateProduct) -> anyhow::Result<Product>;
    async fn delete(&self, id: String) -> anyhow::Result<()>;
}
