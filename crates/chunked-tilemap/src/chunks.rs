use bevy::{prelude::*};
use bevy_ecs_tilemap::{prelude::TilemapId};

use crate::{spawn::{PrepareChunkEvent}, TilemapChunk, bundle::ChunkedTilemap};

pub fn update_current_chunk(
  mut q_tilemaps: Query<&mut ChunkedTilemap>,
){
  for mut tilemap in q_tilemaps.iter_mut(){
    let actually_current_chunk = get_chunk_at_position(
      tilemap.center,
      tilemap.chunk_size,
      tilemap.tile_size,
    );
    if tilemap.current_chunk != actually_current_chunk{
      tilemap.current_chunk = actually_current_chunk;
      info!("current chunk changed {}", tilemap.current_chunk);
    }
  }
}

pub fn spawn_chunks_around_current(
  mut ew_prepare_chunk: EventWriter<PrepareChunkEvent>,
  q_tilemaps: Query<(&ChunkedTilemap, Entity)>
){
  for (tilemap, entity) in q_tilemaps.iter(){
    for y in (tilemap.current_chunk.y - tilemap.range)..=(tilemap.current_chunk.y + tilemap.range) {
      for x in (tilemap.current_chunk.x - tilemap.range)..=(tilemap.current_chunk.x + tilemap.range) {
        let index = IVec2::new(x, y);
        if !tilemap.chunks.contains(&index){
          info!("Spawning chunk init event for {:#?}", entity);
          ew_prepare_chunk.send(PrepareChunkEvent {
            tilemap_entity: entity,
            chunk_index: index
          });
        }
      }
    }
  }
}

pub fn despawn_outbound_chunks(
  mut commands: Commands,
  q_chunks: Query<(&Transform, Entity), With<TilemapChunk>>,
  mut q_tilemaps: Query<(&mut ChunkedTilemap, &Children)>
){
  for (mut tilemap, children) in q_tilemaps.iter_mut(){
    for &children in children.iter(){
      if let Ok((transform, entity)) =  q_chunks.get(children){    
        let chunk_index = get_chunk_at_position(
          transform.translation.truncate(),
          tilemap.chunk_size,
          tilemap.tile_size
        );
        if (chunk_index.x-tilemap.current_chunk.x).abs() > tilemap.range|| (chunk_index.y-tilemap.current_chunk.y).abs() > tilemap.range {
          // info!("despawning chunk at {:?} - {}", chunk_index, chunk_index-current_chunk.0);
          tilemap.chunks.remove(&chunk_index);
          commands.entity(entity).despawn_recursive();
        }
      }
    }
  }
}

pub fn nest_chunks(
  mut commands: Commands,
  added: Query<(Entity,&TilemapId),  (Added<TilemapId>, Without<Parent>)>
){
  if added.iter().count() > 0 {
    println!("fixed nesting of {} items", added.iter().count());
    for (entity, tilemap) in added.iter(){
  
      commands.entity(tilemap.0).push_children(&[entity]);
    }
  }
}

pub fn get_chunk_at_position(position: Vec2, chunk_size: UVec2, tile_size: Vec2,)->IVec2{
  return IVec2::new(
    (position.x/(tile_size.x*chunk_size.x as f32)).round() as i32,
    (-position.y/(tile_size.y*chunk_size.y as f32)).round() as i32,
  )
}


pub fn get_chunk_center(
  chunk_size: UVec2,
  tile_size: Vec2,
  relative_position: IVec2,
)->Vec2{
  Vec2::new(
    -tile_size.x*((chunk_size.x-1)/2) as f32 + ((relative_position.x  as f32)*tile_size.x*chunk_size.x as f32),
    -tile_size.y*((chunk_size.y-1)/2) as f32 + ((-relative_position.y as f32)*tile_size.y*chunk_size.y as f32)
  )
}

pub fn local_tile_index_to_global(
  chunk_index: IVec2,
  chunk_size: UVec2,
  local_tile_index: IVec2 //  relative to chunk
)->IVec2{
  IVec2 {
    x: local_tile_index.x-((chunk_size.x) as f32/2.).floor() as i32 + chunk_index.x*(chunk_size.x as i32),
    y: -local_tile_index.y+((chunk_size.y) as f32/2.).floor() as i32  + chunk_index.y*(chunk_size.y as i32)
  }
}

#[cfg(test)]
mod test{
  use bevy::prelude::*;
  use rstest::rstest;

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
      , UVec2::new(10, 10), Vec2::new(32., 32.)
    ), IVec2::from(expected));
  }

  const tile_size: i32 =  32;
  const CHUNK_SIZE: UVec2 = UVec2{x: 5, y: 5};

  #[rstest]
  #[case(IVec2::new(0, 0), IVec2::new(0, 0), (-2, 2))]
  #[case(IVec2::new(0, 0), IVec2::new(2, 2), (0, 0))]
  #[case(IVec2::new(0, 0), IVec2::new(4, 4), (2, -2))]

  #[case(IVec2::new(1, 0), IVec2::new(0, 0), (3, 2))]
  #[case(IVec2::new(1, 0), IVec2::new(2, 2), (5, 0))]
  #[case(IVec2::new(1, 0), IVec2::new(4, 4), (7, -2))]

  fn local_tile_index_to_global_test(
    #[case] chunk_index: IVec2,
    #[case] tile_index: IVec2,
    #[case] expected: (i32, i32),
  ){
    let local_index = super::local_tile_index_to_global(
      chunk_index,
      CHUNK_SIZE,
      tile_index,
    );
    assert_eq!(local_index, IVec2::from(expected));
  }
}