use bevy::{prelude::{Component, Handle, HandleUntyped}, sprite::TextureAtlas};
use perlin2d::PerlinNoise2D;
pub mod init;
pub mod game;
pub mod player;
pub mod load;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]

pub enum GameStates{
  Load,
  Init,
  Game,
}


#[derive(Component)]
pub struct GroundTilemap;


#[derive(Component)]
pub struct OverGroundTilemap;

#[derive(Default)]
pub struct TextureAtlases{
  pub player: Handle<TextureAtlas>,
}

#[derive(Default)]
pub struct AssetsLoading(pub Vec<HandleUntyped>);


#[derive(Component)]
pub struct DefaultCamera;

pub struct WorldNoise(pub PerlinNoise2D);
