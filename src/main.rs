mod character;
mod settings;
mod window;

use bevy::prelude::*;
use settings::SettingsPlugin;
use window::WindowSetupPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WindowSetupPlugin)
        .add_plugin(SettingsPlugin)
        .run();
}
