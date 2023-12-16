use super::Id;
use derive_new::new;

#[derive(new, Debug, Clone, PartialEq)]
pub struct ProductCategory {
    id: Id<ProductCategory>,
    name: String,
}
