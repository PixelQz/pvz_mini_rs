use assets_load::MyAssetsPlugin;
use bevy::{prelude::*, window::WindowResolution};
use controller::ControllerPlugin;
use panel::PanelPlugin;
use play::PlayPlugin;
use menu::menu::MenuPlugin;

mod assets_load;
mod controller;
mod play;
mod menu;
mod setting;
mod panel;

#[derive(Clone, Copy,Debug,Default,PartialEq, Eq,Hash,States)]
pub enum GameState {
    #[default]
    Menu,
    Play,
    Exit,
}
// assets/Cards
fn main() {
    let mut app=App::new();
    app
    .add_plugins( DefaultPlugins
        .set(WindowPlugin {
            primary_window: Some(Window {
                title: "Plants vs. Zombies: Mini".to_string(),
                resolution: WindowResolution::new(1024.0, 768.0),
                resizable: false,
                ..default()
            }),
            ..default()
        })
        .set(ImagePlugin::default_nearest()),)

    .add_plugins(MyAssetsPlugin)
    .add_plugins(PlayPlugin)
    .add_plugins(MenuPlugin)
    .add_plugins(ControllerPlugin)
    .add_plugins(PanelPlugin)
    .init_state::<GameState>()
    ;
    app.run();
}







