use crate::misc::HealthComponent;
use bevy::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use thetawave_interface::spawnable::ProjectileType;

use crate::player::components::AbilityType;

/// Contains data necessary to create a player entity.
/// A character is chosen at the beginning of the game.
/// The base stats of the player are provided from the character.
/// Other data such as sprite sheets are also included with the character.
#[derive(Deserialize, Clone)]
pub struct Character {
    /// Base acceleration
    pub acceleration: Vec2,
    /// Base deceleration
    pub deceleration: Vec2,
    /// Base speed
    pub speed: Vec2,
    /// Collider size (relative to the sprite size)
    pub collider_dimensions: Vec2,
    /// Density of the collider (mass of collider is proportional to its size)
    pub collider_density: f32,
    /// Character type
    pub character_type: CharacterType,
    /// Projectile type
    pub projectile_type: ProjectileType,
    /// Time until fired projectile despawns
    pub projectile_despawn_time: f32,
    /// Velocity of fired projectile
    pub projectile_velocity: Vec2,
    /// Position of projectile spawn relative to player
    pub projectile_offset_position: Vec2,
    /// Period of time between firing blasts
    pub fire_period: f32,
    /// Health of the player
    pub health: usize,
    /// Shields of the player
    pub shields: usize,
    /// Shields recharging rate
    pub shields_recharge_rate: f32,
    /// Amount of damage dealt per attack
    pub attack_damage: usize,
    /// Amount of damage dealt on contact
    pub collision_damage: usize,
    /// Distance to attract items and consumables
    pub attraction_distance: f32,
    /// Acceleration applied to items and conumables in attraction distance
    pub attraction_acceleration: f32,
    /// Amount of money character has collected
    pub money: usize,
    /// Ability cooldown time
    pub ability_period: f32,
    /// Type of ability
    pub ability_type: AbilityType,
}

impl From<&Character> for HealthComponent {
    fn from(character: &Character) -> Self {
        HealthComponent::new(
            character.health,
            character.shields,
            character.shields_recharge_rate,
        )
    }
}

#[derive(Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
pub enum CharacterType {
    Captain,
    Juggernaut,
}

/// Manages all characters
#[derive(Resource, Deserialize)]
pub struct CharactersResource {
    /// Names mapped to characters for all characters
    pub characters: HashMap<CharacterType, Character>,
}
