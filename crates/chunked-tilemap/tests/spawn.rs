use bevy::{prelude::*, winit::WinitPlugin, log::LogPlugin};
use bevy_ecs_tilemap::tiles::{TileBundle, TilePos, TileTexture};
use chunked_tilemap::{ChunkedTilemapPlugin, bundle::{ChunkedTilemap, ChunkedTilemapBundle}, TilemapChunk, spawn_chunk::{SpawnChunkEvent, PrepareChunkEvent}, fill_chunk::FillChunkEvent};

const CHUNK_SIZE: u32 = 5;
const TILE_SIZE: f32 = 32.;

fn fill_chunk(
  mut er_prepare_chunk: EventReader<PrepareChunkEvent>,
  mut ew_fill_chunk: EventWriter<FillChunkEvent>,
){
  let mut bundles = vec![];
  for x in 0..10{
    bundles.push(TileBundle {
      position: TilePos { x, y: 0},
      texture: TileTexture(1),
      ..Default::default()
    });
  }
  for event in er_prepare_chunk.iter(){
    
    ew_fill_chunk.send(FillChunkEvent{
      bundles: bundles.clone(),
      chunk_entity: event. chunk_entity,
      chunk_index: event.chunk_index
    })
  }
}

fn get_app()->App{
  let mut app = App::new();
  app
    .add_plugins_with(DefaultPlugins, |group| {
      group.disable::<WinitPlugin>();
      group.disable::<LogPlugin>()
    })
    .add_plugin(ChunkedTilemapPlugin)
    .add_system(fill_chunk);
  app
}

#[test]
fn test_chunks_spawned_on_start() {
  let mut app = get_app();
  app.world.spawn().insert_bundle(ChunkedTilemapBundle{
    chunked_tilemap: ChunkedTilemap{
      chunk_size: UVec2::new(CHUNK_SIZE, CHUNK_SIZE),
      tile_size: Vec2::new(TILE_SIZE, TILE_SIZE),
      range: 1,
      ..Default::default()
    },
    ..Default::default()
  });
  app.update();
  app.update();
  app.update();

  app.update();
  app.update();
  app.update();
  app.update();
  app.update();
  app.update();
  
  let c_chunks = app.world.query::<&TilemapChunk>().iter(&app.world).len();

  assert_eq!(c_chunks, 9);
}