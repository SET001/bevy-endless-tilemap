use bevy::{prelude::*, DefaultPlugins, sprite::MaterialMesh2dBundle, input::mouse::MouseMotion};
use bevy_ecs_tilemap::tiles::{TileStorage, TilePos, TileTexture, TileVisible};
use bevy_editor_pls::EditorPlugin;
use chunked_tilemap::{ChunkedTilemapPlugin, bundle::{ChunkedTilemapBundle, ChunkedTilemap}, spawn::InitChunkEvent, TilemapChunk};

const CHUNK_SIZE: i32 = 5;
const TILE_SIZE: i32 = 32;

#[derive(Default)]
pub struct TilemapLayers{
  pub ground: Option<Entity>,
  pub trees: Option<Entity>
}
fn main() {
  let mut app = App::new();
  app
  .init_resource::<TilemapLayers>()
  .add_plugins(DefaultPlugins)
  .add_plugin(ChunkedTilemapPlugin)
  .add_plugin(EditorPlugin)
  .add_system(move_camera)
  .add_startup_system(startup)
  .add_system_to_stage(CoreStage::Last, init_ground_chunk)
  .add_system_to_stage(CoreStage::Last, init_trees_chunk);
    
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
    transform: Transform::default().with_scale(Vec3::splat((TILE_SIZE*CHUNK_SIZE) as f32)).with_translation(Vec3::new(0., 0., 20.)),
    material: materials.add(ColorMaterial::from(Color::rgba(0.5, 0.0, 0.5, 0.5))),
    ..default()
  }).insert(CenterMarker);


  tilemap_layers.ground = Some(commands.spawn_bundle(ChunkedTilemapBundle{
    name: Name::new("Ground layer"),
    chunked_tilemap: ChunkedTilemap{
      chunk_size: IVec2::new(CHUNK_SIZE, CHUNK_SIZE),
      tile_size: IVec2::new(TILE_SIZE, TILE_SIZE),
      range: 2,
      texture_handle: asset_server.load("../../../assets/images/grass_tiles.png"),
      ..Default::default()
    },
    ..Default::default()
  }).id());

  tilemap_layers.trees = Some(commands.spawn_bundle(ChunkedTilemapBundle{
    name: Name::new("Trees layer"),
    chunked_tilemap: ChunkedTilemap{
      chunk_size: IVec2::new(CHUNK_SIZE, CHUNK_SIZE),
      tile_size: IVec2::new(TILE_SIZE, TILE_SIZE),
      range: 2,
      texture_handle: asset_server.load("../../../assets/images/tree_tiles.png"),
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

        q_center_marker.get_single_mut().unwrap().translation = camera_transform.translation;
        for mut tilemap in q_tilemaps.iter_mut(){
          tilemap.center = camera_transform.translation.truncate();
        }
      }
    }
  };
}

fn init_ground_chunk(
  mut er_init_chunk: EventReader<InitChunkEvent>,
  q_tilemaps: Query<(&mut ChunkedTilemap, &Children)>,
  mut q_tile_texture: Query<&mut TileTexture>,
  tilemap_layers: ResMut<TilemapLayers>,
  q_chunk: Query<(&TileStorage, &TilemapChunk)>
){
  let init_ground_chunk_events = er_init_chunk.iter().filter(|event| event.tilemap == tilemap_layers.ground.unwrap());
  for event in init_ground_chunk_events{
    let (tilemap, children) = q_tilemaps.get(event.tilemap).unwrap();
    for &child in children.iter(){
      if let Ok((tile_storage, tilemap_chunk)) = q_chunk.get(child){
        if tilemap_chunk.0 == event.index{
          let tile_index =  if (event.index.x+event.index.y).abs() % 2 > 0 {
              0
            } else {
              3
            };
          for x in 0..tilemap.chunk_size.x{
            for y in 0..tilemap.chunk_size.y{
              if let Some(tile) = tile_storage.get(&TilePos{
                x: x as u32,
                y: y as u32
              }){
                let mut texture = q_tile_texture.get_mut(tile).unwrap();
                texture.0 = tile_index;
              }else {
                info!("no tile pos");
              }
            }
          }
          

        }
      } else {
        info!("no tile storage");
      }
    }
  }
}

fn init_trees_chunk(
  mut er_init_ground_chunk: EventReader<InitChunkEvent>,
  q_tilemaps: Query<(&mut ChunkedTilemap, &Name, &Children)>,
  q_chunk: Query<(&TileStorage, &TilemapChunk)>,
  mut q_tile: Query<(&mut TileTexture, &mut TileVisible)>,
){
  for event in er_init_ground_chunk.iter(){
    let (tilemap, name, children) = q_tilemaps.get(event.tilemap).unwrap();
    if name.to_string() == "Trees layer".to_string(){
      for &child in children.iter(){
        if let Ok((tile_storage, tilemap_chunk)) = q_chunk.get(child){
          if tilemap_chunk.0 == event.index{

            for x in 0..tilemap.chunk_size.x{
              for y in 0..tilemap.chunk_size.y{
                if let Some(tile) = tile_storage.get(&TilePos{
                  x: x as u32,
                  y: y as u32
                }){
                  let (mut tile_texture, mut tile_visible) = q_tile.get_mut(tile).unwrap();
                  if (x+y) % 2 > 0 {
                    tile_texture.0 = 10;
                    tile_visible.0 = true;
                    } else {
                      tile_visible.0 = false;
                    };
                }else {
                  info!("no tile pos");
                }
              }
            }
            

          }
        } else {
          info!("no tile storage");
        }
      }
    }
  }
}