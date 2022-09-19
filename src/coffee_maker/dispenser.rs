use std::{sync::Arc, thread};

use super::{
    orders::{Ingredients, Order, Orders},
    Resources,
};

/// Handles a single order.
/// Taking the necessary ingredients from the resources.
fn handle_order(ingredients: Ingredients, resources: &Arc<Resources>) {
    println!("Handling order: {:?}", ingredients);
    resources.get_coffee(ingredients.coffee).unwrap();
    resources.get_water(ingredients.water).unwrap();
    resources.get_foam(ingredients.foam).unwrap();
    println!("Done handling order: {:?}", ingredients);
}

/// Handles orders from the queue until there are no more orders.
fn dispenser(orders: Arc<Orders>, resources: Arc<Resources>) {
    while let Order::Order(ingredients) = orders.pop() {
        handle_order(ingredients, &resources);
    }
    orders.push(Order::NoMoreOrders);
}

/// Spawns a new dispenser thread and returns its handle.
/// The dispenser will handle orders from the given queue.
/// The dispenser will stop when it receives a `NoMoreOrders` order, leaving it in the queue.
pub fn spawn_dispenser(orders: Arc<Orders>, resources: Arc<Resources>) -> thread::JoinHandle<()> {
    thread::spawn(move || dispenser(orders, resources))
}
