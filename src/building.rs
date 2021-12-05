use num_bigint::BigUint;

pub struct Building {
    pub name: String,
    pub shekels_per_second: BigUint,
    pub count: BigUint,
    pub cost: BigUint,
}

impl Building {
    pub fn new(name: String, shekels_per_second: BigUint, cost: BigUint) -> Self {
        Self {
            name,
            shekels_per_second,
            count: BigUint::from(0u32),
            cost,
        }
    }

    pub fn calculate_income(&self) -> BigUint {
        &self.shekels_per_second * &self.count
    }

    pub fn adjust_cost(&mut self) {
        self.cost = &self.cost + &self.cost / BigUint::from(2u32);
    }
}
