use chrono::{DateTime, Local};

struct ProductTable {
    id: i32,
    name: String,
    price: i32,
    category_id: i64,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
}
