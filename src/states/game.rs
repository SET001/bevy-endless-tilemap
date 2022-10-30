use bevy::prelude::*;

use crate::{GameStates, player::{spawn_player, player_controls, bind_camera_to_player}, };


pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system_set(
        SystemSet::on_enter(GameStates::Game)
          .with_system(start)
          .with_system(spawn_player)
      )
      .add_system_set(
        SystemSet::on_update(GameStates::Game)
          .with_system(player_controls)
          .with_system(bind_camera_to_player)
          .with_system(restart)
          .with_system(despawn_outofrange_chunks)
          .with_system(despawn_outofrange_chunks)
          
      )
      ;
  }
}
fn start(){
  info!("entering game state");
}

fn restart(
  mut buttons: ResMut<Input<MouseButton>>,
  mut app_state: ResMut<State<GameStates>>,
){
  // if buttons.just_released(MouseButton::Left) {
  //   info!("button is just released in game state");
  //   buttons.clear();
  //   app_state.set(GameStates::Init).unwrap();
  // }
}

fn spawn_chunk(){

}

fn spawn_chunks_around_camera(){
  
}

fn despawn_outofrange_chunks(){

}