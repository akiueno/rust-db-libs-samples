use crate::entities::prelude::Product as ProductTable;
use crate::entities::product::{ActiveModel, Model};
use crate::repository::{RepositoryError, RepositoryForSeaOrm};
use async_trait::async_trait;
use domain::model::product::{NewProduct, Product, UpdateProduct};
use domain::repository::product::ProductRepository;
use sea_orm::entity::Set;
use sea_orm::EntityTrait;
use sea_orm::IntoActiveModel;
use sea_orm::TransactionTrait;
use sea_orm::{ActiveModelTrait, DatabaseTransaction};
use std::sync::Arc;

#[async_trait]
impl ProductRepository for RepositoryForSeaOrm<Product> {
    async fn create(&self, source: NewProduct) -> anyhow::Result<Product> {
        let transaction = self.init().await?;
        let row = Model::from(source);
        // ActiveModelを取得する
        let new_product: ActiveModel = row.into_active_model();
        let insert_result = new_product.insert(&transaction).await?;

        Ok(Product::try_from(insert_result)?)
    }

    async fn find(&self, id: String) -> anyhow::Result<Product> {
        let transaction = self.init().await?;
        let result = ProductTable::find_by_id(id.clone())
            .one(&transaction)
            .await?
            .ok_or(RepositoryError::NotFound(id))?;

        Ok(Product::try_from(result)?)
    }

    async fn all(&self) -> anyhow::Result<Vec<Product>> {
        let transaction = self.init().await?;
        let results = ProductTable::find().all(&transaction).await?;

        results.into_iter().map(Product::try_from).collect()
    }

    async fn update(&self, source: UpdateProduct) -> anyhow::Result<Product> {
        let transaction = self.init().await?;
        // 更新対象を取得する
        let target = ProductTable::find_by_id(source.get_id().get_value().to_string())
            .one(&transaction)
            .await?
            .ok_or(RepositoryError::NotFound(
                source.get_id().get_value().to_string(),
            ))?;
        // ActiveModelを取得する
        let mut update_row: ActiveModel = target.clone().into_active_model();
        // 値を変更する
        update_row.name = Set(source.get_name().clone().unwrap_or(target.clone().name));
        update_row.price = Set(source.get_price().unwrap_or(target.price));
        update_row.category_id = Set(source
            .get_category_id()
            .clone()
            .unwrap_or(target.category_id.try_into()?)
            .get_value()
            .to_string());
        // レコードを更新する
        let update_result = update_row.update(&transaction).await?;
        Ok(Product::try_from(update_result)?)
    }

    async fn delete(&self, id: String) -> anyhow::Result<()> {
        let transaction = self.init().await?;
        // 削除対象を取得する
        let target = ProductTable::find_by_id(id.clone())
            .one(&transaction)
            .await?
            .ok_or(RepositoryError::NotFound(id))?;
        // ActiveModelを取得する
        let delete_row: ActiveModel = target.into_active_model();
        // 対象レコードを削除する
        delete_row.delete(&transaction).await?;
        Ok(())
    }
}

impl RepositoryForSeaOrm<Product> {
    async fn init(&self) -> anyhow::Result<DatabaseTransaction> {
        let pool = Arc::clone(&self.pool.0);
        let transaction = pool.begin().await?;

        Ok(transaction)
    }
}

#[cfg(test)]
mod test {
    use crate::persistence::pool::Db;
    use domain::model::Id;

    use super::*;

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
        let repository = RepositoryForSeaOrm::new(db);
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
            .update(UpdateProduct::new(
                product_id.clone(),
                Some(name.clone()),
                Some(1500),
                Some(category_id.clone()),
            ))
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
