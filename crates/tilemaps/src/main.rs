use bevy::{prelude::*, DefaultPlugins, sprite::MaterialMesh2dBundle, input::mouse::MouseMotion};
use bevy_editor_pls::EditorPlugin;
use tilemaps::{chunks::{CurrentChunk, ChunkedTilemapConfig}, ChunkedTilemapPlugin};

fn main() {
  let mut app = App::new();
  app
    .add_startup_system(startup)
    .add_system(move_camera)
    .insert_resource(ChunkedTilemapConfig{
      chunk_size: IVec2::new(5, 5),
      tile_size: IVec2::new(32, 32),
      ..Default::default()
    })
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
  mut current_chunk: ResMut<CurrentChunk>,
  chunked_config: Res<ChunkedTilemapConfig>
){
  current_chunk.0.x = 0;
  current_chunk.0.y = 0;

  commands.spawn_bundle(Camera2dBundle::default()).insert(DefaultCamera);
  commands.spawn_bundle(MaterialMesh2dBundle {
    mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
    transform: Transform::default().with_scale(Vec3::splat((chunked_config.chunk_size.x) as f32)).with_translation(Vec3::new(0., 0., 20.)),
    material: materials.add(ColorMaterial::from(Color::rgba(0.5, 0.0, 0.5, 0.5))),
    ..default()
  }).insert(CenterMarker);

}

fn move_camera(
  mut q_camera: Query<&mut Transform, With<DefaultCamera>>,
  mut q_center_marker: Query<&mut Transform, (With<CenterMarker>, Without<DefaultCamera>)>,
  mut motion_evr: EventReader<MouseMotion>,
  buttons: Res<Input<MouseButton>>,
  mut chunked_config: ResMut<ChunkedTilemapConfig>
){
  if let Ok(mut camera_transform) = q_camera.get_single_mut(){
    if buttons.pressed(MouseButton::Left){
      for event in motion_evr.iter(){
        camera_transform.translation.x -= event.delta.x;
        camera_transform.translation.y += event.delta.y;
        q_center_marker.get_single_mut().unwrap().translation = camera_transform.translation;
        chunked_config.center = camera_transform.translation.truncate();
      }
    }
  };
}