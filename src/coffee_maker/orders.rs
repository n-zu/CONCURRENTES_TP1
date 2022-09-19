use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std_semaphore::Semaphore;

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

pub struct Orders {
    orders: Mutex<VecDeque<Order>>,
    semaphore: Semaphore,
}

impl Orders {
    pub fn new() -> Arc<Orders> {
        Arc::new(Orders {
            orders: Mutex::new(VecDeque::new()),
            semaphore: Semaphore::new(0),
        })
    }

    pub fn push(&self, order: Order) {
        let mut orders = self.orders.lock().unwrap();
        orders.push_back(order);
        self.semaphore.release();
    }

    pub fn pop(&self) -> Order {
        self.semaphore.acquire();
        let mut orders = self.orders.lock().unwrap();
        orders.pop_front().unwrap()
    }
}
