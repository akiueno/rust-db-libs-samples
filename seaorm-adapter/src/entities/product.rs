//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.3

use chrono::{DateTime, Local};
use domain::model::product::{NewProduct, Product};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "seaorm", table_name = "product")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub name: String,
    pub price: i32,
    pub category_id: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::product_category::Entity",
        from = "Column::CategoryId",
        to = "super::product_category::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    ProductCategory,
}

impl Related<super::product_category::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ProductCategory.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

/*
 *
 *
 *
 *
 *
*/
impl From<NewProduct> for Model {
    fn from(s: NewProduct) -> Self {
        let created_at = Local::now();
        let updated_at = Local::now();
        Self {
            id: s.get_id().get_value().to_string(),
            name: s.get_name().to_string(),
            price: s.get_price(),
            category_id: s.get_category_id().get_value().to_string(),
            created_at,
            updated_at,
        }
    }
}

impl TryFrom<Model> for Product {
    type Error = anyhow::Error;
    fn try_from(s: Model) -> Result<Self, Self::Error> {
        Ok(Product::new(
            s.id.try_into()?,
            s.name,
            s.price,
            s.category_id.try_into()?,
        ))
    }
}
