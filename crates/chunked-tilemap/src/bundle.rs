use bevy::{prelude::*, utils::HashSet};

#[derive(Default, Component, Clone, Reflect)]
#[reflect(Component)]
pub struct ChunkedTilemap{
  pub chunk_size:  UVec2,
  pub tile_size: Vec2,
  pub range: i32,
  pub center: Vec2,
  pub current_chunk: IVec2,
  pub chunks: HashSet<IVec2>,
  pub texture_handle: Handle<Image>
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