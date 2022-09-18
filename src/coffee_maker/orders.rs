use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy)]
pub enum Order {
    Order(i32, i32, i32),
    NoMoreOrders,
}

pub type Orders = Arc<Mutex<Vec<Order>>>;
