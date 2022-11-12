use bevy::prelude::*;
use bevy_ecs_tilemap::{tiles::TileBundle, prelude::TilemapId};

pub struct FillChunkEvent{
  pub chunk_index: IVec2,
  pub chunk_entity: Entity,
  pub bundles: Vec<TileBundle>,
}
pub fn fill_chunk(
  mut commands: Commands,
  mut er_fill_chunk_event: EventReader<FillChunkEvent>
){
  for event in er_fill_chunk_event.iter(){
    info!("filling chunk {:?} with {:?} bundles", event.chunk_index, event.bundles.len());
    let mut bundles = event.bundles.clone();
    for bundle in bundles.iter_mut(){
      bundle.tilemap_id = TilemapId(event.chunk_entity);
    }
    commands.spawn_batch(bundles);
  }
}