use bevy::{prelude::*, DefaultPlugins, sprite::MaterialMesh2dBundle, input::mouse::MouseMotion, asset::AssetServerSettings};
use bevy_ecs_tilemap::{tiles::{TilePos, TileTexture, TileBundle}};
use bevy_editor_pls::EditorPlugin;
use chunked_tilemap::{ChunkedTilemapPlugin, bundle::{ChunkedTilemapBundle, ChunkedTilemap}, spawn::{PrepareChunkEvent, SpawnChunkEvent}};

const CHUNK_SIZE: u32 = 15;
const TILE_SIZE: f32 = 32.;

#[derive(Default)]
pub struct TilemapLayers{
  pub ground: Option<Entity>,
  pub trees: Option<Entity>
}
fn main() {
  let mut app = App::new();
  app
  .init_resource::<TilemapLayers>()
  .insert_resource(AssetServerSettings {
    watch_for_changes: true,
    asset_folder: format!("{}/assets", std::env::current_dir().unwrap().to_str().unwrap()),
    ..Default::default()
  })
  .add_plugins(DefaultPlugins)
  .add_plugin(ChunkedTilemapPlugin)
  .add_plugin(EditorPlugin)
  .add_startup_system(startup)
  .add_system(move_camera)
  .add_system_to_stage(CoreStage::PreUpdate, init_ground_chunk)
  .add_system_to_stage(CoreStage::PreUpdate, init_trees_chunk);
    
  app.run();
}

#[derive(Component)]
struct DefaultCamera;

#[derive(Component)]
struct CenterMarker;

fn startup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  mut tilemap_layers: ResMut<TilemapLayers>,
  asset_server: Res<AssetServer>,
){
  commands.spawn_bundle(Camera2dBundle::default()).insert(DefaultCamera);
  commands.spawn_bundle(MaterialMesh2dBundle {
    mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
    transform: Transform::default().with_scale(Vec3::splat(TILE_SIZE*(CHUNK_SIZE as f32))).with_translation(Vec3::new(0., 0., 20.)),
    material: materials.add(ColorMaterial::from(Color::rgba(0.5, 0.0, 0.5, 0.5))),
    ..default()
  }).insert(CenterMarker);


  tilemap_layers.ground = Some(commands.spawn_bundle(ChunkedTilemapBundle{
    name: Name::new("Ground layer"),
    chunked_tilemap: ChunkedTilemap{
      chunk_size: UVec2::new(CHUNK_SIZE, CHUNK_SIZE),
      tile_size: Vec2::new(TILE_SIZE, TILE_SIZE),
      range: 1,
      texture_handle: asset_server.load("images/grass_tiles.png"),
      ..Default::default()
    },
    ..Default::default()
  }).id());

  tilemap_layers.trees = Some(commands.spawn_bundle(ChunkedTilemapBundle{
    name: Name::new("Trees layer"),
    chunked_tilemap: ChunkedTilemap{
      chunk_size: UVec2::new(CHUNK_SIZE, CHUNK_SIZE),
      tile_size: Vec2::new(TILE_SIZE, TILE_SIZE),
      range: 1,
      texture_handle: asset_server.load("images/tree_tiles.png"),
      ..Default::default()
    },
    spatial: SpatialBundle{
      transform: Transform::from_xyz(0., 0., 10.),
      ..Default::default()
    },
    ..Default::default()
  }).id());
}

fn move_camera(
  mut q_camera: Query<&mut Transform, With<DefaultCamera>>,
  mut q_center_marker: Query<&mut Transform, (With<CenterMarker>, Without<DefaultCamera>)>,
  mut motion_evr: EventReader<MouseMotion>,
  mut q_tilemaps: Query<&mut ChunkedTilemap>,
  buttons: Res<Input<MouseButton>>,
){

  if let Ok(mut camera_transform) = q_camera.get_single_mut(){
    if buttons.pressed(MouseButton::Left){
      for event in motion_evr.iter(){
        camera_transform.translation.x -= event.delta.x;
        camera_transform.translation.y += event.delta.y;

        q_center_marker.get_single_mut().expect("no center marker").translation = camera_transform.translation;
        for mut tilemap in q_tilemaps.iter_mut(){
          tilemap.center = camera_transform.translation.truncate();
        }
      }
    }
  };
}

