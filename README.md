# Internet of Coffee

![Coffee Maker](/assets/coffee_maker.gif)

Simulate a multi-nozzle coffee maker.

The machine possesses a predetermined amount of **coffee** , **foam** , **coffee beans** (to grind) , **milk** (to whip) as well as an unlimited supply of **water**.

The orders are read from `assets/orders.csv` as lines specifying _coffee (mg)_, _water (ml)_ and _milk (ml)_, in that order, separated by commas.

## Design

The following section describes and justifies the design decisions made for this project.

### Orders

`orders` `take_orders`

To emulate [how a real coffee machine works](https://www.youtube.com/watch?v=ce22H2-0xh4&ab_channel=IntoTheOrdinary) in the most accurate way possible; a dispenser must first apply the coffee, then water and finally add the foam.

It is assumed that the cups can't hold more coffee or foam than the machines containers could. Which means that fulfilling an order wouldn't require multiple bean grindings or milk whippings.

It is also assumed that the machine's resources are enough to fulfill all the orders.

Orders are queued concurrently in a `Mutex<VecDeque>` and kept track through a `Semaphore` to be fulfilled in FIFO order.

> `Orders` provides an interface similar to a queue that can be used safely by multiple threads.
> Differently, `pop` will block until an order is available; which is achieved through the use of a `Semaphore`.
> This means the struct should be used conscientiously, as it could lead to a permanent lock if there are no producers.

A single thread is dedicated to reading the input file, acting as the single producer to the dispensers which consume orders.
An `Orders::NoMoreOrders` object is used to signal that the dispensers should stop, after fulfilling all orders.

### Resources

`resources` `resource_monitor`

- resources :
  - coffee/foam have locked access ( only one at a time )
  - will produce only what is needed
  - as we asume a cup does not surpass the container, we wont need multiple grinding / whipping
- output : clears/refreshes screen to update data , friendly realtime UI, rather than logging

### Dispensers

`dispenser`

## Development

This project was bootstrapped with [Cargo](https://doc.rust-lang.org/cargo/), the [Rust](https://www.rust-lang.org/) package manager.

Some commands you might find useful:

- `cargo test` - Run the tests.
- `cargo build` - Compile the project.
- `cargo run` - Build and Run the project.
