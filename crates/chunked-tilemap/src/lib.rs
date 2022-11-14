pub mod chunks;
pub mod spawn_chunk;
pub mod spawn_around;
pub mod despawn_outrange;
pub mod bundle;
pub mod fill_chunk;

use bevy::{prelude::{Plugin, Vec2, IVec2, Component, CoreStage, ParallelSystemDescriptorCoercion}};
use bevy_ecs_tilemap::TilemapPlugin;
use chunks::{update_current_chunk, nest_chunks};
use despawn_outrange::despawn_outrange_chunks;
use spawn_chunk::{SpawnChunkEvent, spawn_chunk, PrepareChunkEvent};
use spawn_around::spawn_chunks_around_current;
use fill_chunk::{fill_chunk, FillChunkEvent};



pub struct ChunkedTilemapPlugin;

impl Plugin for ChunkedTilemapPlugin{
  fn build(&self, app: &mut bevy::prelude::App) {
    app
      .add_event::<SpawnChunkEvent>()
      .add_event::<PrepareChunkEvent>()
      .add_event::<FillChunkEvent>()
      .add_plugin(TilemapPlugin)
      .add_system(update_current_chunk)
      .add_system(spawn_chunks_around_current)
      .add_system(spawn_chunk)
      .add_system(fill_chunk)
      .add_system(nest_chunks.after(fill_chunk))
      .add_system(despawn_outrange_chunks);
  }
}


#[derive(Component)]
pub struct TilemapChunk(pub IVec2);

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