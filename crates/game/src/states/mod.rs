use bevy::{prelude::PluginGroup, app::PluginGroupBuilder};

pub mod game;
pub mod load;
pub struct GameStatesPlugins;

impl PluginGroup for GameStatesPlugins {
  fn build(&mut self, group: &mut PluginGroupBuilder) {
    group
      .add(game::GameStatePlugin)
      .add(load::LoadStatePlugin);
  }
}
