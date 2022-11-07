use bevy::asset::diagnostic::AssetCountDiagnosticsPlugin;
use bevy::diagnostic::EntityCountDiagnosticsPlugin;
use bevy::window::{PresentMode, WindowMode};
use bevy::{asset::AssetServerSettings, diagnostic::FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

use bevy_ecs_tilemap::tiles::{TileTexture, TilePos, TileBundle};
use bevy_editor_pls::EditorPlugin;
use chunked_tilemap::spawn::{SpawnChunkEvent, PrepareChunkEvent};
use chunked_tilemap::{
  ChunkedTilemapPlugin,
  bundle::{ChunkedTilemap, ChunkedTilemapBundle}
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

const TILE_SIZE: f32 = 32.;

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
      mode: WindowMode::Fullscreen,
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
    .add_system(init_grass_chunk)
    .add_system(init_trees_chunk)
    // .add_system(on_window_resize)
    ;
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
  let chunk_size = UVec2::new(
    (primary_window.width()/TILE_SIZE as f32).round() as u32,
    (primary_window.height()/TILE_SIZE as f32).round() as u32
  );

  // let chunk_size = IVec2::new(
  //   10,
  //   10
  // );

  info!("window size: {}x{}", primary_window.width(), primary_window.height());
  info!("chunk_size: {chunk_size}");

  tilemap_layers.ground = Some(commands.spawn_bundle(ChunkedTilemapBundle{
    name: Name::new("Ground layer"),
    chunked_tilemap: ChunkedTilemap{
      chunk_size,
      tile_size: Vec2::new(TILE_SIZE, TILE_SIZE),
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
      tile_size: Vec2::new(TILE_SIZE, TILE_SIZE),
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
  mut er_prepare_chunk: EventReader<PrepareChunkEvent>,
  mut ew_spawn_chunk: EventWriter<SpawnChunkEvent>,
  tilemap_layers: Res<TilemapLayers>,
  q_tilemaps: Query<&mut ChunkedTilemap>,
  perlin: Res<WorldNoise>
){
  let init_ground_chunk_events = er_prepare_chunk.iter().filter(|event| event.tilemap_entity == tilemap_layers.trees.unwrap());
  for event in init_ground_chunk_events{
    let tilemap = q_tilemaps.get(event.tilemap_entity).expect("no tilemap");
    let mut bundles = vec![];
    for x in 0..tilemap.chunk_size.x{
      for y in 0..tilemap.chunk_size.y{
        if perlin.0.get_noise(
          (event.chunk_index.x as f64) * (tilemap.chunk_size.x as f64)+(x as f64)*(tilemap.tile_size.x as f64),
          (event.chunk_index.y as f64) * (tilemap.chunk_size.y as f64)+(y as f64)*(tilemap.tile_size.y as f64)
        ) > 8.  {
          let mut rng = thread_rng();
          let tile_index = rng.gen_range(0..20);
          bundles.push(TileBundle {
            position: TilePos { x, y},
            texture: TileTexture(tile_index),
            ..Default::default()
          });
        }
      }
    }
    ew_spawn_chunk.send(SpawnChunkEvent{
      bundles,
      tilemap_entity: event.tilemap_entity,
      chunk_index: event.chunk_index
    })
  }
}

fn init_grass_chunk(
  mut er_prepare_chunk: EventReader<PrepareChunkEvent>,
  mut ew_spawn_chunk: EventWriter<SpawnChunkEvent>,
  tilemap_layers: Res<TilemapLayers>,
  q_tilemaps: Query<&mut ChunkedTilemap>,
){
  let init_chunk_events = er_prepare_chunk.iter().filter(|event| event.tilemap_entity == tilemap_layers.ground.unwrap());
  for event in init_chunk_events{
    let tilemap = q_tilemaps.get(event.tilemap_entity).expect("no tilemap");
    let mut bundles = vec![];
    for x in 0..tilemap.chunk_size.x{
      for y in 0..tilemap.chunk_size.y{
        bundles.push(TileBundle {
          position: TilePos { x, y},
          ..Default::default()
        });
      }
    }
    ew_spawn_chunk.send(SpawnChunkEvent{
      bundles,
      tilemap_entity: event.tilemap_entity,
      chunk_index: event.chunk_index
    })
  }
}


// fn on_window_resize(
//   mut e_resized: EventReader<WindowResized>,
//   mut q_tilemaps: Query<&mut ChunkedTilemap>,
//   windows: Res<Windows>,
// ){
//   for _ in e_resized.iter(){
//     let window = windows.get_primary().expect("no primary window");
//     info!("window resized: {}x{}", window.width(), window.height());
//     for mut tilemap in q_tilemaps.iter_mut(){
//       tilemap.chunk_size = IVec2::new(
//         (window.width()/TILE_SIZE as f32).round() as i32,
//         (window.height()/TILE_SIZE as f32).round() as i32
//       );
//     }
//   }
// }