# Internet of Coffee

![Coffee Maker](/assets/coffee_maker.gif)

`assets/orders.csv`

## Design

- order: coffee , water , foam
- we asume orders wont require more than the available resources, but it's not checked nor handled in any way . ( maybe say its required as ... )
- order taker : single producer - multiple consumer , concurrent
- resources :
  - coffee/foam have locked access ( only one at a time )
  - will produce only what is needed
  - as we asume a cup does not surpass the container, we wont need multiple grinding / wiping
- output : clears/refreshs screen to update data , friendly realtime UI, rather than logging

## Development

This project was bootstrapped with [Cargo](https://doc.rust-lang.org/cargo/), the Rust package manager.

Some commands you might find useful:

- `cargo test` - Run the tests.
- `cargo build` - Compile the project.
- `cargo run` - Build and Run the project.
