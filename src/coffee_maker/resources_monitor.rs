use std::{
    fmt,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

use super::config;

/// Stores a copy of the resources to keep track of them as well as status flags.
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
    /// Creates a new ResourcesMonitor instance.
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

    /// Updates the amount of coffee.
    pub fn update_coffee(&mut self, coffee: u32) {
        self.coffee = coffee;
    }
    /// Updates the amount of coffee beans.
    pub fn update_coffee_beans(&mut self, coffee_beans: u32) {
        if coffee_beans < config::G * config::X / 100 {
            self.low_on_coffee_beans = true;
        }
        self.coffee_beans = coffee_beans;
    }

    /// Updates the amount of foam.
    pub fn update_foam(&mut self, foam: u32) {
        self.foam = foam;
    }
    /// Updates the amount of milk.
    pub fn update_milk(&mut self, milk: u32) {
        if milk < config::L * config::X / 100 {
            self.low_on_milk = true;
        }
        self.milk = milk;
    }
}

impl fmt::Display for ResourcesMonitor {
    /// Displays the current status of the resources.
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

        writeln!(f, "{}", info)
    }
}

/// Starts the monitor in a new thread.
/// This will print the current resources at an interval.
/// Returns a handle to the monitor thread and an AtomicBool to stop it.
/// The AtomicBool is set to true when the monitor is stopped.
pub fn monitor_resources(
    monitor: Arc<Mutex<ResourcesMonitor>>,
    interval_millis: u64,
) -> (JoinHandle<()>, Arc<AtomicBool>) {
    let stop = Arc::new(AtomicBool::new(false));
    let stop_thread = stop.clone();

    let handle = thread::spawn(move || loop {
        thread::sleep(std::time::Duration::from_millis(interval_millis));

        print!("{}[2J", 27 as char); // clear screen
        println!("{}", monitor.lock().unwrap());

        if stop_thread.load(Ordering::Relaxed) {
            break;
        };
    });

    (handle, stop)
}
