use std::{sync::Arc, thread};

use super::orders::{Ingredients, Order, Orders};

fn handle_order(ingredients: Ingredients) {
    thread::sleep(std::time::Duration::from_millis(1000));
    println!("Dispensed order: {:?}", ingredients);
}

fn dispenser(orders: Arc<Orders>) {
    while let Order::Order(ingredients) = orders.pop() {
        handle_order(ingredients);
    }
    orders.push(Order::NoMoreOrders);
}

pub fn spawn_dispenser(orders: Arc<Orders>) -> thread::JoinHandle<()> {
    thread::spawn(move || dispenser(orders))
}
