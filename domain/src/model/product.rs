use super::{product_category::ProductCategory, Id};
use derive_new::new;

#[derive(new)]
pub struct NewProduct {
    name: String,
    price: i32,
    category_id: Id<Product>,
}

impl NewProduct {
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_price(&self) -> i32 {
        self.price
    }
    pub fn get_category_id(&self) -> &Id<Product> {
        &self.category_id
    }
}

#[derive(Clone)]
pub struct UpdateProduct {
    name: Option<String>,
    price: Option<i32>,
    category_id: Option<Id<ProductCategory>>,
}

impl UpdateProduct {
    pub fn get_name(&self) -> &Option<String> {
        &self.name
    }
    pub fn get_price(&self) -> &Option<i32> {
        &self.price
    }
    pub fn get_category_id(&self) -> &Option<Id<ProductCategory>> {
        &self.category_id
    }
}

#[derive(new, Clone)]
pub struct Product {
    id: Id<Product>,
    name: String,
    price: i32,
    category_id: Id<ProductCategory>,
}

impl Product {
    pub fn get_id(&self) -> &Id<Product> {
        &self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_price(&self) -> &i32 {
        &self.price
    }
    pub fn get_category_id(&self) -> &Id<ProductCategory> {
        &self.category_id
    }
}
