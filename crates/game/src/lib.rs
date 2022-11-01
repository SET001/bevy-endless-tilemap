use bevy::{prelude::{Component, Handle, HandleUntyped}, sprite::TextureAtlas};
use perlin2d::PerlinNoise2D;
pub mod states;
pub mod player;

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

pub struct AppConfig{
  pub tile_size: i32,
  pub chunk_size: i32
}