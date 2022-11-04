use bevy::{prelude::{Plugin, ResMut, Vec2, IVec2}, sprite::Rect};
use bevy_ecs_tilemap::TilemapPlugin;
use chunks::{ChunkManager, CurrentChunk, spawn_chunks_around_current, update_current_chunk};
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
      .init_resource::<CurrentChunk>()
      
      .add_plugin(TilemapPlugin)
      .add_system(update_current_chunk)
      .add_system(spawn_chunk)
      .add_system(spawn_chunks_around_current);
  }
}

pub struct Chunk{
  index: IVec2,
  position: Vec2
}

pub fn is_chunk_updated(
  current_chunk: Chunk,
  current_position: Vec2
) -> Option<IVec2> {
  // if !current_position in current_chunk{
    // current_position.dot(rhs)
  // };
  None
}

pub fn is_point_in_rect(
  rect_pos: Vec2,
  rect_size: Vec2,
  point: Vec2
)->bool{
  let max = rect_pos+rect_size/2.;
  
  // println!("max {max}");
  // println!("max.dot(max) {}", max.dot(max));
  // println!("max.dot(point) {}", max.dot((point+rect_pos).abs()));

  // println!("max.dot(max) {}", min.dot(max));
  // println!("max.dot(point) {}", max.dot(point));
  // true
  max.dot(max)/2.>=max.dot((point+rect_pos).abs())
  // let a = rect_pos-rect_size/2.;
  // let b = a*Vec2::new(1., -1.);
  // let c = rect_pos+rect_size/2.;
  // let ab = a-b;
  // let bc = c-c;
  // let bm = point-b;

  // println!("a {a}, b {b}, c {c}");

  // println!("{ab}, {cb}");

  // 0.<=bc.dot(bm) && bc.dot(bm)<=bc.dot(bc) && ab.dot(bm)<=ab.dot(ab)
}