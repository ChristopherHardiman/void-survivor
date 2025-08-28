//! Bevy Arena Survivor - A top-down arena survival shooter
//! 
//! This game features:
//! - Player movement and shooting
//! - Multiple enemy types with different behaviors
//! - Wave-based gameplay with increasing difficulty
//! - Loot collection and player upgrades
//! - Particle effects and visual feedback

use bevy::prelude::*;

// Re-export Bevy for convenience
pub use bevy;

// Game modules
pub mod player;
pub mod enemy;
pub mod wave;
pub mod loot;
pub mod upgrade;
pub mod ui;
pub mod fx;
pub mod audio;
pub mod config;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    MainMenu,
    Playing,
    Paused,
    GameOver,
}

#[derive(Component)]
pub struct GameEntity;

#[derive(Component)]
pub struct MainCamera;
