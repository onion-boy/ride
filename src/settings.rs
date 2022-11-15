use bevy::prelude::*;

#[derive(Default)]
pub struct Settings {
    keybinds: Keybinds,
}

#[derive(Default)]
pub struct Keybinds {
    jump: Binding,
}

#[derive(Default)]
pub struct Binding(String, String);

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Settings>();
    }
}
