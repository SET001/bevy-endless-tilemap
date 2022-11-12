use bevy::{prelude::*};

use crate::{TilemapChunk, bundle::ChunkedTilemap};

pub fn despawn_outrange_chunks(
  mut commands: Commands,
  q_chunks: Query<(&Transform, Entity, &TilemapChunk)>,
  mut q_tilemaps: Query<(&mut ChunkedTilemap, &Children)>
){
  for (mut tilemap, children) in q_tilemaps.iter_mut(){
    for &children in children.iter(){
      if let Ok((_, entity, chunk)) =  q_chunks.get(children){    
        let range = (chunk.0 - tilemap.current_chunk).abs();
        // if (chunk_index.x-tilemap.current_chunk.x).abs() > tilemap.range|| (chunk_index.y-tilemap.current_chunk.y).abs() > tilemap.range {
        if range.x > tilemap.range || range.y > tilemap.range {
          debug!("despawning chunk at {:?}", chunk.0);
          tilemap.chunks.remove(&chunk.0);
          commands.entity(entity).despawn_recursive();
        }
      }
    }
  }
}