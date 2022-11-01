use bevy::{prelude::*, utils::HashSet, sprite::MaterialMesh2dBundle};
use bevy_ecs_tilemap::{
  prelude::*,
  tiles::{TileStorage, TilePos, TileBundle},
  TilemapBundle,
  helpers::geometry::get_tilemap_center
};

use crate::{GameStates, player::{spawn_player, player_controls, bind_camera_to_player}, AppConfig, GroundTilemap, };

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event::<SpawnChunkEvent>()
      .init_resource::<ChunkManager>()
      .add_system_set(
        SystemSet::on_enter(GameStates::Game)
          .with_system(start)
          .with_system(spawn_player)
        )
        .add_system_set(
          SystemSet::on_update(GameStates::Game)
          // .with_system(player_controls)
          .with_system(bind_camera_to_player)
          .with_system(restart)
          .with_system(spawn_chunks_around_current)
          .with_system(despawn_outofrange_chunks)
          .with_system(spawn_chunk)
        );
  }
}
fn start(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
){
  info!("entering game state");
  commands.spawn_bundle(MaterialMesh2dBundle {
    mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
    transform: Transform::from_translation(Vec2::default().extend(-10.)).with_scale(Vec3::splat(128.)),
    material: materials.add(ColorMaterial::from(Color::PURPLE)),
    ..default()
});
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