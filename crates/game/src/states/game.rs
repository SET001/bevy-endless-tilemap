use bevy::{prelude::*};
use chunked_tilemap::bundle::ChunkedTilemap;

use crate::{GameStates, player::{spawn_player, player_controls, bind_camera_to_player}, DefaultCamera, };

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
          .with_system(bind_camera_to_player.after(player_controls))
          .with_system(update_tilemaps)
          // .with_system(restart)
        );
  }
}
fn start(
  // mut commands: Commands,
  // mut meshes: ResMut<Assets<Mesh>>,
  //   mut materials: ResMut<Assets<ColorMaterial>>,
){
  info!("entering game state");
}

// fn restart(
//   mut buttons: ResMut<Input<MouseButton>>,
//   mut app_state: ResMut<State<GameStates>>,
// ){
//   // if buttons.just_released(MouseButton::Left) {
//   //   info!("button is just released in game state");
//   //   buttons.clear();
//   //   app_state.set(GameStates::Init).unwrap();
//   // }
// }

fn update_tilemaps(
  mut q_tilemaps: Query<&mut ChunkedTilemap>,
  q_camera: Query<&Transform, With<DefaultCamera>>,
){
  let camera_transform = q_camera.get_single().unwrap();
  for mut tilemap in q_tilemaps.iter_mut(){
    tilemap.center = camera_transform.translation.truncate();
  }
}