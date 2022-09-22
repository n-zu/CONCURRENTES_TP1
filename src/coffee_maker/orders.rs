use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std_semaphore::Semaphore;

use super::config;

/// Stores the ingredients that are needed to make a drink.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ingredients {
    pub coffee: u32,
    pub water: u32,
    pub foam: u32,
}

/// Encapsules an Order or the lack thereof.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Order {
    Order(Ingredients),
    NoMoreOrders,
}

impl Order {
    /// Creates an order from the given ingredients.
    pub fn from(coffee: u32, water: u32, foam: u32) -> Result<Order, String> {
        if coffee > config::C {
            Err("Coffee is too much".to_string())
        } else if foam > config::E {
            Err("Foam is too much".to_string())
        } else {
            Ok(Order::Order(Ingredients {
                coffee,
                water,
                foam,
            }))
        }
    }
}

/// Encapsules the orders that are to be made.
/// Functions as a thread-safe queue.
pub struct Orders {
    orders: Mutex<VecDeque<Order>>,
    semaphore: Semaphore,
}

impl Orders {
    /// Creates a new Orders instance.
    pub fn new() -> Arc<Orders> {
        Arc::new(Orders {
            orders: Mutex::new(VecDeque::new()),
            semaphore: Semaphore::new(0),
        })
    }

    /// Adds an order to the queue.
    pub fn push(&self, order: Order) {
        let mut orders = self.orders.lock().expect("Failed to lock orders");
        orders.push_back(order);
        self.semaphore.release();
    }

    /// Returns the next order in the queue.
    /// If there are no orders, the thread will be blocked until there is one.
    pub fn pop(&self) -> Order {
        self.semaphore.acquire();
        let mut orders = self.orders.lock().expect("Failed to lock orders");
        orders
            .pop_front()
            .expect("No orders in queue (Invalid State)")
    }
}
