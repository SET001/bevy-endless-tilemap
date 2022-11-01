use bevy::{prelude::*, DefaultPlugins};
use bevy_editor_pls::EditorPlugin;
use tilemaps::{spawn::SpawnChunkEvent, chunks::{ChunkManager, get_chunk_center}, ChunkedTilemapPlugin};

fn main() {
  let mut app = App::new();
  app
    .add_startup_system(startup)
    .add_system(spawn_chunks_around_current)
    // .init_resource::<TextureAtlases>()
    // .insert_resource(AppConfig{
    //   tile_size: 32,
    //   chunk_size: 5
    // })
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
  
  let current_chunk_index: IVec2 = IVec2 { x: 2, y: 2 };

  for y in (current_chunk_index.y - 2)..=(current_chunk_index.y + 2) {
    for x in (current_chunk_index.x - 2)..=(current_chunk_index.x + 2) {
      let index = IVec2::new(x, y);
      if !chunk_manager.chunks.contains(&index){
        ew_spawn_chunk.send(SpawnChunkEvent {
          chunk_size: IVec2::new(5, 5),
          tile_size: IVec2::new(32, 32),
          chunk_possition: get_chunk_center(
            5,
            IVec2::new(index.x - current_chunk_index.x, index.y - current_chunk_index.y),
            32
          ),
          chunk_index: index
        });
      }
    }
  }
}