use std::time::Duration;

use num_bigint::BigUint;
use yew::{
    prelude::*,
    services::{interval::IntervalTask, IntervalService},
};

mod building;

use building::Building;

const SHEKEL: &str = "₪";
const SHEKELS_PER_SECOND: &str = " ₪/s";

enum Msg {
    ShekelClicked,
    Tick,
    BuildingBought(usize, BigUint),
}

struct Model {
    link: ComponentLink<Self>,
    shekel_count: BigUint,
    building_types: Vec<Building>,
    #[allow(dead_code)]
    interval_task: IntervalTask,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let interval_task =
            IntervalService::spawn(Duration::new(1, 0), link.callback(|_| Msg::Tick));

        Self {
            link,
            shekel_count: BigUint::from(0u32),
            building_types: vec![
                Building::new(
                    "Yewish House".to_string(),
                    BigUint::from(1u32),
                    BigUint::from(50u32),
                ),
                Building::new(
                    "Yewish Commune".to_string(),
                    BigUint::from(10u32),
                    BigUint::from(400u32),
                ),
                Building::new(
                    "Yewish Town".to_string(),
                    BigUint::from(100u32),
                    BigUint::from(3000u32),
                ),
            ],
            interval_task,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use Msg::*;

        match msg {
            ShekelClicked => {
                self.shekel_count = &self.shekel_count + 1u32;
                true
            }
            Tick => {
                for building in &self.building_types {
                    self.shekel_count = &self.shekel_count + building.calculate_income();
                }

                true
            }
            BuildingBought(index, count) => {
                let building = &mut self.building_types[index];
                let mut i = BigUint::from(0u32);

                while i < count && self.shekel_count >= building.cost {
                    self.shekel_count = &self.shekel_count - &building.cost;
                    building.count = &building.count + BigUint::from(1u32);
                    building.adjust_cost();
                    i = &i + BigUint::from(1u32);
                }

                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <div class="shekel-container">
                    <button onclick=self.link.callback(|_| Msg::ShekelClicked)
                            class="shekel-button">
                        <img draggable="false" style="width: 100%" src="/shekel.png"/>
                    </button>
                    <div class="shekel-count">{ self.shekel_count.to_string() } { SHEKEL }</div>
                </div>
                <div>
                    <h1>{ "Statistics" }</h1>
                    <div>{ "Income: " } { self.calculate_total_income() } { SHEKELS_PER_SECOND }</div>
                    <h1>{ "Buildings" }</h1>
                    <table class="upgrades">
                        { for self.building_types.iter().enumerate().map(|(index, building)| {
                            self.view_building_upgrade(index, building)
                        }) }
                    </table>
                </div>
            </div>
        }
    }
}

impl Model {
    fn view_building_upgrade(&self, index: usize, building: &Building) -> Html {
        let buy = move |count: BigUint| {
            self.link
                .callback(move |_| Msg::BuildingBought(index, count.clone()))
        };

        html! {
            <tr>
                <td>{ building.name.to_string() }</td>
                <td>{ "Cost: " } { building.cost.to_string() } { SHEKEL }</td>
                <td class="center-text">
                    { "Income: " }
                    { building.shekels_per_second.to_string() }
                    { " " }
                    { SHEKEL }
                    { "/s" }
                </td>
                <td>{ "Owned: " } { building.count.to_string() }</td>
                <td class="center-text">
                    { "Total: " }
                    { building.calculate_income().to_string() }
                    { SHEKELS_PER_SECOND }
                </td>
                { for [1u32, 10, 50, 100, 500].iter().map(|count| {
                    let count = BigUint::from(*count);

                    html! {
                        <td>
                            <button class="buy-button" onclick=buy(count.clone())>
                                { count.to_string() }
                            </button>
                        </td>
                    }
                }) }
            </tr>
        }
    }

    fn calculate_total_income(&self) -> BigUint {
        let mut income = BigUint::from(0u32);

        for building in &self.building_types {
            income = &income + building.calculate_income();
        }

        income
    }
}

fn main() {
    yew::start_app::<Model>();
}
