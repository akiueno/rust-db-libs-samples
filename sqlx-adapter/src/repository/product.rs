use super::{RepositoryError, RepositoryForSqlx};
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
        .bind(source.get_category_id().get_value().to_string())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| RepositoryError::Unexpected(e.to_string()))?;

        Ok(Product::try_from(product_table)?)
    }

    async fn find(&self, id: i32) -> anyhow::Result<Product> {
        let product_table = sqlx::query_as::<_, ProductTable>(
            r#"
            select * from sqlx.product
            where id = $1
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => RepositoryError::NotFound(id),
            _ => RepositoryError::Unexpected(e.to_string()),
        })?;

        Ok(Product::try_from(product_table)?)
    }

    async fn all(&self) -> anyhow::Result<Vec<Product>> {
        let product_tables = sqlx::query_as::<_, ProductTable>(
            r#"
            select * from sqlx.product
            order by id desc;
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::Unexpected(e.to_string()))?;

        product_tables.into_iter().map(Product::try_from).collect()
    }

    async fn update(&self, id: i32, source: UpdateProduct) -> anyhow::Result<Product> {
        let old_product = self.find(id).await?;
        let updated_product = sqlx::query_as::<_, ProductTable>(
            r#"
            update sqlx.product
            set name = $1, price = $2, category_id = $3
            where id = $4
            returning *
            "#,
        )
        .bind(
            source
                .get_name()
                .as_deref()
                .unwrap_or(old_product.get_name()),
        )
        .bind(source.get_price().unwrap_or(*old_product.get_price()))
        .bind(
            source
                .get_category_id()
                .clone()
                .unwrap_or_else(|| old_product.get_category_id().clone())
                .get_value()
                .to_string(),
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => RepositoryError::NotFound(id),
            _ => RepositoryError::Unexpected(e.to_string()),
        })?;

        Ok(Product::try_from(updated_product)?)
    }

    async fn delete(&self, id: i32) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            delete from sqlx.product
            where id = $1
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => RepositoryError::NotFound(id),
            _ => RepositoryError::Unexpected(e.to_string()),
        })?;

        Ok(())
    }
}
