use num_bigint::BigUint;

use crate::item_struct;

item_struct! {
    Building {
        {
            name: String,
            shekels_per_second: BigUint,
        }

        fn adjust_cost(&mut self) {
            self.cost = &self.cost + &self.cost / 2u32;
        }
    }
}

impl Building {
    pub fn new(name: &str, shekels_per_second: BigUint, cost: BigUint) -> Self {
        Self {
            name: name.to_string(),
            shekels_per_second,
            level: 0u32.into(),
            cost,
        }
    }

    pub fn calculate_income(&self) -> BigUint {
        &self.shekels_per_second * &self.level
    }
}
