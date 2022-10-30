use bevy::prelude::*;
use bevy_ecs_tilemap::{tiles::{TileStorage, TileTexture, TilePos, TileVisible}, prelude::{get_tile_neighbors, TilemapType, NeighborDirection, Neighbors}};
use rand::{thread_rng, Rng};
use perlin2d::PerlinNoise2D;

use crate::{GameStates, GroundTilemap, OverGroundTilemap, WorldNoise};

pub struct InitStatePlugin;

impl Plugin for InitStatePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system_set(
        SystemSet::on_enter(GameStates::Init)
          .with_system(init_tilemaps)
      ).add_system_set(
        SystemSet::on_enter(GameStates::Init)
          .with_system(generate_grass)
          .after(init_tilemaps)
      )
      .add_system_set(
        SystemSet::on_enter(GameStates::Init)
          .with_system(generate_sand_spots)
          .with_system(generate_roads)
          .with_system(generate_forests)
          .after(generate_grass)
      )
      .add_system_set(
        SystemSet::on_enter(GameStates::Init)
          .with_system(rounding_sand_corners)
          .after(generate_sand_spots)
      )
      .add_system_set(
        SystemSet::on_update(GameStates::Init)
          .with_system(finish_init)
      );
  }
}

fn init_tilemaps(
  mut commands: Commands
){
  info!("generating tilemaps");
  
}

fn generate_grass(
  q_tilestorage: Query<&TileStorage, With<GroundTilemap>>,
  mut tile_query: Query<&mut TileTexture>,
){
  let mut rng = thread_rng();
  let dark_gras_tiles = [3, 5, 7, 11, 13, 15, 17, 19, 21, 23, 25, 27];
  // let gt = rng.gen_range(0..dark_gras_tiles.len());
  info!("generating grass");
  for tilestorage in q_tilestorage.iter(){
    for x in 0..tilestorage.size.x {
      for y in 0..tilestorage.size.y {
        let tile = tilestorage.get(&TilePos{x, y}).unwrap();
        if let Ok(mut tile_texture) = tile_query.get_mut(tile) {
          tile_texture.0 = dark_gras_tiles[rng.gen_range(0..dark_gras_tiles.len())]+1;
          //tile_texture.0 = rng.gen_range(0..45);
        }
      }
    }
  }
}

fn generate_sand_spots(
  world_perlin: Res<WorldNoise>,
  q_tilestorage: Query<&TileStorage, With<GroundTilemap>>,
  mut tile_query: Query<&mut TileTexture>,
){
  info!("generating sands");
  let mut rng = thread_rng();
  let tilestorage = q_tilestorage.get_single().unwrap();

  let perlin = &world_perlin.0;
  for tilestorage in q_tilestorage.iter(){
    for x in 0..tilestorage.size.x {
      for y in 0..tilestorage.size.y {
        let noise = perlin.get_noise(x.into(), y.into());
        if noise < 1. {
          let tile = tilestorage.get(&TilePos{x, y}).unwrap();
          let mut tile_texture = tile_query.get_mut(tile).unwrap();
          tile_texture.0 = 30;
        }
        // println!("{x}:{y} => {noise}");
      }
    }
  }

  // for x in 0..
  // for _ in 0..rng.gen_range(10..40) {
  //   let x = rng.gen_range(0..tilestorage.size.x);
  //   let y = rng.gen_range(0..tilestorage.size.y);
  //   let tile = tilestorage.get(&TilePos{x, y}).unwrap();
  //   let mut tile_texture = tile_query.get_mut(tile).unwrap();
  //   tile_texture.0 = 30;

  //   let neighboring_entities = get_tile_neighbors(&TilePos { x, y }, tilestorage, &TilemapType::Square {
  //     diagonal_neighbors: true,
  //   });
  //   // let mut sand_tiles_que = sand_tiles.clone();

  //   for entity in neighboring_entities.into_iter(){
      
  //     let mut tile_texture = tile_query.get_mut(entity).unwrap();
  //     // let new_tile = sand_tiles_que.pop().unwrap();
  //     tile_texture.0 = 30;
  //     // if tile_texture.0 != new_tile && (tile_texture.0 == 30 || sand_tiles.contains(&tile_texture.0)){
  //     // } else {
  //     //   tile_texture.0 = new_tile;
  //     // }
  //   }
  // }
}


fn generate_roads(){
  let mut rng = thread_rng();
  let c_roads = rng.gen_range(1..3);
  info!("generating {c_roads} roads");
  for _ in 0..c_roads{
    let c_road = rng.gen_range(1..3);
  }
}

