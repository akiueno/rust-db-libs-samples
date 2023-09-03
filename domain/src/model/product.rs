use derive_new::new;

pub struct NewProduct {
    name: String,
    price: i32,
    category_id: i64,
}

impl NewProduct {
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_price(&self) -> i32 {
        self.price
    }
    pub fn get_category_id(&self) -> i64 {
        self.category_id
    }
}

pub struct UpdateProduct {
    name: String,
    price: i32,
    category_id: i64,
}

#[derive(new)]
pub struct Product {
    id: i64,
    name: String,
    price: i32,
    category_id: i64,
}
