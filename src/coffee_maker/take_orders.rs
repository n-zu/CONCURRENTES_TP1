use super::orders::{Order, Orders};
use std::{
    fs::File,
    io::{self, BufRead},
    sync::Arc,
    thread::{self, JoinHandle},
};

fn parse_line(line: io::Result<String>) -> io::Result<Order> {
    let line = line?;
    let mut iter = line.split(',');

    if iter.clone().count() != 3 {
        return Err(io::Error::from(io::ErrorKind::InvalidData));
    }

    let error = io::ErrorKind::InvalidData;

    let coffee = iter.next().ok_or(error)?.parse().or(Err(error))?;
    let water = iter.next().ok_or(error)?.parse().or(Err(error))?;
    let foam = iter.next().ok_or(error)?.parse().or(Err(error))?;

    let order = Order::from(coffee, water, foam);
    Ok(order)
}

fn take_orders_loop(orders_file: File, orders: Arc<Orders>) {
    let lines = io::BufReader::new(&orders_file).lines();

    for line in lines {
        if let Ok(order) = parse_line(line) {
            orders.push(order);
        }
    }

    orders.push(Order::NoMoreOrders);
}

pub fn take_orders(orders_filename: String, orders: Arc<Orders>) -> io::Result<JoinHandle<()>> {
    let orders_file = File::open(orders_filename)?;
    let handler = thread::spawn(move || take_orders_loop(orders_file, orders));
    Ok(handler)
}

#[cfg(test)]
mod parse_line_tests {
    use crate::coffee_maker::orders::Order;

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
        assert_eq!(res, Order::from(1, 2, 3));

        let input = "10,0,0";
        let res = parse_line(Ok(input.to_string())).unwrap();
        assert_eq!(res, Order::from(10, 0, 0));

        let input = "0,20,30";
        let res = parse_line(Ok(input.to_string())).unwrap();
        assert_eq!(res, Order::from(0, 20, 30));
    }
}

#[cfg(test)]
mod take_orders_tests {

    use std::{fs, io::Write};

    use crate::coffee_maker::orders::{Order, Orders};

    use super::*;

    fn create_file(filename: &str, contents: &str) {
        let mut file = fs::File::create(filename).unwrap();
        file.write_all(contents.as_bytes()).unwrap();
    }

    #[test]
    fn correctly_read_file() {
        let filename = "assets/_temp__take_orders__correctly_read_file.csv";
        create_file(filename, "1,2,3\n4,5,6\n7,8,9\n");

        let orders = Orders::new();
        let handle = take_orders(filename.to_string(), orders.clone()).unwrap();

        handle.join().unwrap();

        let expected = vec![
            Order::from(1, 2, 3),
            Order::from(4, 5, 6),
            Order::from(7, 8, 9),
            Order::NoMoreOrders,
        ];

        for order in expected {
            assert_eq!(orders.pop(), order);
        }

        fs::remove_file(filename).unwrap();
    }

    #[test]

    fn ignore_invalid_lines() {
        let filename = "assets/_temp__take_orders__ignore_invalid_lines.csv";
        create_file(filename, "Hello World!\n1,2,3\n4,5,6,7,8,9\n1,2,3");

        let orders = Orders::new();
        let handle = take_orders(filename.to_string(), orders.clone()).unwrap();

        handle.join().unwrap();

        let expected = vec![
            Order::from(1, 2, 3),
            Order::from(1, 2, 3),
            Order::NoMoreOrders,
        ];

        for order in expected {
            assert_eq!(orders.pop(), order);
        }

        fs::remove_file(filename).unwrap();
    }
}
