use bevy::prelude::{AppBuilder, IntoSystem, Plugin, State, SystemSet};

use super::state::*;

pub struct SwitchStatePlugin;
impl Plugin for SwitchStatePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_state(AppStates::Setup)
            //Setup sequence
            .add_system_set(
                SystemSet::on_enter(AppStates::Setup).with_system(on_setup_entry.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppStates::Setup).with_system(on_setup_process.system()),
            )
            .add_system_set(
                SystemSet::on_exit(AppStates::Setup).with_system(on_setup_exit.system()),
            )
            //State A
            .add_system_set(
                SystemSet::on_enter(AppStates::A).with_system(on_state_a_entered.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppStates::A).with_system(on_state_a_process.system()),
            )
            .add_system_set(SystemSet::on_exit(AppStates::A).with_system(on_state_a_exit.system()))
            //State B
            .add_system_set(
                SystemSet::on_enter(AppStates::B).with_system(on_state_b_entered.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppStates::B).with_system(on_state_b_process.system()),
            )
            .add_system_set(SystemSet::on_exit(AppStates::B).with_system(on_state_b_exit.system()));
    }
}
