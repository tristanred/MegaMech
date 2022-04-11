mod bouncy_ball;
mod arena_walls;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

use bouncy_ball::*;
use arena_walls::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup)
        .add_startup_system(setup_bouncy_ball)
        .add_startup_system(setup_arena_walls)
        .add_system(update_walls)
        .run();
}

fn setup(mut commands: Commands, _: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
