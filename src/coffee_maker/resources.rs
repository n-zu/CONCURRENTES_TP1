use std::{
    sync::{Arc, Mutex},
    thread,
};

const INITIAL_COFFEE: u32 = 1000;
const INITIAL_COFFEE_BEANS: u32 = 10000;
const INITIAL_FOAM: u32 = 1000;
const INITIAL_MILK: u32 = 10000;

const COFFEE_FIXED_TIME: u32 = 4;
const COFFEE_TIME_PER_MG: u32 = 2;

const WATER_FIXED_TIME: u32 = 2;
const WATER_TIME_PER_ML: u32 = 1;

const FOAM_FIXED_TIME: u32 = 5;
const FOAM_TIME_PER_ML: u32 = 5;

#[derive(Debug, Clone)]
pub enum Error {
    InsufficientResources,
}

type ResourceResult = Result<(), Error>;

pub struct Resources {
    coffee: Mutex<u32>,
    coffee_beans: Mutex<u32>,
    foam: Mutex<u32>,
    milk: Mutex<u32>,
}

impl Resources {
    pub fn new() -> Arc<Resources> {
        Arc::new(Resources {
            coffee: Mutex::new(INITIAL_COFFEE),
            coffee_beans: Mutex::new(INITIAL_COFFEE_BEANS),
            foam: Mutex::new(INITIAL_FOAM),
            milk: Mutex::new(INITIAL_MILK),
        })
    }

    pub fn get_coffee(&self, amount: u32) -> ResourceResult {
        let coffee = self.coffee.lock().unwrap();
        let coffee_beans = self.coffee_beans.lock().unwrap();

        if amount > *coffee + *coffee_beans {
            return Err(Error::InsufficientResources);
        }

        let duration = amount * COFFEE_TIME_PER_MG + COFFEE_FIXED_TIME;
        thread::sleep(std::time::Duration::from_millis(duration.into()));
        Ok(())
    }

    pub fn get_water(&self, amount: u32) -> ResourceResult {
        let duration = amount * WATER_TIME_PER_ML + WATER_FIXED_TIME;
        thread::sleep(std::time::Duration::from_millis(duration.into()));
        Ok(())
    }

    pub fn get_foam(&self, amount: u32) -> ResourceResult {
        let foam = self.foam.lock().unwrap();
        let milk = self.milk.lock().unwrap();

        if amount > *foam + *milk {
            return Err(Error::InsufficientResources);
        }

        let duration = amount * FOAM_TIME_PER_ML + FOAM_FIXED_TIME;
        thread::sleep(std::time::Duration::from_millis(duration.into()));
        Ok(())
    }
}
