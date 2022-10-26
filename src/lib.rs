use bevy::prelude::Component;

pub mod init;
pub mod game;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]

pub enum GameStates{
  Init,
  Game,
}


#[derive(Component)]
pub struct GroundTilemap;


#[derive(Component)]
pub struct OverGroundTilemap;