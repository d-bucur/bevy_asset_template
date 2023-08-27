use bevy::prelude::*;
use your_asset_name::MyPlugin;

impl Plugin for MyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup() {
    println!("Hello MyPlugin");
}