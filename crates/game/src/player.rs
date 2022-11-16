use bevy::prelude::*;
use leafwing_input_manager::{Actionlike, InputManagerBundle, prelude::{InputMap, ActionState}};

use crate::{TextureAtlases, DefaultCamera};

pub enum Direction{
  Up,
  Right,
  Down,
  Left
}

#[derive(Component, Default)]
pub struct Player;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum PlayerAction {
  Fire,
  MoveLeft,
  MoveRight,
  MoveUp,
  MoveDown
}

#[derive(Component)]
pub enum PlayerRace{
  Gnome,
  Human,
}

impl Default for PlayerRace{
  fn default()->PlayerRace{
    PlayerRace::Human
  }
}

#[derive(Bundle, Default)]
pub struct PlayerBundle{
  pub player: Player,
  pub name: Name,
  pub race: PlayerRace,
  #[bundle]
  pub spatial_bundle: SpatialBundle,
}
pub fn spawn_player(
  mut commands: Commands,
  texture_handles: Res<TextureAtlases>,
){
  commands.spawn_bundle(PlayerBundle{
    name: Name::new("Player".to_string()),
    spatial_bundle: SpatialBundle{
      transform: Transform::from_translation(Vec3::new(0., 0., 50.)),
      ..Default::default()
    },
    ..Default::default()
  })
  .insert_bundle(InputManagerBundle::<PlayerAction> {
    action_state: ActionState::default(),
    input_map: InputMap::new([
      (KeyCode::Right, PlayerAction::MoveRight),
      (KeyCode::Left, PlayerAction::MoveLeft),
      (KeyCode::Up, PlayerAction::MoveUp),
      (KeyCode::Down, PlayerAction::MoveDown),
    ]),
  })
  .with_children(|parent|{
    parent.spawn_bundle(SpriteSheetBundle{
      texture_atlas: texture_handles.player.clone(),
      sprite: TextureAtlasSprite{
        index: 5,
        ..Default::default()
      },
      ..Default::default()
    });
  });
  
}

pub fn bind_camera_to_player(
  mut q_camera: Query<&mut Transform, With<DefaultCamera>>,
  q_player: Query<&Transform, (With<Player>, Without<DefaultCamera>)>
){
  let mut camera_transform = q_camera.single_mut();
  let player_transform = q_player.single();
  camera_transform.translation = player_transform.translation.clone();
}

pub fn player_controls(
  mut q_player: Query<(&ActionState<PlayerAction>, &mut Transform), With<Player>>,
  time: Res<Time>,
){
  let speed = 300.5;
  let update_step = time.delta_seconds()* speed;
  
  let (action_state, mut player_transform) = q_player.single_mut();
  if action_state.pressed(PlayerAction::MoveRight) {
    player_transform.translation.x+=update_step;
  }
  if action_state.pressed(PlayerAction::MoveLeft) {
    player_transform.translation.x-=update_step;
  }
  if action_state.pressed(PlayerAction::MoveUp) {
    player_transform.translation.y+=update_step;
  }
  if action_state.pressed(PlayerAction::MoveDown) {
    player_transform.translation.y-=update_step;
  }
}

pub fn despawn_player(){
  
}