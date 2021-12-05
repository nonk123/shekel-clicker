use num_bigint::BigUint;
use serde::{Deserialize, Serialize};

use crate::{
    building::Building,
    item::Item,
    upgrade::{Upgrade, UpgradeKind},
};

#[derive(Serialize, Deserialize)]
pub struct State {
    pub shekel_count: BigUint,
    pub taxation_upgrade: Upgrade,
    pub thievery_upgrade: Upgrade,
    pub building_types: Vec<Building>,
}

impl State {
    pub fn new() -> Self {
        Self {
            shekel_count: BigUint::from(0u32),
            taxation_upgrade: Upgrade::new(
                "Taxation",
                "Get more shekels from buildings",
                100u32.into(),
                UpgradeKind::Multiplier(10u32.into()),
            ),
            thievery_upgrade: Upgrade::new(
                "Thievery",
                "Get more shekels from clicking",
                100u32.into(),
                UpgradeKind::Power(2u32.into()),
            ),
            building_types: vec![
                Building::new("Yewish House", 1u32.into(), 100u32.into()),
                Building::new("Yewish Commune", 10u32.into(), 800u32.into()),
                Building::new("Yewish Village", 100u32.into(), 16000u32.into()),
                Building::new("Yewish Town", 1000u32.into(), 100000u32.into()),
                Building::new("Yewish City", 10000u32.into(), 8000000u32.into()),
            ],
        }
    }

    pub fn purchase(&mut self, item: &mut impl Item, count: BigUint) {
        let mut i = BigUint::from(0u32);

        while i < count && self.shekel_count >= item.cost() {
            self.shekel_count = &self.shekel_count - &item.cost();
            item.increase_level();
            item.adjust_cost();
            i = &i + 1u32;
        }
    }
}
