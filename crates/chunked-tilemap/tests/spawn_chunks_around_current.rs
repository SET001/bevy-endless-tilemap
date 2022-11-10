use bevy::{prelude::*, winit::WinitPlugin, log::LogPlugin, utils::HashSet};
use chunked_tilemap::{ChunkedTilemapPlugin, bundle::{ChunkedTilemap, ChunkedTilemapBundle}, spawn::PrepareChunkEvent};

const CHUNK_SIZE: u32 = 5;
const TILE_SIZE: f32 = 32.;

fn get_app()->App{
  let mut app = App::new();
  app
    .add_plugins_with(DefaultPlugins, |group| {
      group.disable::<WinitPlugin>();
      group.disable::<LogPlugin>()
    })
    .add_plugin(ChunkedTilemapPlugin);
  app
}

#[test]
fn should_emit_events(){
  let mut app = get_app();
  let mut chunks = HashSet::new();
  chunks.insert(IVec2::new(-1, -1));
  chunks.insert(IVec2::new(-100, -100));
  app.world.spawn().insert_bundle(ChunkedTilemapBundle{
    chunked_tilemap: ChunkedTilemap{
      chunk_size: UVec2::new(CHUNK_SIZE, CHUNK_SIZE),
      tile_size: Vec2::new(TILE_SIZE, TILE_SIZE),
      chunks,
      range: 1,
      ..Default::default()
    },
    ..Default::default()
  });

  app.update();
  
  let er = app.world.resource::<Events<PrepareChunkEvent>>();
  assert_eq!(er.len(), 8);
}