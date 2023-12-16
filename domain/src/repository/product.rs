use crate::model::product::{NewProduct, Product, UpdateProduct};
use async_trait::async_trait;

#[async_trait]
#[cfg_attr(test, mockall::automock)]
pub trait ProductRepository {
    async fn create(&self, source: NewProduct) -> anyhow::Result<Product>;
    async fn find(&self, id: String) -> anyhow::Result<Product>;
    async fn all(&self) -> anyhow::Result<Vec<Product>>;
    async fn update(&self, id: String, source: UpdateProduct) -> anyhow::Result<Product>;
    async fn delete(&self, id: String) -> anyhow::Result<()>;
}

#[cfg(test)]
mod tests {
    use fake::{Fake, Faker};
    use mockall::predicate;

    use super::*;

    #[tokio::test]
    async fn test_mock_repository() {
        let mut product_repo = MockProductRepository::new();
        let expected_product = Faker.fake::<Product>();
        product_repo
            .expect_find()
            .with(predicate::eq(
                expected_product.get_id().get_value().to_string(),
            ))
            .times(1)
            .returning(Ok(expected_product.clone()));

        let found_product = product_repo
            .find(expected_product.get_id().get_value().to_string())
            .await
            .unwrap();

        assert_eq!(expected_product, found_product)
    }
}
