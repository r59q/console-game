use bevy_ecs::entity::Entity;
use bevy_ecs::system::EntityCommands;
use bevy_ecs::world::{EntityMut};

pub mod player_character_prefab;
mod cell_prefab;

pub struct Prefabs {}

pub type Prefab = fn(EntityMut) -> Entity;
pub type PrefabCommands = fn(EntityCommands) -> Entity;
