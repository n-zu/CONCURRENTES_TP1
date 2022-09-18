use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Order {
    Order(u32, u32, u32),
    NoMoreOrders,
}

pub type Orders = Arc<Mutex<Vec<Order>>>;

pub fn create_orders() -> Orders {
    Arc::new(Mutex::new(Vec::new()))
}
