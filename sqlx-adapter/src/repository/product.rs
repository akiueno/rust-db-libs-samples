use std::sync::Arc;

use super::{RepositoryError, RepositoryForSqlx};
use crate::model::product::ProductTable;
use async_trait::async_trait;
use domain::model::product::{NewProduct, Product, UpdateProduct};
use domain::model::Id;
use domain::repository::product::ProductRepository;

#[async_trait]
impl ProductRepository for RepositoryForSqlx<Product> {
    async fn create(&self, source: NewProduct) -> anyhow::Result<Product> {
        let pool = Arc::clone(&self.pool.0);
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
        .fetch_one(&*pool)
        .await
        .map_err(|e| RepositoryError::Unexpected(e.to_string()))?;

        Ok(Product::try_from(product_table)?)
    }

    async fn find(&self, id: String) -> anyhow::Result<Product> {
        let pool = Arc::clone(&self.pool.0);
        let product_table = sqlx::query_as::<_, ProductTable>(
            r#"
            select * from sqlx.product
            where id = $1
            "#,
        )
        .bind(&id)
        .fetch_one(&*pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => RepositoryError::NotFound(id),
            _ => RepositoryError::Unexpected(e.to_string()),
        })?;

        Ok(Product::try_from(product_table)?)
    }

    async fn all(&self) -> anyhow::Result<Vec<Product>> {
        let pool = Arc::clone(&self.pool.0);
        let product_tables = sqlx::query_as::<_, ProductTable>(
            r#"
            select * from sqlx.product
            order by id desc;
            "#,
        )
        .fetch_all(&*pool)
        .await
        .map_err(|e| RepositoryError::Unexpected(e.to_string()))?;

        product_tables.into_iter().map(Product::try_from).collect()
    }

    async fn update(&self, id: String, source: UpdateProduct) -> anyhow::Result<Product> {
        let pool = Arc::clone(&self.pool.0);
        let old_product = self.find(id.clone()).await?;
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
        .bind(&id)
        .fetch_one(&*pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => RepositoryError::NotFound(id),
            _ => RepositoryError::Unexpected(e.to_string()),
        })?;

        Ok(Product::try_from(updated_product)?)
    }

    async fn delete(&self, id: String) -> anyhow::Result<()> {
        let pool = Arc::clone(&self.pool.0);
        sqlx::query(
            r#"
            delete from sqlx.product
            where id = $1
            "#,
        )
        .bind(&id)
        .execute(&*pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => RepositoryError::NotFound(id),
            _ => RepositoryError::Unexpected(e.to_string()),
        })?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use domain::model::Id;

    use super::*;
    use crate::persistence::postgres::Db;

    #[tokio::test]
    async fn todo_crud_scenario() {
        let product_id = Id::gen();
        let name = "product name".to_string();
        let price = 1000;
        let category_id = Id::gen();
        let expected_product =
            Product::new(product_id.clone(), name.clone(), price, category_id.clone());

        // create
        let db = Db::new().await;
        let repository = RepositoryForSqlx::new(db);
        let created_product = repository
            .create(NewProduct::new(
                product_id.clone(),
                name.clone(),
                price,
                category_id.clone(),
            ))
            .await
            .unwrap();
        assert_eq!(expected_product, created_product);

        // find
        let founded_product = repository.find(product_id.value.to_string()).await.unwrap();
        assert_eq!(expected_product, founded_product);

        // all
        let products = repository.all().await.unwrap();
        assert_eq!(vec![expected_product], products);

        // update
        let updated_product = repository
            .update(
                product_id.value.to_string(),
                UpdateProduct::new(Some(name.clone()), Some(1500), Some(category_id.clone())),
            )
            .await
            .unwrap();
        assert_eq!(
            Product::new(product_id.clone(), name.clone(), 1500, category_id.clone()),
            updated_product
        );

        // delete
        let res = repository.delete(product_id.value.to_string()).await;
        assert!(res.is_ok())
    }
}
