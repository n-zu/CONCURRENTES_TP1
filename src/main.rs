mod coffee_maker;
use coffee_maker::take_orders::take_orders;

use crate::coffee_maker::orders::create_orders;

fn main() {
    let orders = create_orders();
    let handle = take_orders(String::from("./assets/orders.csv"), orders.clone());

    handle.join().unwrap();
    println!("{:?}", orders);
}
