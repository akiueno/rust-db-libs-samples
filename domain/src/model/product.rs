pub struct NewProduct {
    name: String,
    price: i32,
    category_id: i64,
}

pub struct UpdateProduct {
    name: String,
    price: i32,
    category_id: i64,
}

pub struct Product {
    id: i64,
    name: String,
    price: i32,
    category_id: i64,
}
