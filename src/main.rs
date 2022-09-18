mod coffee_maker;
use coffee_maker::take_orders::take_orders;
use std::sync::{Arc, Mutex};

fn main() {
    let orders: coffee_maker::orders::Orders = Arc::new(Mutex::new(Vec::new()));
    let handle = take_orders(String::from("./assets/orders.csv"), orders.clone());

    handle.join().unwrap();
    println!("{:?}", orders);
}
