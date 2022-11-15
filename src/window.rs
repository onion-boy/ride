use bevy::{prelude::*, window::PresentMode};

pub struct WindowSetupPlugin;

impl WindowSetupPlugin {
    fn add_camera(mut commands: Commands) {
        commands.spawn_bundle(Camera2dBundle::default());
    }
}

impl Plugin for WindowSetupPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WindowDescriptor {
            title: "ride".to_string(),
            width: 1000.,
            height: 700.,
            present_mode: PresentMode::AutoVsync,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::hex("efefef").unwrap()))
        .add_startup_system(WindowSetupPlugin::add_camera);
    }
}
