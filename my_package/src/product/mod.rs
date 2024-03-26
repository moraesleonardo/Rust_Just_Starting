use crate::Category;

#[derive(Debug, PartialEq)]
pub struct Product {
    id: u64,
    name: String,
    price: f64,
    category: Category,
}

pub mod category;

impl Product {
    pub fn new(id: u64, name: String, price: f64, category: Category) -> Self {
        Product {
            id,
            name,
            price,
            category,
        }
    }

    fn calculate_tax(&self) -> f64 {
        self.price * 0.1
    }

    pub fn product_price(&self) -> f64 {
        self.price + self.calculate_tax()
    }
}
