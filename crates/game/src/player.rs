use bevy::prelude::*;
use leafwing_input_manager::{Actionlike, InputManagerBundle, prelude::{InputMap, ActionState}};

use crate::{TextureAtlases, DefaultCamera};

pub enum Direction{
  Up,
  Right,
  Down,
  Left
}

#[derive(Component)]
pub struct MoveDirection(pub Direction);

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
pub enum Gender{
  Male,
  Female
}
pub enum CentaurColor{
  Brown,
  DarkBrown,
  DarkGrey,
  LightBrown,
  LightGrey
}
#[derive(Component)]
pub enum PlayerRace{
  Centaur(Gender, CentaurColor),
  DeepDwarf,
  DeepElf,
  Demigod,
  Demonspawn,
  Draconian,
  Dwarf,
  Elf,
  Formicid,
  Gargoyle,
  Ghoul,
  Ghoul2,
  Gnome,
  Halfling,
  Human,
  Kenku,
  Kobold,
  Lorc,
  Merfolk,
  Minotaur,
  Mummy,
  Naga,
  Octopode,
  Ogre,
  Orc,
  Shadow,
  Spriggan,
  Tengu,
  Troll,
  Vampire
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
  
  // info!("race: {}", PlayerRace::Ogre as usize);
  commands.spawn_bundle(PlayerBundle{
    name: Name::new("Player".to_string()),
    spatial_bundle: SpatialBundle{
      transform: Transform::from_translation(Vec3::new(0., 0., 50.)),
      ..Default::default()
    },
    ..Default::default()
  })
  .insert_bundle(InputManagerBundle::<PlayerAction> {
    // Stores "which actions are currently pressed"
    action_state: ActionState::default(),
    // Describes how to convert from player inputs into those actions
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
  mut q_player: Query<(&ActionState<PlayerAction>, &mut Transform, Option<&MoveDirection>), With<Player>>,
  time: Res<Time>,
){
  let speed = 5.;
  let step: f32 = 1./30.;
  let delta = time.delta_seconds();
  let update_step = delta/step*speed;
  let update_step = time.delta_seconds()* 200.5;
  
  let (action_state, mut player_transform, move_direction) = q_player.single_mut();
  if move_direction.is_some(){

  } else {
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
  //player_transform.translation.x+=speed;
}

pub fn despawn_player(){
  
}