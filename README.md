<h1>
Internet of Coffee

<a href="docs/README-ES.md">
  <img align="right" height="40"
  alt="ES" src="https://cdn-icons-png.flaticon.com/512/197/197593.png">
</a>

</h1>

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

Orders are queued concurrently in a `Mutex<VecDeque>` and kept track through a `Semaphore`.

> `Orders` provides an interface similar to a queue that can be used safely by multiple threads.
> Differently, `pop` will block until an order is available; which is achieved through the use of a `Semaphore`.
> This means the struct should be used conscientiously, as it could lead to a permanent lock if there are no producers.

A single thread is dedicated to reading the input file, acting as the single producer to the dispensers which consume orders.
An `Orders::NoMoreOrders` object is used to signal that the dispensers should stop, after fulfilling all orders.

### Resources

`resources` `resource_monitor`

`Resources` provides a simple interface for using a set amount of **coffee**, **water** and **foam**; making sure that coffee and foam are only used by a single consumer at a time; as well as grinding and whipping milk to reach the required amount.

The usage of all resources is emulated as a `sleep`, lineally dependent on the amount of resource used.

Coffee/Beans and Foam/Milk work in an analogous fashion.

Orders are fulfilled in a greedy fashion; if a resource needs to be _transformed_, only the minimum required will be processed.

> When an amount of coffee greater than that in the container is requested, the machine will grind just enough beans to cover the request.
>
> When an amount of foam greater than that in the container is requested, the machine will whip just enough milk to cover the request.

This last decision could result inefficient if the fixed-time for processing was high. Yet, it minimizes waste and makes orders _individually_ faster. Also, this would be a more realistic approach, where ingredients are kept as fresh as possible.

In order to be able to monitor resources even when they are being used, a `ResourceMonitor` is used to keep track of the current amount of resources in a duplicate set of fields that gets updated after the their usage.

A friendly UI is provided to monitor the resources in real time.
The data is refreshed at an interval, marking low resources accordingly.

```
Coffee: 100 mg
Coffee Beans: 8180 mg
Foam: 100 ml
Milk: 150 ml [WARNING: below threshold]
```

### Dispensers

`dispenser`

A simple interface is provided to consume orders and use resources to fulfill them.
Its implementation is trivial as most logic is handled by `Resources`.

An interesting aspect of its functioning is that when it pops `NoMoreOrders` from the queue, it will put it back and stop; so that other dispensers can also stop.

If an order fails to be fulfilled, it will be ignored, as it is assumed that the machine's resources are enough to fulfill all the orders.

## Development

This project was bootstrapped with [Cargo](https://doc.rust-lang.org/cargo/), the [Rust](https://www.rust-lang.org/) package manager.

Some commands you might find useful:

- `cargo test` - Run the tests.
- `cargo build` - Compile the project.
- `cargo run` - Build and Run the project.
