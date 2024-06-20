#![feature(trait_upcasting)]

use bevy::prelude::*;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use game_plugin::GamePlugin;

pub mod game_plugin;
pub mod characters;
pub mod automata;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins
                        .set(ImagePlugin::default_nearest()),
                        FrameTimeDiagnosticsPlugin::default(),
                        GamePlugin,
                    ))
        .run();
}
