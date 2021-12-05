use std::time::Duration;

use num_bigint::BigUint;
use serde::{Deserialize, Serialize};
use yew::{
    format::Json,
    prelude::*,
    services::{interval::IntervalTask, storage::Area, IntervalService, StorageService},
};

mod building;

use building::Building;

pub const SHEKEL: &str = "₪";
pub const SHEKELS_PER_SECOND: &str = " ₪/s";

#[derive(Serialize, Deserialize)]
struct State {
    shekel_count: BigUint,
    building_types: Vec<Building>,
}

impl State {
    pub fn new() -> Self {
        Self {
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
                    "Yewish Village".to_string(),
                    BigUint::from(100u32),
                    BigUint::from(3000u32),
                ),
                Building::new(
                    "Yewish Town".to_string(),
                    BigUint::from(1000u32),
                    BigUint::from(20000u32),
                ),
                Building::new(
                    "Yewish City".to_string(),
                    BigUint::from(10000u32),
                    BigUint::from(100000u32),
                ),
            ],
        }
    }
}

enum Msg {
    ShekelClicked,
    Tick,
    Save,
    ResetSave,
    BuildingBought(usize, BigUint),
}

struct Model {
    link: ComponentLink<Self>,
    storage: StorageService,
    state: State,
    #[allow(dead_code)]
    tick_task: IntervalTask,
    #[allow(dead_code)]
    save_task: IntervalTask,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let tick_task = IntervalService::spawn(Duration::new(1, 0), link.callback(|_| Msg::Tick));
        let save_task = IntervalService::spawn(Duration::new(20, 0), link.callback(|_| Msg::Save));

        let mut storage = StorageService::new(Area::Local).expect("local storage");
        let state = Self::load_save(&mut storage);

        Self {
            link,
            storage,
            state,
            tick_task,
            save_task,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use Msg::*;

        match msg {
            ShekelClicked => {
                self.state.shekel_count = &self.state.shekel_count + 1u32;
                true
            }
            Tick => {
                for building in &self.state.building_types {
                    self.state.shekel_count =
                        &self.state.shekel_count + building.calculate_income();
                }

                true
            }
            Save => {
                self.save();
                false
            }
            ResetSave => {
                self.reset_save();
                true
            }
            BuildingBought(index, count) => {
                let building = &mut self.state.building_types[index];
                let mut i = BigUint::from(0u32);

                while i < count && self.state.shekel_count >= building.cost {
                    self.state.shekel_count = &self.state.shekel_count - &building.cost;
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
                    <div class="shekel-count">{ self.state.shekel_count.to_string() } { SHEKEL }</div>
                </div>
                <div>
                    <h1>{ "Statistics" }</h1>
                    <div>{ "Income: " } { self.calculate_total_income() } { SHEKELS_PER_SECOND }</div>
                    <div class="buttons">
                        <button onclick=self.link.callback(|_| Msg::Save)>{ "Save" }</button>
                        <button onclick=self.link.callback(|_| Msg::ResetSave)>{ "Reset" }</button>
                    </div>
                    <h1>{ "Buildings" }</h1>
                    <table class="upgrades">
                        { for self.state.building_types.iter().enumerate().map(|(index, building)| {
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
                <td>
                    { "Income: " }
                    { building.shekels_per_second.to_string() }
                    { " " }
                    { SHEKEL }
                    { "/s" }
                </td>
                <td>{ "Owned: " } { building.count.to_string() }</td>
                <td>
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

    fn save(&mut self) {
        self.storage.store("save", Json(&self.state));
    }

    fn load_save(storage: &mut StorageService) -> State {
        let result: Result<String, _> = storage.restore("save");

        if result.is_ok() {
            serde_json::from_str(&result.unwrap()).expect("save")
        } else {
            let save = State::new();
            storage.store("save", Json(&save));
            save
        }
    }

    fn reset_save(&mut self) {
        self.storage.remove("save");
        self.state = State::new();
    }

    fn calculate_total_income(&self) -> BigUint {
        let mut income = BigUint::from(0u32);

        for building in &self.state.building_types {
            income = &income + building.calculate_income();
        }

        income
    }
}

fn main() {
    yew::start_app::<Model>();
}
