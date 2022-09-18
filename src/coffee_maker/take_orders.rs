use std::{
    fs::File,
    io::{self, BufRead},
    thread::{self, JoinHandle},
};

use super::orders::{Order, Orders};

fn parse_line(line: io::Result<String>) -> io::Result<Order> {
    let line = line?;
    let mut iter = line.split(',');

    if iter.clone().count() != 3 {
        return Err(io::Error::from(io::ErrorKind::InvalidData));
    }

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

#[cfg(test)]
mod parse_line_tests {
    use super::*;

    #[test]
    fn errors_bubble_up() {
        let errors = [
            Err(io::Error::from(io::ErrorKind::NotFound)),
            Err(io::Error::from(io::ErrorKind::InvalidData)),
            Err(io::Error::from(io::ErrorKind::Unsupported)),
        ];

        for err in errors {
            let res = parse_line(err);
            assert!(res.is_err());
        }
    }

    #[test]
    fn invalid_data() {
        let invalid_inputs = ["Hello World!", "1,2", "1,2,3,4", "1,2,-3", "1.0,2,3"];

        for line in invalid_inputs {
            let res = parse_line(Ok(line.to_string()));
            assert!(res.is_err());
        }
    }

    #[test]
    fn valid_data() {
        let input = "1,2,3";
        let res = parse_line(Ok(input.to_string())).unwrap();
        assert_eq!(res, Order::Order(1, 2, 3));

        let input = "10,0,0";
        let res = parse_line(Ok(input.to_string())).unwrap();
        assert_eq!(res, Order::Order(10, 0, 0));

        let input = "0,20,30";
        let res = parse_line(Ok(input.to_string())).unwrap();
        assert_eq!(res, Order::Order(0, 20, 30));
    }
}

#[cfg(test)]
mod take_orders_tests {

    use std::{fs, io::Write};

    use crate::coffee_maker::orders::create_orders;

    use super::*;

    fn create_file(filename: &str, contents: &str) {
        let mut file = fs::File::create(filename).unwrap();
        file.write_all(contents.as_bytes()).unwrap();
    }

    #[test]
    fn correctly_read_file() {
        let filename = "assets/_temp__take_orders__correctly_read_file.csv";
        create_file(filename, "1,2,3\n4,5,6\n7,8,9\n");

        let orders: Orders = create_orders();
        let handle = take_orders(filename.to_string(), orders.clone());

        handle.join().unwrap();

        let orders = orders.lock().unwrap();
        assert_eq!(
            *orders,
            vec![
                Order::Order(1, 2, 3),
                Order::Order(4, 5, 6),
                Order::Order(7, 8, 9),
                Order::NoMoreOrders
            ]
        );

        fs::remove_file(filename).unwrap();
    }

    #[test]

    fn ignore_invalid_lines() {
        let filename = "assets/_temp__take_orders__ignore_invalid_lines.csv";
        create_file(filename, "Hello World!\n1,2,3\n4,5,6,7,8,9\n1,2,3");

        let orders: Orders = create_orders();
        let handle = take_orders(filename.to_string(), orders.clone());

        handle.join().unwrap();

        let orders = orders.lock().unwrap();
        assert_eq!(
            *orders,
            vec![
                Order::Order(1, 2, 3),
                Order::Order(1, 2, 3),
                Order::NoMoreOrders
            ]
        );

        fs::remove_file(filename).unwrap();
    }
}
