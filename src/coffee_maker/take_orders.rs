use std::{
    fs::File,
    io::{self, BufRead},
    thread::{self, JoinHandle},
};

use super::orders::{Order, Orders};

fn parse_line(line: io::Result<String>) -> io::Result<Order> {
    let line = line?;
    let mut iter = line.split(',');

    let error = io::ErrorKind::InvalidData;

    Ok(Order::Order(
        iter.next().ok_or(error)?.parse().or(Err(error))?,
        iter.next().ok_or(error)?.parse().or(Err(error))?,
        iter.next().ok_or(error)?.parse().or(Err(error))?,
    ))
}

fn load_order(order: Order, orders: &Orders) {
    let mut orders = orders.lock().unwrap();
    orders.push(order);
}

pub fn take_orders(orders_filename: String, orders: Orders) -> JoinHandle<()> {
    thread::spawn(move || {
        let orders_file = File::open(orders_filename).unwrap();
        let lines = io::BufReader::new(&orders_file).lines();

        for line in lines {
            match parse_line(line) {
                Ok(order) => load_order(order, &orders),
                Err(e) => println!("Error Reading Order: {}", e),
            }
        }

        let mut orders = orders.lock().unwrap();
        orders.push(Order::NoMoreOrders);
    })
}
