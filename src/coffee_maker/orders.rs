use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ingredients {
    pub coffee: u16,
    pub water: u16,
    pub foam: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Order {
    Order(Ingredients),
    NoMoreOrders,
}

impl Order {
    pub fn from(coffee: u32, water: u32, foam: u32) -> Order {
        Order::Order(Ingredients {
            coffee: coffee as u16,
            water: water as u16,
            foam: foam as u16,
        })
    }
}

pub type Orders = Arc<Mutex<VecDeque<Order>>>;

pub fn create_orders() -> Orders {
    Arc::new(Mutex::new(VecDeque::new()))
}
