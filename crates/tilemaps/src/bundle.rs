use bevy::{prelude::*, utils::HashSet};

#[derive(Default, Component, Clone)]
pub struct ChunkedTilemap{
  pub chunk_size:  IVec2,
  pub tile_size: IVec2,
  pub range: i32,
  pub center: Vec2,
  pub current_chunk: IVec2,
  pub chunks: HashSet<IVec2>
}

#[derive(Default, Component)]
pub struct ChunkedTilemapCenter(pub Vec2);

#[derive(Bundle, Default)]
pub struct ChunkedTilemapBundle{
  pub chunked_tilemap: ChunkedTilemap,
  pub name: Name,
  #[bundle]
  pub spatial: SpatialBundle
}