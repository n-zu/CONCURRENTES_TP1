use std::{sync::Arc, thread};

use super::{
    orders::{Ingredients, Order, Orders},
    ResourceResult, Resources,
};

/// Handles a single order.
/// Taking the necessary ingredients from the resources.
fn handle_order(ingredients: Ingredients, resources: &Arc<Resources>) -> ResourceResult {
    resources.use_coffee(ingredients.coffee)?;
    resources.use_water(ingredients.water)?;
    resources.use_foam(ingredients.foam)?;
    Ok(())
}

/// Handles orders from the queue until there are no more orders.
fn dispenser(orders: Arc<Orders>, resources: Arc<Resources>) {
    while let Order::Order(ingredients) = orders.pop() {
        if let Err(_err) = handle_order(ingredients, &resources) {
            // unable to handle order
        }
    }
    orders.push(Order::NoMoreOrders);
}

/// Spawns a new dispenser thread and returns its handle.
/// The dispenser will handle orders from the given queue.
/// The dispenser will stop when it receives a `NoMoreOrders` order, leaving it in the queue.
pub fn spawn_dispenser(orders: Arc<Orders>, resources: Arc<Resources>) -> thread::JoinHandle<()> {
    thread::spawn(move || dispenser(orders, resources))
}

#[cfg(test)]
mod dispenser_tests {

    use super::*;

    #[test]
    fn dispenser_consumes_resources() {
        let orders = Orders::new();
        let resources = Resources::new(100, 0, 100, 0).expect("Failed to create resources");
        let dispenser = spawn_dispenser(orders.clone(), resources.clone());

        for _ in 0..9 {
            orders.push(Order::from(10, 10, 10).expect("Failed to create order"));
        }
        orders.push(Order::NoMoreOrders);

        dispenser.join().expect("Failed to join dispenser thread");

        resources.use_coffee(10).expect("Should not be empty");
        resources.use_foam(10).expect("Should not be empty");

        resources.use_coffee(10).expect_err("Should be empty");
        resources.use_foam(10).expect_err("Should be empty");
    }
}
