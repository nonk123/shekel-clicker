use gloo_storage::{LocalStorage, Storage};
use gloo_timers::callback::Interval;
use lazy_static::lazy_static;
use num_bigint::BigUint;
use yew::prelude::*;

mod building;
mod item;
mod state;
mod upgrade;

use building::Building;
use state::State;
use upgrade::Upgrade;

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
    state: State,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();
        Interval::new(1_000, move || link.send_message(Msg::Tick)).forget();

        let link = ctx.link().clone();
        Interval::new(20_000, move || link.send_message(Msg::Save)).forget();

        let state = Self::load_save();

        Self { state }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
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

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <div class="shekel-container">
                    <button
                        onclick={ ctx.link().callback(|_| Msg::ClickShekel) }
                        class="shekel-button"
                    >
                        <img draggable="false" style="width: 100%" src="/shekel.png"
                             alt="The mother-shekel. Click it!"/>
                    </button>
                    <div class="shekel-count">{ self.state.shekel_count.to_string() } { SHEKEL }</div>
                </div>
                <div>
                    <h1>{ "Statistics" }</h1>
                    <div>{ "Income: " } { self.calculate_total_income() } { SHEKELS_PER_SECOND }</div>
                    <div>{ self.calculate_shekels_per_click() } { SHEKEL } { " per click" }</div>
                    <div class="buttons">
                        <button onclick={ ctx.link().callback(|_| Msg::Save) }>{ "Save" }</button>
                        <button onclick={ ctx.link().callback(|_| Msg::ResetSave) }>{ "Reset" }</button>
                        <button onclick={ ctx.link().callback(|_| Msg::Hack) }>{ "Hack" }</button>
                    </div>
                    <h1>{ "Upgrades" }</h1>
                    <table class="upgrades">
                        { self.view_upgrade(ctx, &self.state.taxation_upgrade, |count| Msg::UpgradeTaxation(count)) }
                        { self.view_upgrade(ctx, &self.state.thievery_upgrade, |count| Msg::UpgradeThievery(count)) }
                    </table>
                    <h1>{ "Buildings" }</h1>
                    <table class="buildings">
                        { for self.state.building_types.iter().enumerate().map(|(index, building)| {
                            self.view_building(ctx, index, building)
                        }) }
                    </table>
                </div>
            </div>
        }
    }
}

impl Model {
    fn view_building(&self, ctx: &Context<Self>, index: usize, building: &Building) -> Html {
        let buy = move |count: BigUint| {
            ctx.link()
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
                            <button class="buy-button" onclick={ buy(count.clone()) }>
                                { count.to_string() }
                            </button>
                        </td>
                    }
                }) }
            </tr>
        }
    }

    fn view_upgrade<T>(
        &self,
        ctx: &Context<Self>,
        upgrade: &Upgrade,
        upgrade_message_provider: T,
    ) -> Html
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
                        move |count: BigUint| ctx.link().callback(move |_| upgrade_message_provider(count.clone()));

                    html! {
                        <td>
                            <button class="buy-button" onclick={ buy(count.clone()) }>
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
        LocalStorage::set("save", &self.state).unwrap();
    }

    fn load_save() -> State {
        let result: Result<State, _> = LocalStorage::get("save");

        match result {
            Ok(result) => result,
            Err(_) => {
                let save = State::new();
                LocalStorage::set("save", &save).unwrap();
                save
            }
        }
    }

    fn reset_save(&mut self) {
        LocalStorage::delete("save");
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
