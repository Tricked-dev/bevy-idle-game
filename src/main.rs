use bevy::{log::info, prelude::*};
use bevy_egui::{
    egui::{
        self,
        epaint::{RectShape, Tessellator},
        pos2, Align2, Color32, FontId, Frame, RichText, Rounding,
    },
    EguiContext, EguiPlugin,
};
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
                ui.label(format!("{} l:{}", d.title, upgrade.level()));
                ui.label(d.desc);
                if upgrade.max() {
                    ui.add_enabled_ui(false, |ui| {
                        if ui
                            .button(RichText::new("MAX").color(Color32::RED))
                            .clicked()
                        {}
                    });
                } else {
                    ui.add_enabled_ui(data.money > upgrade.price() as i128, |ui| {
                        if ui
                            .button(RichText::new(format!("${}", upgrade.price())).color(
                                if data.money > upgrade.price() as i128 {
                                    Color32::WHITE
                                } else {
                                    Color32::RED
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
            "Money/s {}",
            (data.calc() / data.interval as f64).floor()
        ));
    });
}

fn setup_system(mut commands: Commands, data: Res<Data>) {
    // Add an entity to the world with a timer
    commands
        .spawn()
        .insert(Timer::from_seconds(data.interval, false));
}

/// This system ticks all the `Timer` components on entities within the scene
/// using bevy's `Time` resource to get the delta between each update.
fn timer_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<&mut Timer>,
    mut data: ResMut<Data>,
) {
    for mut timer in query.iter_mut() {
        if timer.tick(time.delta()).just_finished() {
            data.money += data.calc() as i128;
            commands
                .spawn()
                .insert(Timer::from_seconds(data.interval, false));
        }
    }
}