fn rounding_sand_corners(
  mut tile_query: Query<(&mut TileTexture, &TilePos)>,
  q_tilestorage: Query<&TileStorage, With<GroundTilemap>>,
){
  info!("rounding_sand_corners");
  let mut update_tiles = Vec::<(&Entity, i32)>::new();
  // let mut east_tiles = Vec::<&Entity>::new();

  let sand_tiles = vec![34, 10, 40, 38, 42, 44, 36, 32, 30];

  for tilestorage in q_tilestorage.iter(){
    for entity in tilestorage.iter(){
      if let Some(entity) = entity {
        let (tile_texture, tile_position) = tile_query.get_mut(*entity).unwrap();
        if tile_texture.0 == 30 {
          let neighboring_entities = get_tile_neighbors(&tile_position, tilestorage, &TilemapType::Square {
            diagonal_neighbors: false,
          });

          let mut bitmask: i32 = 0;
          let base: i32 = 2;
          for (index, neighboring_entity) in neighboring_entities.into_iter().enumerate(){
            if let Ok((neighboring_tile_texture, _)) = tile_query.get(neighboring_entity){
              if !sand_tiles.contains(&neighboring_tile_texture.0) {
                bitmask += base.pow(index as u32);
              }
            }
          }
          if bitmask > 0 {
            update_tiles.push((entity, bitmask));
          }

        }

      }
    }
  }

  for (update_entity, bitmask) in update_tiles{
    if let Ok((mut tile_texture, tile_position)) = tile_query.get_mut(*update_entity){
      tile_texture.0 = match bitmask {
        1 => 32,
        2 => 44,
        3 => 36,
        4 => 38,
        6 => 42,
        8 => 10,
        9 => 34,
        12 => 40,
        _ => {
          println!("unknown bitmask: {bitmask}");
          30
        }
      };
    };
  }
}

fn generate_forests(
  world_perlin: Res<WorldNoise>,
  mut tile_query: Query<(&mut TileTexture, &mut TileVisible)>,
  q_tilestorage: Query<&TileStorage, With<OverGroundTilemap>>,
){
  info!("generating forests");
  let mut rng = thread_rng();
  let perlin = &world_perlin.0;

  for tilestorage in q_tilestorage.iter(){
    for x in 0..tilestorage.size.x {
      for y in 0..tilestorage.size.y {
        let tile = tilestorage.get(&TilePos{x, y}).unwrap();
        if let Ok((_, mut tile_visible)) = tile_query.get_mut(tile) {
          tile_visible.0 = false;
        }
      }
    }
    for x in 0..tilestorage.size.x {
      for y in 0..tilestorage.size.y {
        let noise = perlin.get_noise(x.into(), y.into());
        if noise > 4. {
          let tile = tilestorage.get(&TilePos{x, y}).unwrap();
          let (mut tile_texture, mut tile_visible) = tile_query.get_mut(tile).unwrap();
          tile_texture.0 = noise as u32 - 4;
          tile_visible.0 = true;
        }
        // println!("{x}:{y} => {noise}");
      }
    }
  }
}

fn generate_stand_alone_trees(
  q_tilestorage: Query<&TileStorage, With<OverGroundTilemap>>,
  mut tile_query: Query<(&mut TileTexture, &mut TileVisible)>,
){
  let mut rng = thread_rng();

  for tilestorage in q_tilestorage.iter(){
    let c_trees = rng.gen_range(5..480);
    for x in 0..tilestorage.size.x {
      for y in 0..tilestorage.size.y {
        let tile = tilestorage.get(&TilePos{x, y}).unwrap();
        if let Ok((_, mut tile_visible)) = tile_query.get_mut(tile) {
          tile_visible.0 = false;
        }
      }
    }
    
    for _ in 0..c_trees {
      let x = rng.gen_range(0..tilestorage.size.x);
      let y = rng.gen_range(0..tilestorage.size.y);
      let tile = tilestorage.get(&TilePos{x, y}).unwrap();
      if let Ok((mut tile_texture, mut tile_visible)) = tile_query.get_mut(tile) {
        tile_texture.0 = rng.gen_range(0..10);
        tile_visible.0 = true;
      }
    }
  };

  info!("generating trees");
}

fn finish_init(
  mut app_state: ResMut<State<GameStates>>,
){
  app_state.set(GameStates::Game).unwrap();
}