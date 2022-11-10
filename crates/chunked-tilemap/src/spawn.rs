use bevy::{prelude::*, utils::Instant};
use bevy_ecs_tilemap::{prelude::{TilemapSize, TilemapGridSize, TilemapTileSize, TilemapTexture, TilemapId}, tiles::{TileStorage, TileBundle}, TilemapBundle};

use crate::{TilemapChunk, bundle::{ChunkedTilemap}, chunks::get_chunk_center};

#[derive(Debug, PartialEq)]
pub struct PrepareChunkEvent{
  pub tilemap_entity: Entity,
  pub chunk_index: IVec2,
}
pub struct SpawnChunkEvent{
  pub tilemap_entity: Entity,
  pub chunk_index: IVec2,
  pub bundles: Vec<TileBundle>,
} 

pub fn spawn_chunk(
  mut er_spawn_chunk: EventReader<SpawnChunkEvent>,
  mut commands: Commands,
  mut q_tilemaps: Query<&mut ChunkedTilemap>,
  #[cfg(feature = "dev-labels")] asset_server: Res<AssetServer>,
){
  
  for event in er_spawn_chunk.iter(){
    info!("spawning chunk {:?}", event.chunk_index);
    let start = Instant::now();
    if let Ok(mut tilemap) = q_tilemaps.get_mut(event.tilemap_entity){

      let tilemap_entity = commands.spawn().id();
      let tilemap_size = TilemapSize::from(tilemap.chunk_size);
      let tile_size = TilemapTileSize::from(tilemap.tile_size);
      let grid_size = TilemapGridSize::from(tilemap.tile_size);
      let mut bundles = event.bundles.clone();
      for bundle in bundles.iter_mut(){
        bundle.tilemap_id = TilemapId(tilemap_entity);
      }
      commands.spawn_batch(bundles);

      let transform = Transform::from_translation(get_chunk_center(
        tilemap.chunk_size,
        tilemap.tile_size,
        event.chunk_index
      ).extend(0.));
      
      // debug!(target: "chunk spawner", "spawning chunk {:?} on position {:?}", event.chunk_index, transform.translation);
  
      let chunk = commands
        .entity(tilemap_entity)
        .insert_bundle(TilemapBundle {
          grid_size,
          size: tilemap_size,
          storage: TileStorage::empty(tilemap_size),
          texture: TilemapTexture::Single(tilemap.texture_handle.clone()),
          tile_size,
          transform,
          ..Default::default()
        })
        .insert(Name::new(format!("Chunk {}:{}", event.chunk_index.x, event.chunk_index.y)))
        .insert(TilemapChunk(event.chunk_index))
        .id();
      #[cfg(feature = "dev-labels")]{
        let font = asset_server.load("../../../assets/fonts/FiraSans-Bold.ttf");
        let text_style = TextStyle {
            font,
            font_size: 20.0,
            color: Color::WHITE,
        };
        let text_alignment = TextAlignment::CENTER;
        let label = commands.spawn_bundle(Text2dBundle {
          text: Text::from_section(format!("{}:{}", event.chunk_index.x, event.chunk_index.y), text_style.clone())
            .with_alignment(text_alignment),
          transform: Transform::from_xyz(
            tile_size.x * (event.chunk_size.x-1) as f32 / 2.,
            tile_size.y * (event.chunk_size.y-1) as f32 / 2.,
            10.
          ),
          ..default()
        }).id();
        commands.entity(chunk).push_children(&[label]);
      }
      
      commands.entity(event.tilemap_entity).push_children(&[chunk]);
      tilemap.chunks.insert(event.chunk_index);
    }
    debug!("chunk spawn took {:?}", start.elapsed());
  }
}