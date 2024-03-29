use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use upgrades::{Multiplier, Upgrades};
mod upgrades;

struct Data {
    money: i128,
    base: f64,
    multiplier: f64,
    additive: f64,
    interval: f32,
}
struct UpgradesData {
    upgrades: Vec<Upgrades>,
}
impl Data {
    fn calc(&self) -> f64 {
        self.base * self.multiplier + self.additive
    }
}
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .insert_resource(Data {
            money: 0,
            base: 5.0,
            multiplier: 1.0,
            additive: 0.0,
            interval: 1.0,
        })
        .insert_resource(UpgradesData {
            upgrades: vec![
                Upgrades::BasicSpeed(0),
                Upgrades::Multiplier(0),
                Upgrades::BaseUpgrade(0),
                Upgrades::BasicAddative(0),
            ],
        })
        .add_startup_system(setup_system)
        .add_system(ui_example)
        .add_system(upgrades_menu)
        .add_system(timer_system)
        .run();
}
fn upgrades_menu(
    mut egui_context: ResMut<EguiContext>,
    mut data: ResMut<Data>,
    mut upgrades_data: ResMut<UpgradesData>,
) {
    egui::Window::new("Upgrades").show(egui_context.ctx_mut(), |ui| {
        upgrades_data.upgrades.iter_mut().for_each(|upgrade| {
            ui.group(|ui| {
                let d = upgrade.display();
                ui.label(format!("{} L:{}", d.title, upgrade.level()));
                ui.label(d.desc);
                if upgrade.max() {
                    ui.add_enabled_ui(false, |ui| {
                        if ui
                            .button(egui::RichText::new("MAX").color(egui::Color32::RED))
                            .clicked()
                        {}
                    });
                } else {
                    ui.add_enabled_ui(data.money > upgrade.price() as i128, |ui| {
                        if ui
                            .button(egui::RichText::new(format!("${}", upgrade.price())).color(
                                if data.money > upgrade.price() as i128 {
                                    egui::Color32::WHITE
                                } else {
                                    egui::Color32::RED
                                },
                            ))
                            .clicked()
                        {
                            data.money -= upgrade.price() as i128;
                            match upgrade.stat() {
                                Multiplier::Addative(x) => {
                                    data.additive -= x;
                                    if let Multiplier::Addative(x) = upgrade.upgrade() {
                                        data.additive += x;
                                    }
                                }
                                Multiplier::Multiplier(x) => {
                                    data.multiplier -= x;
                                    if let Multiplier::Multiplier(x) = upgrade.upgrade() {
                                        data.multiplier += x;
                                    }
                                }
                                Multiplier::Base(x) => {
                                    data.base -= x;
                                    if let Multiplier::Base(x) = upgrade.upgrade() {
                                        data.base += x;
                                    }
                                }
                                Multiplier::Speed(x) => {
                                    data.interval += x;
                                    if let Multiplier::Speed(x) = upgrade.upgrade() {
                                        data.interval -= x;
                                    }
                                }
                            }
                        }
                    });
                };
            });
        })
    });
}
fn ui_example(mut egui_context: ResMut<EguiContext>, data: Res<Data>) {
    egui::Window::new("Stats").show(egui_context.ctx_mut(), |ui| {
        ui.label(format!("Money {}", data.money));
        ui.label(format!(
            "Money/s {:.2}",
            (data.calc() / data.interval as f64)
        ));
        ui.label(format!("Base Earnings {}", data.base));
        ui.label(format!("Multiplier {:.2}", data.multiplier));
        ui.label(format!("Additive {}", data.additive));
        ui.label(format!("Money Interval {}", data.interval));
    });
}

fn setup_system(mut commands: Commands, data: Res<Data>) {
    commands
        .spawn()
        .insert(Timer::from_seconds(data.interval, false));
}

fn timer_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<&mut Timer>,
    mut data: ResMut<Data>,
    mut upgrades_data: ResMut<UpgradesData>,
) {
    for mut timer in query.iter_mut() {
        if timer.tick(time.delta()).just_finished() {
            data.money += data.calc() as i128;
            commands
                .spawn()
                .insert(Timer::from_seconds(data.interval, false));
            if data.calc() > 300.0
                && !upgrades_data
                    .upgrades
                    .iter()
                    .any(|x| Upgrades::SuperBaseUpgrade(x.level()) == *x)
            {
                upgrades_data.upgrades.push(Upgrades::SuperBaseUpgrade(0))
            }
            if data.calc() > 1400.0
                && !upgrades_data
                    .upgrades
                    .iter()
                    .any(|x| Upgrades::BetterMultiplier(x.level()) == *x)
            {
                upgrades_data.upgrades.push(Upgrades::BetterMultiplier(0))
            }
            if data.calc() > 30000.0
                && !upgrades_data
                    .upgrades
                    .iter()
                    .any(|x| Upgrades::InsaneBaseUpgrade(x.level()) == *x)
            {
                upgrades_data.upgrades.push(Upgrades::InsaneBaseUpgrade(0))
            }
        }
    }
}
