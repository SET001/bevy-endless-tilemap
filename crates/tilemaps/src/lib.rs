use bevy::prelude::Plugin;
use bevy_ecs_tilemap::TilemapPlugin;
use chunks::ChunkManager;
use spawn::{SpawnChunkEvent, spawn_chunk};

pub mod chunks;
pub mod spawn;
pub mod despawn;

pub struct ChunkedTilemapPlugin;

impl Plugin for ChunkedTilemapPlugin{
  fn build(&self, app: &mut bevy::prelude::App) {
    app
      .add_event::<SpawnChunkEvent>()
      .init_resource::<ChunkManager>()
      .add_plugin(TilemapPlugin)
      .add_system(spawn_chunk);
  }
}