fn init_ground_chunk(
  mut er_prepare_chunk: EventReader<PrepareChunkEvent>,
  mut ew_spawn_chunk: EventWriter<SpawnChunkEvent>,
  q_tilemaps: Query<&mut ChunkedTilemap>,
  tilemap_layers: ResMut<TilemapLayers>,
){
  let prepare_ground_chunk_events = er_prepare_chunk.iter().filter(|event| event.tilemap_entity == tilemap_layers.ground.expect("no ground layer"));
  for event in prepare_ground_chunk_events{
    let tilemap = q_tilemaps.get(event.tilemap_entity).expect("no tilemap");

    let tile_index =  if (event.chunk_index.x+event.chunk_index.y).abs() % 2 > 0 {
      0
    } else {
      3
    };
    let mut bundles = vec![];

    for x in 0..tilemap.chunk_size.x{
      for y in 0..tilemap.chunk_size.y{
        bundles.push(TileBundle {
          position: TilePos { x, y},
          texture: TileTexture(tile_index),
          ..Default::default()
        });
      }
    }
    ew_spawn_chunk.send(SpawnChunkEvent{
      bundles,
      tilemap_entity: event.tilemap_entity,
      chunk_index: event.chunk_index
    })
  }
}

fn init_trees_chunk(
  mut er_prepare_chunk: EventReader<PrepareChunkEvent>,
  mut ew_spawn_chunk: EventWriter<SpawnChunkEvent>,
  q_tilemaps: Query<&mut ChunkedTilemap>,
  tilemap_layers: ResMut<TilemapLayers>,
){
  let prepare_chunk_events = er_prepare_chunk.iter().filter(|event| event.tilemap_entity == tilemap_layers.trees.expect("no ground layer"));
  for event in prepare_chunk_events{
    let tilemap = q_tilemaps.get(event.tilemap_entity).expect("no tilemap");

    let tile_index =  if (event.chunk_index.x+event.chunk_index.y).abs() % 2 > 0 {
      0
    } else {
      3
    };
    let mut bundles = vec![];

    for x in 0..tilemap.chunk_size.x{
      for y in 0..tilemap.chunk_size.y{
        bundles.push(TileBundle {
          position: TilePos { x, y},
          texture: TileTexture(tile_index),
          ..Default::default()
        });
      }
    }
    ew_spawn_chunk.send(SpawnChunkEvent{
      bundles,
      tilemap_entity: event.tilemap_entity,
      chunk_index: event.chunk_index
    })
  }
//   for event in er_init_ground_chunk.iter(){
//     let (tilemap, name, children) = q_tilemaps.get(event.tilemap).unwrap();
//     if name.to_string() == "Trees layer".to_string(){
//       for &child in children.iter(){
//         if let Ok((tile_storage, tilemap_chunk)) = q_chunk.get(child){
//           if tilemap_chunk.0 == event.index{

//             for x in 0..tilemap.chunk_size.x{
//               for y in 0..tilemap.chunk_size.y{
//                 if let Some(tile) = tile_storage.get(&TilePos{
//                   x: x as u32,
//                   y: y as u32
//                 }){
//                   let (mut tile_texture, mut tile_visible) = q_tile.get_mut(tile).unwrap();
//                   if (x+y) % 2 > 0 {
//                     tile_texture.0 = 10;
//                     tile_visible.0 = true;
//                     } else {
//                       tile_visible.0 = false;
//                     };
//                 }else {
//                   info!("no tile pos");
//                 }
//               }
//             }
            

//           }
//         } else {
//           info!("no tile storage");
//         }
//       }
//     }
//   }
}