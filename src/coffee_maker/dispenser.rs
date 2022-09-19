use std::{sync::Arc, thread};

use super::orders::{Ingredients, Order, Orders};

/// Handles a single order.
/// Taking the necessary ingredients from the resources.
fn handle_order(ingredients: Ingredients) {
    thread::sleep(std::time::Duration::from_millis(1000));
    println!("Dispensed order: {:?}", ingredients);
}

/// Handles orders from the queue until there are no more orders.
fn dispenser(orders: Arc<Orders>) {
    while let Order::Order(ingredients) = orders.pop() {
        handle_order(ingredients);
    }
    orders.push(Order::NoMoreOrders);
}

/// Spawns a new dispenser thread and returns its handle.
/// The dispenser will handle orders from the given queue.
/// The dispenser will stop when it receives a `NoMoreOrders` order, leaving it in the queue.
pub fn spawn_dispenser(orders: Arc<Orders>) -> thread::JoinHandle<()> {
    thread::spawn(move || dispenser(orders))
}
