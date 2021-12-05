use std::time::Duration;

use lazy_static::lazy_static;
use num_bigint::BigUint;
use upgrade::Upgrade;
use yew::{
    format::Json,
    prelude::*,
    services::{interval::IntervalTask, storage::Area, IntervalService, StorageService},
};

mod building;
mod item;
mod state;
mod upgrade;

use building::Building;
use state::State;

pub const SHEKEL: &str = "₪";
pub const SHEKELS_PER_SECOND: &str = " ₪/s";

const BUY_COUNT: [u32; 5] = [1, 10, 50, 100, 500];

lazy_static! {
    pub static ref TAX_RATE: BigUint = BigUint::from(10u32);
    pub static ref CLICK_BASE: BigUint = BigUint::from(2u32);
}

enum Msg {
    ClickShekel,
    Tick,
    Save,
    ResetSave,
    Hack,
    BuildingBought(usize, BigUint),
    UpgradeTaxation(BigUint),
    UpgradeThievery(BigUint),
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
            ClickShekel => {
                self.state.shekel_count =
                    &self.state.shekel_count + self.calculate_shekels_per_click();

                true
            }
            Tick => {
                for building in &self.state.building_types {
                    self.state.shekel_count =
                        &self.state.shekel_count + self.calculate_building_income(building);
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
            Hack => {
                self.state.shekel_count = &self.state.shekel_count + 100000000u32;
                true
            }
            BuildingBought(index, count) => {
                let mut building = self.state.building_types[index].clone();
                self.state.purchase(&mut building, count);
                self.state.building_types[index] = building;

                true
            }
            UpgradeTaxation(count) => {
                let mut upgrade = self.state.taxation_upgrade.clone();
                self.state.purchase(&mut upgrade, count);
                self.state.taxation_upgrade = upgrade;

                true
            }
            UpgradeThievery(count) => {
                let mut upgrade = self.state.thievery_upgrade.clone();
                self.state.purchase(&mut upgrade, count);
                self.state.thievery_upgrade = upgrade;

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
                    <button onclick=self.link.callback(|_| Msg::ClickShekel)
                            class="shekel-button">
                        <img draggable="false" style="width: 100%" src="/shekel.png"/>
                    </button>
                    <div class="shekel-count">{ self.state.shekel_count.to_string() } { SHEKEL }</div>
                </div>
                <div>
                    <h1>{ "Statistics" }</h1>
                    <div>{ "Income: " } { self.calculate_total_income() } { SHEKELS_PER_SECOND }</div>
                    <div>{ self.calculate_shekels_per_click() } { SHEKEL } { " per click" }</div>
                    <div class="buttons">
                        <button onclick=self.link.callback(|_| Msg::Save)>{ "Save" }</button>
                        <button onclick=self.link.callback(|_| Msg::ResetSave)>{ "Reset" }</button>
                        <button onclick=self.link.callback(|_| Msg::Hack)>{ "Hack" }</button>
                    </div>
                    <h1>{ "Upgrades" }</h1>
                    <table class="upgrades">
                        { self.view_upgrade(&self.state.taxation_upgrade, |count| Msg::UpgradeTaxation(count)) }
                        { self.view_upgrade(&self.state.thievery_upgrade, |count| Msg::UpgradeThievery(count)) }
                    </table>
                    <h1>{ "Buildings" }</h1>
                    <table class="buildings">
                        { for self.state.building_types.iter().enumerate().map(|(index, building)| {
                            self.view_building(index, building)
                        }) }
                    </table>
                </div>
            </div>
        }
    }
}

impl Model {
    fn view_building(&self, index: usize, building: &Building) -> Html {
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
                <td>{ "Owned: " } { building.level.to_string() }</td>
                <td>
                    { "Total: " }
                    { self.calculate_building_income(building).to_string() }
                    { SHEKELS_PER_SECOND }
                </td>
                { for BUY_COUNT.iter().map(|count| {
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

    fn view_upgrade<T>(&self, upgrade: &Upgrade, upgrade_message_provider: T) -> Html
    where
        T: Fn(BigUint) -> Msg + Copy + 'static,
    {
        html! {
            <tr>
                <td>{ upgrade.name.to_string() }</td>
                <td>{ "Level: " } { upgrade.level.to_string() }</td>
                <td>{ "Cost: " } { upgrade.cost.to_string() } { SHEKEL }</td>
                <td>{ upgrade.description.to_string() }</td>
                { for BUY_COUNT.iter().map(|count| {
                    let count = BigUint::from(*count);

                    let buy =
                        move |count: BigUint| self.link.callback(move |_| upgrade_message_provider(count.clone()));

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

    fn calculate_shekels_per_click(&self) -> BigUint {
        self.state.thievery_upgrade.get_value()
    }

    fn calculate_building_income(&self, building: &Building) -> BigUint {
        let income = building.calculate_income();
        &income + &income * self.state.taxation_upgrade.get_value() / 100u32
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
            income = &income + self.calculate_building_income(building);
        }

        income
    }
}

fn main() {
    yew::start_app::<Model>();
}
