use std::time::Duration;

use num_bigint::BigUint;
use yew::{
    prelude::*,
    services::{interval::IntervalTask, IntervalService},
};

mod building;

use building::Building;

const SHEKEL: &str = "₪";

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
                    BigUint::from(500u32),
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
            <div style="display: flex;">
                <div style="width: 50%;">
                    <button onclick=self.link.callback(|_| Msg::ShekelClicked)
                            class="shekel-button">
                        <img draggable="false" style="width: 100%" src="/shekel.png"/>
                    </button>
                    <div class="shekel-count">{ self.shekel_count.to_string() } { SHEKEL }</div>
                </div>
                <div class="upgrades">
                    { for self.building_types.iter().enumerate().map(|(index, building)| {
                        let buy = move |count: BigUint| {
                            self.link
                                .callback(move |_| Msg::BuildingBought(index, count.clone()))
                        };

                        html! {
                            <div class="upgrade">
                                <div>
                                    <div>{ building.name.to_string() }</div>
                                    <div>
                                        { "Owned: " }
                                        { building.count.to_string() }
                                    </div>
                                    <div>
                                        { "Cost: " }
                                        { building.cost.to_string() }
                                        { SHEKEL }
                                    </div>
                                </div>
                                <div>
                                    <div class="center-text">
                                        { "Income: " }
                                        { building.shekels_per_second.to_string() }
                                        { " " }
                                        { SHEKEL }
                                        { "/s" }
                                    </div>
                                    <div class="center-text">
                                        { "Total: " }
                                        { building.calculate_income().to_string() }
                                        { " " }
                                        { SHEKEL }
                                        { "/s" }
                                    </div>
                                </div>
                                { for [1u32, 10, 50, 100, 500].iter().map(|count| {
                                    let count = BigUint::from(*count);

                                    html! {
                                        <button class="buy-button" onclick=buy(count.clone())>
                                            { count.to_string() }
                                        </button>
                                    }
                                }) }
                            </div>
                        }
                    }) }
                </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
