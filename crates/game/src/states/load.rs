use bevy::{prelude::*, app::AppExit, asset::LoadState};

use crate::{GameStates, TextureAtlases, AssetsLoading};

pub struct LoadStatePlugin;

impl Plugin for LoadStatePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system_set(SystemSet::on_enter(GameStates::Load).with_system(load))
      .add_system_set(SystemSet::on_update(GameStates::Load).with_system(update));
  }
}

fn load(
  mut loading: ResMut<AssetsLoading>,
  asset_server: Res<AssetServer>,
){
  loading.0.append(&mut asset_server.load_folder("images").unwrap().clone());

}

fn update(
  mut app_state: ResMut<State<GameStates>>,
  mut e_exit: EventWriter<AppExit>,
  mut textures: ResMut<Assets<TextureAtlas>>,
  mut texture_handles: ResMut<TextureAtlases>,
  asset_server: Res<AssetServer>,
  loading: Res<AssetsLoading>,
){
  match asset_server.get_group_load_state(loading.0.iter().map(|h| h.id)){
    LoadState::Failed => {
      error!("assets loading failure");
      e_exit.send(AppExit)
    }
    LoadState::Loaded => {
      info!("all {:?} assets loaded", loading.0.len());
      texture_handles.player = textures.add(TextureAtlas::from_grid(
        asset_server.get_handle("images/players.png"),
        Vec2::new(32.,32.),
        11, 
        11
      ));
      info!("texture_handles.player: {:?}", texture_handles.player);
      app_state.set(GameStates::Game).unwrap();
    }
    _ => {}
  }
}