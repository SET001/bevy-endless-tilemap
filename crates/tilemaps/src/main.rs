use bevy::{prelude::*, DefaultPlugins, sprite::MaterialMesh2dBundle, input::mouse::MouseMotion};
use bevy_editor_pls::EditorPlugin;
use tilemaps::{ChunkedTilemapPlugin, bundle::{ChunkedTilemapBundle, ChunkedTilemap}};

const CHUNK_SIZE: i32 = 5;
const TILE_SIZE: i32 = 32;
fn main() {
  let mut app = App::new();
  app
    .add_startup_system(startup)
    .add_system(move_camera)
    .add_plugins(DefaultPlugins)
    .add_plugin(ChunkedTilemapPlugin)
    .add_plugin(EditorPlugin);
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
){
  commands.spawn_bundle(Camera2dBundle::default()).insert(DefaultCamera);
  commands.spawn_bundle(MaterialMesh2dBundle {
    mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
    transform: Transform::default().with_scale(Vec3::splat((TILE_SIZE*CHUNK_SIZE) as f32)).with_translation(Vec3::new(0., 0., 20.)),
    material: materials.add(ColorMaterial::from(Color::rgba(0.5, 0.0, 0.5, 0.5))),
    ..default()
  }).insert(CenterMarker);


  commands.spawn_bundle(ChunkedTilemapBundle{
    name: Name::new("Ground layer"),
    chunked_tilemap: ChunkedTilemap{
      chunk_size: IVec2::new(CHUNK_SIZE, CHUNK_SIZE),
      tile_size: IVec2::new(TILE_SIZE, TILE_SIZE),
      range: 2,
      ..Default::default()
    },
    ..Default::default()
  });

  commands.spawn_bundle(ChunkedTilemapBundle{
    name: Name::new("Trees layer"),
    chunked_tilemap: ChunkedTilemap{
      chunk_size: IVec2::new(CHUNK_SIZE, CHUNK_SIZE),
      tile_size: IVec2::new(TILE_SIZE, TILE_SIZE),
      range: 2,
      ..Default::default()
    },
    ..Default::default()
  });
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