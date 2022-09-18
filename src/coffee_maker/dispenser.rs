use std::thread;

use log::{trace, warn};

use super::orders::{Ingredients, Order, Orders};

fn handle_order(ingredients: Ingredients, name: &str) {
    trace!("[{name}] Dispensing order: {:?}", ingredients);
    thread::sleep(std::time::Duration::from_millis(100));
    trace!("[{name}] Dispensed order: {:?}", ingredients);
}

fn dispenser(id: u16, orders: Orders) -> () {
    let name = format!("Dispenser {id}");

    let mut orders = orders.lock().unwrap();
    let order = orders.pop_front();

    match order {
        Some(Order::Order(ingredients)) => {
            trace!("[{name}] Took order: {:?}", order);
            drop(orders);
            handle_order(ingredients, &name);
        }
        Some(Order::NoMoreOrders) => {
            trace!("[{name}] No more orders");
            orders.push_front(Order::NoMoreOrders);
        }
        None => {
            warn!("[{name}] Found no orders");
        }
    }
}

pub fn spawn_dispenser(id: u16, orders: Orders) -> thread::JoinHandle<()> {
    thread::spawn(move || dispenser(id, orders))
}
