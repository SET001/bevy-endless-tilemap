use bevy::prelude::*;
use bevy_ecs_tilemap::{prelude::{TilemapSize, TilemapGridSize, TilemapTileSize, TilemapTexture, TilemapId}, tiles::{TileStorage, TileTexture, TilePos, TileBundle}, TilemapBundle};

use crate::{TilemapChunk, bundle::ChunkedTilemap};

pub struct SpawnChunkEvent{
  pub tilemap_entity: Entity,
  pub chunk_possition: Vec2,
  pub chunk_index: IVec2,
  pub chunk_size: IVec2,
  pub tile_size: IVec2
}

pub fn spawn_chunk(
  mut er_spawn_chunk: EventReader<SpawnChunkEvent>,
  mut commands: Commands,
  mut q_tilemaps: Query<&mut ChunkedTilemap>,
  asset_server: Res<AssetServer>,
){
  for event in er_spawn_chunk.iter(){
    if let Ok(mut tilemap) = q_tilemaps.get_mut(event.tilemap_entity){

      let ground_tilemap_entity = commands.spawn().id();
      let tilemap_size = TilemapSize{
        x: event.chunk_size.x as u32,
        y: event.chunk_size.y as u32,
      };
      let tile_size = TilemapTileSize {
        x: event.tile_size.x as f32,
        y: event.tile_size.y as f32
      };
      let grid_size = TilemapGridSize { x: 32.0, y: 32.0};
  
      let mut tile_storage = TileStorage::empty(tilemap_size);
  
      
      let tile =  if (event.chunk_index.x+event.chunk_index.y).abs() % 2 > 0 {
        0
      } else {
        3
      };
  
      for y in 0..tilemap_size.y {
        for x in 0..tilemap_size.x {
          // let tile =  if (y+x) % 2 > 0 {
          //   0
          // } else {
          //   3
          // };
          let tile_pos = TilePos { x, y};
            let tile_entity = commands
              .spawn()
              .insert_bundle(TileBundle {
                position: tile_pos,
                texture: TileTexture(tile),
                tilemap_id: TilemapId(ground_tilemap_entity),
                ..Default::default()
              })
              .id();
            tile_storage.set(&tile_pos, tile_entity);
            commands.entity(ground_tilemap_entity).push_children(&[tile_entity]);
        }
      }
      let transform = Transform::from_translation(event.chunk_possition.extend(0.));
      info!(target: "chunk spawner", "spawning chunk {:?} on position {:?}", event.chunk_index, transform.translation);
      let font = asset_server.load("../../../assets/fonts/FiraSans-Bold.ttf");
      let text_style = TextStyle {
          font,
          font_size: 20.0,
          color: Color::WHITE,
      };
  
      let chunk = commands
        .entity(ground_tilemap_entity)
        .insert_bundle(TilemapBundle {
          grid_size,
          size: tilemap_size,
          storage: tile_storage,
          texture: TilemapTexture::Single(asset_server.load("../../../assets/images/grass_tiles.png").clone()),
          tile_size,
          transform,
          ..Default::default()
        })
        .insert(Name::new(format!("Chunk {}:{}", event.chunk_index.x, event.chunk_index.y)))
        .insert(TilemapChunk)
        .with_children(|parent|{
  
          let text_alignment = TextAlignment::CENTER;
          parent.spawn_bundle(Text2dBundle {
            text: Text::from_section(format!("{}:{}", event.chunk_index.x, event.chunk_index.y), text_style.clone())
              .with_alignment(text_alignment),
            transform: Transform::from_xyz(
              tile_size.x * (event.chunk_size.x-1) as f32 / 2.,
              tile_size.y * (event.chunk_size.y-1) as f32 / 2.,
              10.
            ),
            ..default()
          });
        }).id();
        // .insert_bundle((
        //   // GroundTilemap,
        //   Name::new("Ground Tilemap")
        // ));
      
      commands.entity(event.tilemap_entity).push_children(&[chunk]);
      tilemap.chunks.insert(event.chunk_index);
    }

    
  }
}