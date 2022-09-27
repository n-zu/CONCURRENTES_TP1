use std::{
    sync::{atomic::AtomicBool, Arc, Mutex, MutexGuard},
    thread::{self, JoinHandle},
};

use super::{
    config,
    resources_monitor::{monitor_resources, ResourcesMonitor},
};

const SPEED: u32 = 10;

const COFFEE_FIXED_TIME: u32 = 4 / SPEED;
const COFFEE_TIME_PER_MG: u32 = 30 / SPEED;

const WATER_FIXED_TIME: u32 = 10 / SPEED;
const WATER_TIME_PER_ML: u32 = 4 / SPEED;

const FOAM_FIXED_TIME: u32 = 4 / SPEED;
const FOAM_TIME_PER_ML: u32 = 20 / SPEED;

const GRIND_COFFEE_FIXED_TIME: u32 = 4 / SPEED;
const GRIND_COFFEE_TIME_PER_MG: u32 = 50 / SPEED;

const WIP_MILK_FIXED_TIME: u32 = 4 / SPEED;
const WIP_MILK_TIME_PER_MG: u32 = 60 / SPEED;

/// Resource Errors
#[derive(Debug, Clone)]
pub enum Error {
    InsufficientResources,
}

/// Result Wrapper for Resource
pub type ResourceResult = Result<(), Error>;

/// Stores the available resources and a monitor to keep track of them.
/// Can be used thread-safely.
pub struct Resources {
    coffee: Mutex<u32>,
    coffee_beans: Mutex<u32>,
    foam: Mutex<u32>,
    milk: Mutex<u32>,
    monitor: Arc<Mutex<ResourcesMonitor>>,
}

impl Resources {
    /// Creates a new Resources instance.
    pub fn new(
        coffee: u32,
        coffee_beans: u32,
        foam: u32,
        milk: u32,
    ) -> Result<Arc<Resources>, String> {
        if coffee > config::C {
            Err("Coffee is too much".to_string())
        } else if coffee_beans > config::G {
            Err("Coffee beans is too much".to_string())
        } else if foam > config::E {
            Err("Foam is too much".to_string())
        } else if milk > config::L {
            Err("Milk is too much".to_string())
        } else {
            Ok(Arc::new(Resources {
                coffee: Mutex::new(coffee),
                coffee_beans: Mutex::new(coffee_beans),
                foam: Mutex::new(foam),
                milk: Mutex::new(milk),
                monitor: Arc::new(Mutex::new(ResourcesMonitor::new(
                    coffee,
                    coffee_beans,
                    foam,
                    milk,
                ))),
            }))
        }
    }

    /// Transforms the required amount of coffee_beans into coffee.
    /// Takes time according to the amount.
    fn grind_needed_coffee_beans<'cof>(
        mut coffee: MutexGuard<'cof, u32>,
        mut coffee_beans: MutexGuard<u32>,
        amount: u32,
        monitor: &Mutex<ResourcesMonitor>,
    ) -> Result<MutexGuard<'cof, u32>, Error> {
        let needed = amount as i64 - *coffee as i64;
        if needed > *coffee_beans as i64 {
            Err(Error::InsufficientResources)
        } else if needed > 0 {
            let duration = needed as u32 * GRIND_COFFEE_TIME_PER_MG + GRIND_COFFEE_FIXED_TIME;
            thread::sleep(std::time::Duration::from_millis(duration.into()));
            *coffee_beans -= needed as u32;
            *coffee += needed as u32;

            let mut monitor = monitor.lock().unwrap();
            monitor.update_coffee(*coffee);
            monitor.update_coffee_beans(*coffee_beans);
            Ok(coffee)
        } else {
            Ok(coffee)
        }
    }

    /// Reduces the required amount of coffee.
    /// Takes time according to the amount.
    pub fn use_coffee(&self, amount: u32) -> ResourceResult {
        let coffee = self.coffee.lock().expect("Failed to lock coffee");
        let coffee_beans = self
            .coffee_beans
            .lock()
            .expect("Failed to lock coffee beans");

        let mut coffee =
            Self::grind_needed_coffee_beans(coffee, coffee_beans, amount, &self.monitor)?;

        let duration = amount * COFFEE_TIME_PER_MG + COFFEE_FIXED_TIME;
        thread::sleep(std::time::Duration::from_millis(duration.into()));
        *coffee -= amount;

        let mut monitor = self.monitor.lock().expect("Failed to lock monitor");
        monitor.update_coffee(*coffee);

        Ok(())
    }

    /// Simulates using the required amount of water.
    /// Takes time according to the amount.
    pub fn use_water(&self, amount: u32) -> ResourceResult {
        let duration = amount * WATER_TIME_PER_ML + WATER_FIXED_TIME;
        thread::sleep(std::time::Duration::from_millis(duration.into()));
        Ok(())
    }

    /// Transforms the required amount of milk into foam.
    /// Takes time according to the amount.
    fn wip_needed_foam<'cof>(
        mut foam: MutexGuard<'cof, u32>,
        mut milk: MutexGuard<u32>,
        amount: u32,
        monitor: &Mutex<ResourcesMonitor>,
    ) -> Result<MutexGuard<'cof, u32>, Error> {
        let needed = amount as i64 - *foam as i64;
        if needed > *milk as i64 {
            Err(Error::InsufficientResources)
        } else if needed > 0 {
            let duration = needed as u32 * WIP_MILK_TIME_PER_MG + WIP_MILK_FIXED_TIME;
            thread::sleep(std::time::Duration::from_millis(duration.into()));
            *milk -= needed as u32;
            *foam += needed as u32;

            let mut monitor = monitor.lock().unwrap();
            monitor.update_foam(*foam);
            monitor.update_milk(*milk);

            Ok(foam)
        } else {
            Ok(foam)
        }
    }

    /// Reduces the required amount of foam.
    /// Takes time according to the amount.
    pub fn use_foam(&self, amount: u32) -> ResourceResult {
        let foam = self.foam.lock().expect("Failed to lock foam");
        let milk = self.milk.lock().expect("Failed to lock milk");

        let mut foam = Self::wip_needed_foam(foam, milk, amount, &self.monitor)?;

        let duration = amount * FOAM_TIME_PER_ML + FOAM_FIXED_TIME;
        thread::sleep(std::time::Duration::from_millis(duration.into()));
        *foam -= amount;

        let mut monitor = self.monitor.lock().expect("Failed to lock monitor");
        monitor.update_foam(*foam);

        Ok(())
    }

    /// Starts the monitor. this will print the current resources at an interval.
    /// Returns a handle to the monitor thread and an AtomicBool to stop it.
    /// The AtomicBool is set to true when the monitor is stopped.
    pub fn monitor(&self, interval_millis: u64) -> (JoinHandle<()>, Arc<AtomicBool>) {
        let monitor = self.monitor.clone();
        monitor_resources(monitor, interval_millis)
    }
}
