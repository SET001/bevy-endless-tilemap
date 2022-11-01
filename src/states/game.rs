use bevy::{prelude::*, utils::HashSet, sprite::MaterialMesh2dBundle};
use bevy_ecs_tilemap::{
  prelude::*,
  tiles::{TileStorage, TilePos, TileBundle},
  TilemapBundle,
  helpers::geometry::get_tilemap_center
};

use crate::{GameStates, player::{spawn_player, player_controls, bind_camera_to_player}, AppConfig, GroundTilemap, };

#[derive(Default)]
struct ChunkManager{
  pub chunks: HashSet<IVec2>
}
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

pub struct SpawnChunkEvent{
  chunk_possition: Vec2,
  chunk_index: IVec2
}

fn spawn_chunk(
  mut er_spawn_chunk: EventReader<SpawnChunkEvent>,
  mut commands: Commands,
  mut chunk_manager: ResMut<ChunkManager>,
  asset_server: Res<AssetServer>,
  config: Res<AppConfig>,
){
  for event in er_spawn_chunk.iter(){
    
    let ground_tilemap_entity = commands.spawn().id();
    let tilemap_size = TilemapSize{
      x: config.chunk_size as u32,
      y: config.chunk_size as u32,
    };
    let tile_size = TilemapTileSize {
      x: config.tile_size as f32,
      y: config.tile_size as f32
    };
    let grid_size = TilemapGridSize { x: 32.0, y: 32.0};

    let mut tile_storage = TileStorage::empty(tilemap_size);

    
    let tile =  if (event.chunk_index.x+event.chunk_index.y) % 2 > 0 {
      0
    } else {
      3
    };

    for x in 0..tilemap_size.x {
      for y in 0..tilemap_size.y {
        // let tile =  if (y+x) % 2 > 0 {
        //   0
        // } else {
        //   3
        // };
        let tile_pos = TilePos { x, y};
          let tile_entity = commands
            .spawn()
            .insert_bundle(TileBundle {
              position: tile_pos,
              texture: TileTexture(tile),
              tilemap_id: TilemapId(ground_tilemap_entity),
              ..Default::default()
            })
            .id();
          tile_storage.set(&tile_pos, tile_entity);
          commands.entity(ground_tilemap_entity).push_children(&[tile_entity]);
      }
    }
    let transform = Transform::from_translation(event.chunk_possition.extend(0.));
    info!("spawning chunk {:?} on position {:?}", event.chunk_index, transform.translation);
    commands
      .entity(ground_tilemap_entity)
      .insert_bundle(TilemapBundle {
        grid_size,
        size: tilemap_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(asset_server.load("images/grass_tiles.png")),
        tile_size,
        transform,
        ..Default::default()
      })
      .insert_bundle((
        GroundTilemap,
        Name::new("Ground Tilemap")
      ));

    chunk_manager.chunks.insert(event.chunk_index);
  }
}

fn spawn_chunks_around_current(
  mut ew_spawn_chunk: EventWriter<SpawnChunkEvent>,
  chunk_manager: ResMut<ChunkManager>,
  config: Res<AppConfig>,
){
  let current_chunk_index: IVec2 = IVec2 { x: 2, y: 2 };

  for y in (current_chunk_index.y - 2)..=(current_chunk_index.y + 2) {
    for x in (current_chunk_index.x - 2)..=(current_chunk_index.x + 2) {
      let index = IVec2::new(x, y);
      if !chunk_manager.chunks.contains(&index){
        ew_spawn_chunk.send(SpawnChunkEvent {
          chunk_possition: get_chunk_center(
            config.chunk_size,
            IVec2::new(index.x - current_chunk_index.x, index.y - current_chunk_index.y),
            32
          ),
          chunk_index: index
        });
      }
    }
  }
}

fn despawn_outofrange_chunks(
  mut commands: Commands
){

}

pub fn get_chunk_center(
  chunk_size: i32,
  relative_position: IVec2,
  tile_size: i32,
)->Vec2{
  Vec2::new(
    (-tile_size*((chunk_size-1)/2)) as f32 - ((relative_position.x)*tile_size*chunk_size) as f32,
    (-tile_size*((chunk_size-1)/2)) as f32 - ((relative_position.y)*tile_size*chunk_size) as f32
  )
}

#[cfg(test)]
mod test{
  use bevy::prelude::*;
  use rstest::rstest;

  use crate::states::game::get_chunk_center;

  #[rstest]
  #[case(1, (0, 0), (0., 0.))]
  #[case(2, (0, 0), (-0., -0.))]
  #[case(3, (0, 0), (-32., -32.))]
  #[case(4, (0, 0), (-32., -32.))]
  #[case(5, (0, 0), (-64., -64.))]
  #[case(1, (-1, 0), (-32., 0.))]
  #[case(1, (1, 1), (32., 32.))]
  #[case(1, (0, 1), (0., 32.))]

  // #[case(3, (0, 1), (-32., -32.))]
  fn get_chunk_center_test(
    #[case] chunk_size: i32,
    #[case] relative_position: (i32, i32),
    #[case] expected: (f32, f32),
  ){
    assert_eq!(get_chunk_center(
      chunk_size,
      IVec2::from(relative_position),
      32
    ), Vec2::from(expected));
  }
}