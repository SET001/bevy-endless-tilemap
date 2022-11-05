use bevy::{prelude::*, utils::HashSet};

use crate::{spawn::SpawnChunkEvent, TilemapChunk};

// pub struct Chunk{
//   position: Vec2,
//   index: IVec2
// }


#[derive(Default)]
pub struct ChunkManager{
  pub chunks: HashSet<IVec2>
}

// pub struct ChunkedTilemaps{

// }
#[derive(Default)]
pub struct ChunkedTilemapConfig{
  pub chunk_size:  IVec2,
  pub tile_size: IVec2,
  pub center: Vec2
}

#[derive(Default)]
pub struct CurrentChunk(pub IVec2);

pub fn update_current_chunk(
  mut current_chunk: ResMut<CurrentChunk>,
  config: Res<ChunkedTilemapConfig>
){
  let actually_current_chunk = get_chunk_at_position(
    config.center,
    config.chunk_size,
    config.tile_size,
  );
  if current_chunk.0 != actually_current_chunk{
    current_chunk.0 = actually_current_chunk;
    info!("current chunk changed {}", current_chunk.0);
  }
}

pub fn spawn_chunks_around_current(
  mut ew_spawn_chunk: EventWriter<SpawnChunkEvent>,
  chunk_manager: Res<ChunkManager>,
  current_chunk: Res<CurrentChunk>,
  chunked_config: Res<ChunkedTilemapConfig>
){
  let chunk_size = chunked_config.chunk_size;
  let tile_size = chunked_config.tile_size;
  for y in (current_chunk.0.y - 2)..=(current_chunk.0.y + 2) {
    for x in (current_chunk.0.x - 2)..=(current_chunk.0.x + 2) {
      let index = IVec2::new(x, y);
      if !chunk_manager.chunks.contains(&index){
        ew_spawn_chunk.send(SpawnChunkEvent {
          chunk_size: chunk_size,
          tile_size: tile_size,
          chunk_possition: get_chunk_center(
            chunk_size,
            tile_size,
            IVec2::new(x, y)
          ),
          chunk_index: index
        });
      }
    }
  }
}

pub fn despawn_outbound_chunks(
  mut commands: Commands,
  mut chunk_manager: ResMut<ChunkManager>,
  q_chunks: Query<(&TilemapChunk, &Transform, Entity)>,
  current_chunk: Res<CurrentChunk>,
  chunked_config: Res<ChunkedTilemapConfig>
){
  for (chunk, transform, entity) in q_chunks.iter(){
    let chunk_index = get_chunk_at_position(
      transform.translation.truncate(),
      chunked_config.chunk_size,
      chunked_config.tile_size
    );
    if (chunk_index.x-current_chunk.0.x).abs() >2 || (chunk_index.y-current_chunk.0.y).abs() >2 {
      // info!("despawning chunk at {:?} - {}", chunk_index, chunk_index-current_chunk.0);
      chunk_manager.chunks.remove(&chunk_index);
      commands.entity(entity).despawn_recursive();
    }
  }
}

pub fn get_chunk_at_position(position: Vec2, chunk_size: IVec2, tile_size: IVec2,)->IVec2{
  return IVec2::new(
    (position.x/(tile_size.x*chunk_size.x) as f32).round() as i32,
    (-position.y/(tile_size.y*chunk_size.y) as f32).round() as i32,
  )
}


pub fn get_chunk_center(
  chunk_size: IVec2,
  tile_size: IVec2,
  relative_position: IVec2,
)->Vec2{
  Vec2::new(
    (-tile_size.x*((chunk_size.x-1)/2)) as f32 + ((relative_position.x)*tile_size.x*chunk_size.x) as f32,
    (-tile_size.y*((chunk_size.y-1)/2)) as f32 + ((-relative_position.y)*tile_size.y*chunk_size.y) as f32
  )
}

#[cfg(test)]
mod test{
  use bevy::prelude::*;
  use rstest::rstest;
  use crate::chunks::get_chunk_center;

  // #[rstest]
  // #[case(1, (0, 0), (0., 0.))]
  // #[case(2, (0, 0), (-0., -0.))]
  // #[case(3, (0, 0), (-32., -32.))]
  // #[case(4, (0, 0), (-32., -32.))]
  // #[case(5, (0, 0), (-64., -64.))]
  // #[case(1, (-1, 0), (-32., 0.))]
  // // #[case(1, (1, 1), (32., 32.))]
  // // #[case(1, (0, 1), (0., 32.))]

  // // #[case(3, (0, 1), (-32., -32.))]
  // fn get_chunk_center_test(
  //   #[case] chunk_size: i32,
  //   #[case] relative_position: (i32, i32),
  //   #[case] expected: (f32, f32),
  // ){
  //   assert_eq!(get_chunk_center(
  //     chunk_size,
  //     IVec2::from(relative_position),
  //     32
  //   ), Vec2::from(expected));
  // }

  #[rstest]
  #[case((0., 0.), (10., 10.), (5., 0.), true)]
  #[case((0., 0.), (10., 10.), (-6., 5.), false)]
  #[case((0., 0.), (10., 10.), (-5., 0.), true)]
  #[case((0., 0.), (10., 10.), (7., 7.), false)]
  #[case((0., 0.), (10., 10.), (5., 0.), true)]
  #[case((0., 0.), (10., 10.), (50., 5.), false)]
  #[case((0., 0.), (10., 10.), (-50., 5.), false)]

  #[case((0., 0.), (10., 10.), (-50., 5.), false)]
  #[case((0., 0.), (10., 10.), (5., 50.), false)]

  #[case((-1., -1.), (10., 10.), (3., 3.), true)]
  #[case((-1., -1.), (10., 10.), (5., 0.), false)]
  #[case((-1., -1.), (10., 10.), (-50., 1.), false)]
  fn is_point_in_rect_test(
    #[case] rect_pos: (f32, f32),
    #[case] rect_size: (f32, f32),
    #[case] point: (f32, f32),
    #[case] expected: bool,
  ){
    assert_eq!(
      crate::is_point_in_rect(
        Vec2::from(rect_pos),
        Vec2::from(rect_size),
        Vec2::from(point),
      ),
      expected
    );
  }

  #[rstest]
  #[case((0., 0.), (0, 0))]
  #[case((10., 10.), (0, 0))]
  #[case((320., 0.), (1, 0))]
  #[case((-320., 0.), (-1, 0))]
  fn get_chunk_at_position_test(
    #[case] position: (f32, f32),
    #[case] expected: (i32, i32),
  ){
    assert_eq!(super::get_chunk_at_position(
      Vec2::from(position)
      , IVec2::new(10, 10), IVec2::new(32, 32)
    ), IVec2::from(expected));
  }
}