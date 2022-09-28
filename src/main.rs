mod coffee_maker;
use std::{sync::atomic::Ordering, thread::JoinHandle};

use coffee_maker::{config, orders::Orders, spawn_dispenser, take_orders, Resources};

pub const DISPENSERS: u16 = 3;

const INITIAL_COFFEE: u32 = config::C;
const INITIAL_COFFEE_BEANS: u32 = config::G;
const INITIAL_FOAM: u32 = config::E;
const INITIAL_MILK: u32 = config::L;

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
    let (monitor_handle, stop_monitor) = resources.monitor(300);

    let mut dispenser_handles: Vec<JoinHandle<()>> = Vec::new();
    for _ in 0..DISPENSERS {
        let handle = spawn_dispenser(orders.clone(), resources.clone());
        dispenser_handles.push(handle);
    }

    order_taker_handle.join().expect("Order Taker Panicked");

    for handle in dispenser_handles {
        handle.join().expect("Dispenser Panicked");
    }

    stop_monitor.store(true, Ordering::Relaxed);
    monitor_handle.join().expect("Monitor Panicked");
}
