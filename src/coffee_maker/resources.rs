use std::{
    sync::{Arc, Mutex, MutexGuard},
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

const GRIND_COFFEE_FIXED_TIME: u32 = 4;
const GRIND_COFFEE_TIME_PER_MG: u32 = 2;

const WIP_MILK_FIXED_TIME: u32 = 4;
const WIP_MILK_TIME_PER_MG: u32 = 2;

#[derive(Debug, Clone)]
pub enum Error {
    InsufficientResources,
}

pub type ResourceResult = Result<(), Error>;

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

    fn grind_needed_coffee_beans<'cof>(
        mut coffee: MutexGuard<'cof, u32>,
        mut coffee_beans: MutexGuard<u32>,
        amount: u32,
    ) -> Result<MutexGuard<'cof, u32>, Error> {
        let needed = amount as i64 - *coffee as i64;
        if needed > *coffee_beans as i64 {
            Err(Error::InsufficientResources)
        } else if needed > 0 {
            let duration = needed as u32 * GRIND_COFFEE_TIME_PER_MG + GRIND_COFFEE_FIXED_TIME;
            thread::sleep(std::time::Duration::from_millis(duration.into()));
            *coffee_beans -= needed as u32;
            *coffee += needed as u32;
            Ok(coffee)
        } else {
            Ok(coffee)
        }
    }

    pub fn use_coffee(&self, amount: u32) -> ResourceResult {
        let coffee = self.coffee.lock().expect("Failed to lock coffee");
        let coffee_beans = self
            .coffee_beans
            .lock()
            .expect("Failed to lock coffee beans");

        let mut coffee = Self::grind_needed_coffee_beans(coffee, coffee_beans, amount)?;

        let duration = amount * COFFEE_TIME_PER_MG + COFFEE_FIXED_TIME;
        thread::sleep(std::time::Duration::from_millis(duration.into()));
        *coffee -= amount;
        Ok(())
    }

    pub fn use_water(&self, amount: u32) -> ResourceResult {
        let duration = amount * WATER_TIME_PER_ML + WATER_FIXED_TIME;
        thread::sleep(std::time::Duration::from_millis(duration.into()));
        Ok(())
    }

    fn wip_needed_foam<'cof>(
        mut foam: MutexGuard<'cof, u32>,
        mut milk: MutexGuard<u32>,
        amount: u32,
    ) -> Result<MutexGuard<'cof, u32>, Error> {
        let needed = amount as i64 - *foam as i64;
        if needed > *milk as i64 {
            Err(Error::InsufficientResources)
        } else if needed > 0 {
            let duration = needed as u32 * WIP_MILK_TIME_PER_MG + WIP_MILK_FIXED_TIME;
            thread::sleep(std::time::Duration::from_millis(duration.into()));
            *milk -= needed as u32;
            *foam += needed as u32;
            Ok(foam)
        } else {
            Ok(foam)
        }
    }

    pub fn use_foam(&self, amount: u32) -> ResourceResult {
        let foam = self.foam.lock().expect("Failed to lock foam");
        let milk = self.milk.lock().expect("Failed to lock milk");

        let mut foam = Self::wip_needed_foam(foam, milk, amount)?;

        let duration = amount * FOAM_TIME_PER_ML + FOAM_FIXED_TIME;
        thread::sleep(std::time::Duration::from_millis(duration.into()));
        *foam -= amount;
        Ok(())
    }
}
