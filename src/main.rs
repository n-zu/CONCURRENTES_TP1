mod coffee_maker;
use std::thread::JoinHandle;

use coffee_maker::{orders::Orders, spawn_dispenser, take_orders, Resources};

pub const DISPENSERS: u16 = 3;

const INITIAL_COFFEE: u32 = 1000;
const INITIAL_COFFEE_BEANS: u32 = 5000;
const INITIAL_FOAM: u32 = 1000;
const INITIAL_MILK: u32 = 5000;

fn main() {
    let orders = Orders::new();
    let order_taker_handle = take_orders(String::from("./assets/orders.csv"), orders.clone())
        .expect("Failed open orders");

    let resources = Resources::new(
        INITIAL_COFFEE,
        INITIAL_COFFEE_BEANS,
        INITIAL_FOAM,
        INITIAL_MILK,
    )
    .expect("Failed to create resources");
    let mut dispenser_handles: Vec<JoinHandle<()>> = Vec::new();
    for _ in 0..DISPENSERS {
        let handle = spawn_dispenser(orders.clone(), resources.clone());
        dispenser_handles.push(handle);
    }

    order_taker_handle.join().expect("Order Taker Panicked");

    for handle in dispenser_handles {
        handle.join().expect("Dispenser Panicked");
    }
}
