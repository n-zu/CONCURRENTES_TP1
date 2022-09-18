mod coffee_maker;
use std::thread::JoinHandle;

use coffee_maker::{orders::create_orders, spawn_dispenser, take_orders};

pub const DISPENSERS: u16 = 8;

fn main() {
    env_logger::init();

    let orders = create_orders();
    let order_taker_handle = take_orders(String::from("./assets/orders.csv"), orders.clone());

    let mut dispenser_handles: Vec<JoinHandle<()>> = Vec::new();

    for id in 0..DISPENSERS {
        let orders = orders.clone();
        let handle = spawn_dispenser(id, orders);
        dispenser_handles.push(handle);
    }

    order_taker_handle.join().unwrap();

    for handle in dispenser_handles {
        handle.join().unwrap();
    }
}
