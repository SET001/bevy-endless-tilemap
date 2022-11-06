use bevy::asset::diagnostic::AssetCountDiagnosticsPlugin;
use bevy::diagnostic::EntityCountDiagnosticsPlugin;
use bevy::window::{PresentMode};
use bevy::{asset::AssetServerSettings, diagnostic::FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

use bevy_ecs_tilemap::tiles::{TileTexture, TileStorage, TileVisible, TilePos};
use bevy_editor_pls::EditorPlugin;
use chunked_tilemap::{
  ChunkedTilemapPlugin,
  bundle::{ChunkedTilemap, ChunkedTilemapBundle}, spawn::InitChunkEvent, TilemapChunk
};
use game::{AssetsLoading, TilemapLayers};
use game::DefaultCamera;
use game::GameStates;
use game::TextureAtlases;
use game::WorldNoise;
use game::player::PlayerAction;
use game::states::GameStatesPlugins;
use leafwing_input_manager::prelude::InputManagerPlugin;
use perlin2d::PerlinNoise2D;
use rand::{thread_rng, Rng};

const TILE_SIZE: i32 = 32;

fn main() {
  let mut app = App::new();
  app
    .add_startup_system(startup)
    .init_resource::<AssetsLoading>()
    .init_resource::<TextureAtlases>()
    .init_resource::<TilemapLayers>()
    .insert_resource(AssetServerSettings {
      watch_for_changes: true,
      asset_folder: format!("{}/assets", std::env::current_dir().unwrap().to_str().unwrap()),
      ..Default::default()
    })
    .insert_resource(WindowDescriptor{
      title: "Bevy app!".to_string(),
      // mode: WindowMode::Fullscreen,
      present_mode: PresentMode::AutoVsync,
      ..default()
  })
    .add_plugin(FrameTimeDiagnosticsPlugin)
    .add_plugin(EntityCountDiagnosticsPlugin)
    .add_plugin(AssetCountDiagnosticsPlugin::<TextureAtlas>::default())
    .add_plugin(AssetCountDiagnosticsPlugin::<Mesh>::default())
    .add_plugin(AssetCountDiagnosticsPlugin::<Image>::default())

    .add_plugins(DefaultPlugins)
    .add_plugin(ChunkedTilemapPlugin)
    .add_plugin(InputManagerPlugin::<PlayerAction>::default())
    .add_plugin(EditorPlugin)
    .add_plugins(GameStatesPlugins)
    .add_state(GameStates::Load)
    // .add_system(init_grass_chunk)
    .add_system(init_trees_chunk);
  app.run();
}


fn startup(
  mut commands: Commands,
  mut tilemap_layers: ResMut<TilemapLayers>,
  asset_server: Res<AssetServer>,
  windows: Res<Windows>,
){
  
  let mut rng = thread_rng();
  let seed = rng.gen_range(0..2000);
  info!("generating perlin noise");
  let perlin = PerlinNoise2D::new(
    6,
    10.0,
    0.5,
    1.0,
    2.0,
    (100.0, 100.0),
    0.5,
    seed
  );
  info!("perlin noise generated");
  commands.insert_resource(WorldNoise(perlin));
  commands.spawn_bundle(Camera2dBundle::default()).insert(DefaultCamera);

  let primary_window = windows.get_primary().expect("no primary window");
  let chunk_size = IVec2::new(
    (primary_window.width()/TILE_SIZE as f32).round() as i32,
    (primary_window.height()/TILE_SIZE as f32).round() as i32
  );

  info!("chunk_size: {chunk_size}");

  tilemap_layers.ground = Some(commands.spawn_bundle(ChunkedTilemapBundle{
    name: Name::new("Ground layer"),
    chunked_tilemap: ChunkedTilemap{
      chunk_size,
      tile_size: IVec2::new(TILE_SIZE, TILE_SIZE),
      range: 1,
      texture_handle: asset_server.load("images/grass_tiles.png"),
      ..Default::default()
    },
    ..Default::default()
  }).id());

  tilemap_layers.trees = Some(commands.spawn_bundle(ChunkedTilemapBundle{
    name: Name::new("Trees layer"),
    chunked_tilemap: ChunkedTilemap{
      chunk_size,
      tile_size: IVec2::new(TILE_SIZE, TILE_SIZE),
      range: 1,
      texture_handle: asset_server.load("images/tree_tiles.png"),
      ..Default::default()
    },
    spatial: SpatialBundle{
      transform: Transform::from_xyz(0., 0., 10.),
      ..Default::default()
    },
    ..Default::default()
  }).id());
}

fn init_trees_chunk(
  mut er_init_ground_chunk: EventReader<InitChunkEvent>,
  q_tilemaps: Query<(&mut ChunkedTilemap, &Name, &Children)>,
  q_chunk: Query<(&TileStorage, &TilemapChunk)>,
  mut q_tile: Query<(&mut TileTexture, &mut TileVisible)>,
  perlin: Res<WorldNoise>
){
  for event in er_init_ground_chunk.iter(){
    let (tilemap, name, children) = q_tilemaps.get(event.tilemap).unwrap();
    if name.to_string() == "Trees layer".to_string(){
      for &child in children.iter(){
        if let Ok((tile_storage, tilemap_chunk)) = q_chunk.get(child){
          if tilemap_chunk.0 == event.index{

            for x in 0..tilemap.chunk_size.x{
              for y in 0..tilemap.chunk_size.y{
                if let Some(tile) = tile_storage.get(&TilePos{
                  x: x as u32,
                  y: y as u32
                }){
                  let (mut tile_texture, mut tile_visible) = q_tile.get_mut(tile).unwrap();
                  let offset = event.index * tilemap.chunk_size*tilemap.tile_size+IVec2::new(x, y)*tilemap.tile_size;
                  if perlin.0.get_noise(offset.x.into(), offset.y.into()) > 8.  {
                    let mut rng = thread_rng();
                    let tile = rng.gen_range(0..20);
                    tile_texture.0 = tile;
                    tile_visible.0 = true;
                  } else {
                    tile_visible.0 = false;
                  };
                }
              }
            }
          }
        }
      }
    }
  }
}

// fn init_grass_chunk(
//   mut er_init_ground_chunk: EventReader<InitChunkEvent>,
//   q_tilemaps: Query<(&mut ChunkedTilemap, &Name, &Children)>,
//   q_chunk: Query<(&TileStorage, &TilemapChunk)>,
//   mut q_tile: Query<(&mut TileTexture, &mut TileVisible)>,
//   perlin: Res<WorldNoise>
// ){
//   for event in er_init_ground_chunk.iter(){
//     let (tilemap, name, children) = q_tilemaps.get(event.tilemap).unwrap();
//     if name.to_string() == "Trees layer".to_string(){
//       for &child in children.iter(){
//         if let Ok((tile_storage, tilemap_chunk)) = q_chunk.get(child){
//           if tilemap_chunk.0 == event.index{

//             for x in 0..tilemap.chunk_size.x{
//               for y in 0..tilemap.chunk_size.y{
//                 if let Some(tile) = tile_storage.get(&TilePos{
//                   x: x as u32,
//                   y: y as u32
//                 }){
//                   let (mut tile_texture, mut tile_visible) = q_tile.get_mut(tile).unwrap();
//                   let offset = event.index * tilemap.chunk_size*tilemap.tile_size+IVec2::new(x, y)*tilemap.tile_size;
//                   if perlin.0.get_noise(offset.x.into(), offset.y.into()) > 0.  {
//                     tile_texture.0 = 10;
//                     tile_visible.0 = true;
//                   } else {
//                     tile_visible.0 = false;
//                   };
//                 }
//               }
//             }
//           }
//         }
//       }
//     }
//   }
// }