use std::{
    fmt,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

use super::config;

#[derive(Debug)]
pub struct ResourcesMonitor {
    coffee: u32,
    coffee_beans: u32,
    foam: u32,
    milk: u32,
    low_on_coffee_beans: bool,
    low_on_milk: bool,
}

impl ResourcesMonitor {
    pub fn new(
        initial_coffee: u32,
        initial_coffee_beans: u32,
        initial_foam: u32,
        initial_milk: u32,
    ) -> ResourcesMonitor {
        ResourcesMonitor {
            coffee: initial_coffee,
            coffee_beans: initial_coffee_beans,
            foam: initial_foam,
            milk: initial_milk,
            low_on_coffee_beans: false,
            low_on_milk: false,
        }
    }

    pub fn update_coffee(&mut self, coffee: u32) {
        self.coffee = coffee;
    }
    pub fn update_coffee_beans(&mut self, coffee_beans: u32) {
        if coffee_beans < config::G * config::X / 100 {
            self.low_on_coffee_beans = true;
        }
        self.coffee_beans = coffee_beans;
    }

    pub fn update_foam(&mut self, foam: u32) {
        self.foam = foam;
    }
    pub fn update_milk(&mut self, milk: u32) {
        if milk < config::L * config::X / 100 {
            self.low_on_milk = true;
        }
        self.milk = milk;
    }
}

impl fmt::Display for ResourcesMonitor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let coffee_beans_warning = match self.low_on_coffee_beans {
            true => "[WARNING: below threshold]",
            false => "",
        };
        let milk_warning = match self.low_on_milk {
            true => "[WARNING: below threshold]",
            false => "",
        };
        let info = format!(
            "Coffee: {} mg\nCoffee Beans: {} mg {}\nFoam: {} ml\nMilk: {} ml {}",
            self.coffee,
            self.coffee_beans,
            coffee_beans_warning,
            self.foam,
            self.milk,
            milk_warning
        );

        write!(f, "{}\n", info)
    }
}

pub fn monitor_resources(
    monitor: Arc<Mutex<ResourcesMonitor>>,
    millis: u64,
) -> (JoinHandle<()>, Arc<AtomicBool>) {
    let stop = Arc::new(AtomicBool::new(false));
    let stop_thread = stop.clone();

    let handle = thread::spawn(move || loop {
        thread::sleep(std::time::Duration::from_millis(millis));

        print!("{}[2J", 27 as char); // clear screen
        println!("{}", monitor.lock().unwrap());

        if stop_thread.load(Ordering::Relaxed) {
            break;
        };
    });

    (handle, stop)
}
