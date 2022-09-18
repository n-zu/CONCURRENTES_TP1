use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Order {
    Order(u32, u32, u32),
    NoMoreOrders,
}

pub type Orders = Arc<Mutex<Vec<Order>>>;
