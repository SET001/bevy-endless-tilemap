use bevy::prelude::*;

use crate::GameStates;


pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system_set(
        SystemSet::on_enter(GameStates::Game)
          .with_system(start)
      )
      .add_system_set(
        SystemSet::on_update(GameStates::Game)
          .with_system(restart)
      );
  }
}
fn start(){
  info!("entering game state");
}

fn restart(
  mut buttons: ResMut<Input<MouseButton>>,
  mut app_state: ResMut<State<GameStates>>,
){
  if buttons.just_released(MouseButton::Left) {
    info!("button is just released in game state");
    buttons.clear();
    app_state.set(GameStates::Init).unwrap();
  }
}