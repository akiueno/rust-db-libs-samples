use async_trait::async_trait;
use derive_new::new;
use domain::repository::product::ProductRepository;
use domain::model::product::{NewProduct, Product, UpdateProduct};


#[derive(new)]
struct ProductRepositoryForSqlx {
    pool: sqlx::PgPool,
}

#[async_trait]
impl ProductRepository for ProductRepositoryForSqlx {
    async fn create(&self, source: NewProduct) -> anyhow::Result<Product> {
        todo!()
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
