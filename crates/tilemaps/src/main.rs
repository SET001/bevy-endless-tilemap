use bevy::{prelude::*, DefaultPlugins};
use bevy_editor_pls::EditorPlugin;
use tilemaps::{spawn::SpawnChunkEvent, chunks::{ChunkManager, get_chunk_center}, ChunkedTilemapPlugin};

const CHUNK_SIZE: i32 = 5;
fn main() {
  let mut app = App::new();
  app
    .add_startup_system(startup)
    .add_system(spawn_chunks_around_current)
    .add_plugins(DefaultPlugins)
    .add_plugin(ChunkedTilemapPlugin)
    .add_plugin(EditorPlugin);
  app.run();
}

fn startup(
  mut commands: Commands,
){
  
  commands.spawn_bundle(Camera2dBundle::default());

}

fn spawn_chunks_around_current(
  mut ew_spawn_chunk: EventWriter<SpawnChunkEvent>,
  chunk_manager: ResMut<ChunkManager>,
){
  
  
  let current_chunk_index: IVec2 = IVec2 { x: 232, y: 451 };

  for y in (current_chunk_index.y - 2)..=(current_chunk_index.y + 2) {
    for x in (current_chunk_index.x - 2)..=(current_chunk_index.x + 2) {
      let index = IVec2::new(x, y);
      if !chunk_manager.chunks.contains(&index){
        ew_spawn_chunk.send(SpawnChunkEvent {
          chunk_size: IVec2::new(CHUNK_SIZE, CHUNK_SIZE),
          tile_size: IVec2::new(32, 32),
          chunk_possition: get_chunk_center(
            CHUNK_SIZE,
            IVec2::new(index.x - current_chunk_index.x, index.y - current_chunk_index.y),
            32
          ),
          chunk_index: index
        });
      }
    }
  }
}