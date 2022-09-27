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

#[cfg(test)]
mod order_tests {
    use super::*;

    #[test]
    fn create_empty_order() {
        let order = Order::from(0, 0, 0).expect("Failed to create order");
        assert_eq!(
            order,
            Order::Order(Ingredients {
                coffee: 0,
                water: 0,
                foam: 0
            })
        );
    }

    #[test]
    fn create_order_with_valid_amount_of_coffee_and_foam() {
        let order = Order::from(config::C, 0, config::E);
        assert!(order.is_ok());
    }

    #[test]
    fn create_order_with_too_much_coffee() {
        let order = Order::from(config::C + 1, 0, 0);
        assert!(order.is_err());
    }

    #[test]
    fn create_order_with_too_much_foam() {
        let order = Order::from(0, 0, config::E + 1);
        assert!(order.is_err());
    }

    #[test]
    fn create_order_with_too_much_coffee_and_foam() {
        let order = Order::from(config::C + 1, 0, config::E + 1);
        assert!(order.is_err());
    }
}

#[cfg(test)]
mod orders_tests {

    use super::*;

    #[test]
    fn create_orders() {
        let orders = Orders::new();
        assert_eq!(orders.orders.lock().unwrap().len(), 0);
    }

    #[test]
    fn push_order() {
        let orders = Orders::new();
        orders.push(Order::from(0, 0, 0).expect("Failed to create order"));
        assert_eq!(orders.orders.lock().unwrap().len(), 1);
    }

    #[test]
    fn pop_order() {
        let orders = Orders::new();
        orders.push(Order::from(0, 0, 0).expect("Failed to create order"));
        assert_eq!(orders.orders.lock().unwrap().len(), 1);
        orders.pop();
        assert_eq!(orders.orders.lock().unwrap().len(), 0);
    }

    #[test]
    fn produce_and_consume_from_different_threads() {
        let order_num = 50;
        let orders = Orders::new();
        let orders_prod = orders.clone();
        let orders_cons = orders.clone();

        let producer = std::thread::spawn(move || {
            for i in 0..order_num {
                orders_prod.push(Order::from(0, i, 0).expect("Failed to create order"));
            }
        });

        let consumer = std::thread::spawn(move || {
            for i in 0..order_num {
                let order = orders_cons.pop();
                if let Order::Order(Ingredients { water: w, .. }) = order {
                    assert_eq!(w, i);
                } else {
                    panic!("Invalid order");
                }
            }
        });

        producer.join().expect("Failed to join producer");
        consumer.join().expect("Failed to join consumer");
    }
}
