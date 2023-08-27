use bevy::prelude::*;
use your_asset_name::MyPlugin;

// An example of how to use the plugin in a Bevy app
fn main() {
    App::new().add_plugins((DefaultPlugins, MyPlugin)).run();
}
