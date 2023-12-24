use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Component)]
pub struct ClientPlayer(pub u8);

#[derive(Component)]
pub struct RenderedPlayer {
    pub client_player: Entity,
}
