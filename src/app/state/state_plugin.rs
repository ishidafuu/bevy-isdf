use bevy::prelude::{App, IntoSystem, Plugin, State, SystemSet};

use super::state::*;

pub struct SwitchStatePlugin;
impl Plugin for SwitchStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_state(AppStates::Setup)
            //Setup sequence
            .add_system_set(
                SystemSet::on_enter(AppStates::Setup).with_system(on_setup_entry),
            )
            .add_system_set(
                SystemSet::on_update(AppStates::Setup).with_system(on_setup_process),
            )
            .add_system_set(
                SystemSet::on_exit(AppStates::Setup).with_system(on_setup_exit),
            )
            //State A
            .add_system_set(
                SystemSet::on_enter(AppStates::A).with_system(on_state_a_entered),
            )
            .add_system_set(
                SystemSet::on_update(AppStates::A).with_system(on_state_a_process),
            )
            .add_system_set(SystemSet::on_exit(AppStates::A).with_system(on_state_a_exit))
            //State B
            .add_system_set(
                SystemSet::on_enter(AppStates::B).with_system(on_state_b_entered),
            )
            .add_system_set(
                SystemSet::on_update(AppStates::B).with_system(on_state_b_process),
            )
            .add_system_set(SystemSet::on_exit(AppStates::B).with_system(on_state_b_exit));
    }
}
