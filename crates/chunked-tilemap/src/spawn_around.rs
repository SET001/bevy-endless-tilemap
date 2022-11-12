use bevy::{prelude::*, utils::HashSet};
use crate::{spawn_chunk::{PrepareChunkEvent, SpawnChunkEvent}, bundle::ChunkedTilemap};

pub fn generate_chunk_indexes(
  current_chunk_index: IVec2,
  range: i32,
)->Vec<IVec2>{
  let mut indexes = vec![];

  for y in ((current_chunk_index.y - range)..=(current_chunk_index.y + range)).rev() {
    for x in (current_chunk_index.x - range)..=(current_chunk_index.x + range) {
      indexes.push(IVec2::new(x, y));
    }
  }
  indexes
}

fn prepare_event(
  existing_indexes: &HashSet<IVec2>,
  chunk_index: IVec2,
  tilemap_entity: Entity
)->Option<SpawnChunkEvent>{
  if !existing_indexes.contains(&chunk_index){
    debug!("Spawning chunk init event for {:#?}", tilemap_entity);
    Some(SpawnChunkEvent {
      tilemap_entity,
      chunk_index
    })
  } else {
    None
  }
}

pub fn spawn_chunks_around_current(
  mut ew_prepare_chunk: EventWriter<SpawnChunkEvent>,
  q_tilemaps: Query<(&ChunkedTilemap, Entity)>
){
  for (tilemap, entity) in q_tilemaps.iter(){
    generate_chunk_indexes(tilemap.current_chunk, tilemap.range as i32).iter().for_each(|index|{
      if let Some(event) = prepare_event(&tilemap.chunks, index.clone(), entity){
        ew_prepare_chunk.send(event);
      }
    });
  }
}

#[cfg(test)]
mod test{
  use bevy::prelude::*;
  use bevy::utils::HashSet;
  use super::{generate_chunk_indexes, prepare_event};
  use crate::spawn_chunk::PrepareChunkEvent;
  use rstest::rstest;

  #[rstest]
  #[case (IVec2::new(0, 0), 1, vec![
    IVec2::new(-1, 1), IVec2::new(0, 1), IVec2::new(1, 1),
    IVec2::new(-1, 0), IVec2::new(0, 0), IVec2::new(1, 0),
    IVec2::new(-1, -1), IVec2::new(0, -1), IVec2::new(1, -1),
  ])]

  #[case (IVec2::new(5, -10), 1, vec![
    IVec2::new(4, -9), IVec2::new(5, -9), IVec2::new(6, -9),
    IVec2::new(4, -10), IVec2::new(5, -10), IVec2::new(6, -10),
    IVec2::new(4, -11), IVec2::new(5, -11), IVec2::new(6, -11),
  ])]

  #[case (IVec2::new(0, 0), 2, vec![
    IVec2::new(-2, 2), IVec2::new(-1, 2), IVec2::new(0, 2), IVec2::new(1, 2), IVec2::new(2, 2),
    IVec2::new(-2, 1), IVec2::new(-1, 1), IVec2::new(0, 1), IVec2::new(1, 1), IVec2::new(2, 1),
    IVec2::new(-2, 0), IVec2::new(-1, 0), IVec2::new(0, 0), IVec2::new(1, 0), IVec2::new(2, 0),
    IVec2::new(-2, -1), IVec2::new(-1, -1), IVec2::new(0, -1), IVec2::new(1, -1), IVec2::new(2, -1),
    IVec2::new(-2, -2), IVec2::new(-1, -2), IVec2::new(0, -2), IVec2::new(1, -2), IVec2::new(2, -2)
  ])]
  fn test_generate_chunk_indexes(
    #[case] current_chunk_index: IVec2,
    #[case] range: i32,
    #[case] expect: Vec<IVec2>
  ){
    assert_eq!(
      generate_chunk_indexes(current_chunk_index, range),
      expect
    )
  }

  // #[test]
  // fn test_prepare_event_for_index_that_does_not_exist(){
  //   let chunk_index = IVec2::new(10, 12);
  //   let tilemap_entity = Entity::from_raw(123);
  //   assert_eq!(
  //     prepare_event(
  //       &HashSet::new(),
  //       chunk_index,
  //       tilemap_entity
  //     ),
  //     Some(SpawnChunkEvent{tilemap_entity, chunk_index})
  //   );
  // }

  // #[test]
  // fn test_prepare_event_for_index_that_does_exist(){
  //   let chunk_index = IVec2::new(10, 12);
  //   let tilemap_entity = Entity::from_raw(123);
  //   let mut indexes = HashSet::new();
  //   indexes.insert(chunk_index);
  //   assert_eq!(
  //     prepare_event(
  //       &indexes,
  //       chunk_index,
  //       tilemap_entity
  //     ),
  //     None
  //   );
  // }
}
