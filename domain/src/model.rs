use std::marker::PhantomData;
use derive_new::new;

pub mod product;
pub mod product_category;

#[derive(new)]
pub struct Id<T> {
    pub value: i64,
    _marker: PhantomData<T>,
}
