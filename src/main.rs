use bevy::{prelude::*, render::texture::ImageSettings};
use bevy_ecs_tilemap::{prelude::{TilemapSize, TilemapId, TilemapTileSize, TilemapTexture, get_tilemap_center_transform, ArrayTextureLoader, TilemapArrayTexture, TilemapGridSize,
  }  , tiles::{TileStorage, TilePos, TileBundle, TileVisible, TileTexture}, TilemapBundle, TilemapPlugin};
use bevy_tilemap_test::{GameStates, init::InitStatePlugin, game::GameStatePlugin, GroundTilemap, OverGroundTilemap, player::{PlayerBundle, PlayerAction}, TextureAtlases, AssetsLoading, load::LoadStatePlugin, DefaultCamera};
use bevy_editor_pls::{EditorPlugin};
use leafwing_input_manager::prelude::InputManagerPlugin;


fn main() {
  let mut app = App::new();
  app
    .add_startup_system(startup)
    .init_resource::<AssetsLoading>()
    .init_resource::<TextureAtlases>()
    .add_plugins(DefaultPlugins)
    .insert_resource(ImageSettings::default_nearest())
    .add_state(GameStates::Load)
    .add_plugin(InputManagerPlugin::<PlayerAction>::default())
    .add_plugin(TilemapPlugin)
    .add_plugin(GameStatePlugin)
    .add_plugin(LoadStatePlugin)
    .add_plugin(EditorPlugin)
    .add_plugin(InitStatePlugin);
  app.run();
  println!("Hello, world!");
}


fn startup(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  windows: Res<Windows>,
  
  
  
){
  
  let window = windows.get_primary().unwrap();
  let tilemap_size = TilemapSize {
    x: 20,
    y: 20

    // x: window.width() as u32 /20,
    // y: window.height() as u32 /20
  };
  
  commands.spawn_bundle(Camera2dBundle::default()).insert(DefaultCamera);

  
  let ground_tilemap_entity = commands.spawn().id();
  let mut tile_storage = TileStorage::empty(tilemap_size);

  for x in 0..tilemap_size.x {
    for y in 0..tilemap_size.y {
      let tile =  if (x+y) % 2 > 0 {
        0
      } else {
        3
      };
      let tile_pos = TilePos { x, y};
      let tile_entity = commands
        .spawn()
        .insert_bundle(TileBundle {
          position: tile_pos,
          texture: TileTexture(tile),
          // texture: TileTexture(dark_gras_tiles[gt as usize]),
          tilemap_id: TilemapId(ground_tilemap_entity),
          ..Default::default()
        })
        .id();
      tile_storage.set(&tile_pos, tile_entity);
      commands.entity(ground_tilemap_entity).push_children(&[tile_entity]);
    }
  }
  let tile_size = TilemapTileSize { x: 32.0, y: 32.0 };
  let grid_size = TilemapGridSize { x: 32.0, y: 32.0};

  commands
    .entity(ground_tilemap_entity)
    .insert_bundle(TilemapBundle {
      grid_size,
      size: tilemap_size,
      storage: tile_storage,
      texture: TilemapTexture::Single(asset_server.load("images/grass_tiles.png")),
      tile_size,
      transform: get_tilemap_center_transform(&tilemap_size, &grid_size, 0.0),
      ..Default::default()
    })
    .insert_bundle((
      GroundTilemap,
      Name::new("Ground Tilemap")
    ));
    

  let overground_tilemap_entity = commands.spawn().id();
  let mut tile_storage = TileStorage::empty(tilemap_size);

  for x in 0..tilemap_size.x {
    for y in 0..tilemap_size.y {
      let tile_pos = TilePos { x, y};
      let tile_entity = commands
        .spawn()
        .insert_bundle(TileBundle {
          position: tile_pos,
          visible: TileVisible(false),
          // texture: TileTexture(dark_gras_tiles[gt as usize]),
          tilemap_id: TilemapId(overground_tilemap_entity),
          ..Default::default()
        })
        .id();
      tile_storage.set(&tile_pos, tile_entity);
      commands.entity(overground_tilemap_entity).push_children(&[tile_entity]);
    }
  }

  commands
    .entity(overground_tilemap_entity)
    .insert_bundle(TilemapBundle {
      grid_size,
      size: tilemap_size,
      storage: tile_storage,
      texture: TilemapTexture::Single(asset_server.load("images/tree_tiles.png")),
      tile_size,
      transform: get_tilemap_center_transform(&tilemap_size, &grid_size, 10.0),
      ..Default::default()
    })
    .insert_bundle((
      OverGroundTilemap,
      Name::new("Overground Tilemap")
    ));
    



  // array_texture_loader.add(TilemapArrayTexture {
  //     texture: TilemapTexture::Single(asset_server.load("images/grass_tiles.png")),
  //     tile_size,
  //     ..Default::default()
  // });
}