use bevy::{prelude::*};
use chunked_tilemap::bundle::ChunkedTilemap;

use crate::{GameStates, player::{spawn_player, player_controls, bind_camera_to_player}, DefaultCamera, };

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system_set(
        SystemSet::on_enter(GameStates::Game)
          .with_system(spawn_player)
        )
        .add_system_set(
          SystemSet::on_update(GameStates::Game)
          .with_system(player_controls)
          .with_system(bind_camera_to_player.after(player_controls))
          .with_system(update_tilemaps)
        );
  }
}


fn update_tilemaps(
  mut q_tilemaps: Query<&mut ChunkedTilemap>,
  q_camera: Query<&Transform, With<DefaultCamera>>,
){
  let camera_transform = q_camera.get_single().unwrap();
  for mut tilemap in q_tilemaps.iter_mut(){
    tilemap.center = camera_transform.translation.truncate();
  }
}
