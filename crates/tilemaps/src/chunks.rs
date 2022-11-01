use bevy::{prelude::*, utils::HashSet};

#[derive(Default)]
pub struct ChunkManager{
  pub chunks: HashSet<IVec2>
}

pub fn get_chunk_center(
  chunk_size: i32,
  relative_position: IVec2,
  tile_size: i32,
)->Vec2{
  Vec2::new(
    (-tile_size*((chunk_size-1)/2)) as f32 + ((relative_position.x)*tile_size*chunk_size) as f32,
    (-tile_size*((chunk_size-1)/2)) as f32 + ((-relative_position.y)*tile_size*chunk_size) as f32
  )
}

#[cfg(test)]
mod test{
  use bevy::prelude::*;
  use rstest::rstest;
  use crate::chunks::get_chunk_center;

  #[rstest]
  #[case(1, (0, 0), (0., 0.))]
  #[case(2, (0, 0), (-0., -0.))]
  #[case(3, (0, 0), (-32., -32.))]
  #[case(4, (0, 0), (-32., -32.))]
  #[case(5, (0, 0), (-64., -64.))]
  #[case(1, (-1, 0), (-32., 0.))]
  #[case(1, (1, 1), (32., 32.))]
  #[case(1, (0, 1), (0., 32.))]

  // #[case(3, (0, 1), (-32., -32.))]
  fn get_chunk_center_test(
    #[case] chunk_size: i32,
    #[case] relative_position: (i32, i32),
    #[case] expected: (f32, f32),
  ){
    assert_eq!(get_chunk_center(
      chunk_size,
      IVec2::from(relative_position),
      32
    ), Vec2::from(expected));
  }
}