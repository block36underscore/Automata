use bevy::prelude::*;

use crate::automata::Life;

pub struct GamePlugin;


impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_camera, Life::setup));
        app.add_systems(Update, Life::render);
        app.insert_resource(ClearColor(Color::rgb(0.05, 0.4, 0.3)));
    }
}

#[derive(Component)]
pub struct CameraMarker;

fn setup_camera(mut commands: Commands) {

    let camera = Camera2dBundle {
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..Default::default()
            };

    commands.spawn((
            camera,
            CameraMarker,
        ));
}
