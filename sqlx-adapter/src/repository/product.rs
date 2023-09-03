use super::RepositoryForSqlx;
use crate::model::product::ProductTable;
use async_trait::async_trait;
use domain::model::product::{NewProduct, Product, UpdateProduct};
use domain::repository::product::ProductRepository;

#[async_trait]
impl ProductRepository for RepositoryForSqlx<Product> {
    async fn create(&self, source: NewProduct) -> anyhow::Result<Product> {
        let product_table = sqlx::query_as::<_, ProductTable>(
            r#"
            insert into sqlx.product (name, price, category_id)
            values ($1, $2, $3)
            returning *
            "#,
        )
        .bind(source.get_name())
        .bind(source.get_price())
        .bind(source.get_category_id())
        .fetch_one(&self.pool)
        .await?;

        Ok(Product::from(product_table))
    }

    async fn find(&self, id: i32) -> anyhow::Result<Product> {
        todo!()
    }

    async fn all(&self) -> anyhow::Result<Vec<Product>> {
        todo!()
    }

    async fn update(&self, id: i32, source: UpdateProduct) -> anyhow::Result<Product> {
        todo!()
    }

    async fn delete(&self, id: i32) -> anyhow::Result<()> {
        todo!()
    }
}
