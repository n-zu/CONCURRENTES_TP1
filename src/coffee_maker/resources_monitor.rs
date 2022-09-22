use super::config;

pub struct ResourcesMonitor {
    coffee: u32,
    coffee_beans: u32,
    foam: u32,
    milk: u32,
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
        }
    }

    pub fn update_coffee(&mut self, coffee: u32) {
        self.coffee = coffee;
    }
    pub fn update_coffee_beans(&mut self, coffee_beans: u32) {
        if coffee_beans < config::G * config::X / 100 {
            println!("Warning: coffee beans are running low");
        }
        self.coffee_beans = coffee_beans;
    }

    pub fn update_foam(&mut self, foam: u32) {
        self.foam = foam;
    }
    pub fn update_milk(&mut self, milk: u32) {
        if milk < config::L * config::X / 100 {
            println!("Warning: milk is running low");
        }
        self.milk = milk;
    }
}
