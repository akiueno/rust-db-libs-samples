use super::Id;

#[derive(Clone)]
pub struct ProductCategory {
    id: Id<ProductCategory>,
    name: String,
}
