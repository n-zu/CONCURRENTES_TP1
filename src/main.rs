mod coffee_maker;
use std::thread::JoinHandle;

use coffee_maker::{orders::Orders, spawn_dispenser, take_orders, Resources};

pub const DISPENSERS: u16 = 3;

fn main() {
    let orders = Orders::new();
    let order_taker_handle =
        take_orders(String::from("./assets/orders.csv"), orders.clone()).unwrap();

    let resources = Resources::new();
    let mut dispenser_handles: Vec<JoinHandle<()>> = Vec::new();
    for _ in 0..DISPENSERS {
        let handle = spawn_dispenser(orders.clone(), resources.clone());
        dispenser_handles.push(handle);
    }

    order_taker_handle.join().unwrap();

    for handle in dispenser_handles {
        handle.join().unwrap();
    }
}
