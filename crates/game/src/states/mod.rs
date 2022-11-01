use bevy::{prelude::PluginGroup, app::PluginGroupBuilder};

pub mod game;
pub mod load;
pub mod init;

pub struct GameStatesPlugins;

impl PluginGroup for GameStatesPlugins {
  fn build(&mut self, group: &mut PluginGroupBuilder) {
    group
      .add(game::GameStatePlugin)
      .add(init::InitStatePlugin)
      .add(load::LoadStatePlugin);
  }
}