use num_bigint::BigUint;
use serde::{Deserialize, Serialize};

use crate::{item::Item, item_struct};

item_struct! {
    Upgrade {
        {
            name: String,
            description: String,
            kind: UpgradeKind,
        }

        fn adjust_cost(&mut self) {
            match &self.kind {
                UpgradeKind::Power(base) => self.cost = &self.cost * base * 2u32,
                UpgradeKind::Multiplier(_) => self.cost = &self.cost + &self.cost / 2u32,
            };
        }
    }
}

impl Upgrade {
    pub fn new(name: &str, description: &str, cost: BigUint, kind: UpgradeKind) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            level: 0u32.into(),
            cost,
            kind,
        }
    }

    pub fn get_value(&self) -> BigUint {
        match &self.kind {
            UpgradeKind::Power(base) => base.pow(self.level().try_into().unwrap()),
            UpgradeKind::Multiplier(operand) => {
                if self.level == 0u32.into() {
                    1u32.into()
                } else {
                    operand * self.level()
                }
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum UpgradeKind {
    Power(BigUint),
    Multiplier(BigUint),
}
