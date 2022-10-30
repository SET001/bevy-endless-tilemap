use bevy::prelude::Component;
use perlin2d::PerlinNoise2D;

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

pub struct WorldNoise(pub PerlinNoise2D);