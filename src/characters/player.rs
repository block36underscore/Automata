use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

impl Player {
    pub fn spawn(mut commands: Commands) {
        println!("spawned");
        commands
            .spawn(SpriteBundle {
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(120.0, 30.0)),
                    color: Color::rgb(0.5, 0.7, 0.6),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Player);
        
    }
}
