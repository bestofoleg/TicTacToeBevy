mod systems;
mod components;
mod core;
mod states;
mod resources;

use crate::components::{GameField, PlayableCell, PlayableCellState};
use crate::states::GameState;
use crate::systems::{load_resources, on_game_field_created_system, startup, ui_example_system};
use avian2d::prelude::PhysicsPickingPlugin;
use avian2d::PhysicsPlugins;
use bevy::app::{App, FixedUpdate, PluginGroup, Startup};
use bevy::prelude::{AppExtStates, ImagePlugin, IntoScheduleConfigs};
use bevy::DefaultPlugins;
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            PhysicsPlugins::default().with_length_unit(100.0),
            PhysicsPickingPlugin,
            EguiPlugin::default(),
        ))
        .add_systems(Startup, (load_resources, startup.after(load_resources)))
        .add_systems(EguiPrimaryContextPass, ui_example_system)
        .add_systems(FixedUpdate, on_game_field_created_system)
        .init_state::<GameState>()
        .run();
}
