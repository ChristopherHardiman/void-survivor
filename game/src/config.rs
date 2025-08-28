//! Configuration system - handles game balance, settings, and data loading
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GameConfig>()
            .init_resource::<PlayerConfig>()
            .init_resource::<EnemyConfig>()
            .init_resource::<WaveConfig>();
    }
}

#[derive(Resource, Serialize, Deserialize, Clone, Debug)]
pub struct GameConfig {
    pub arena_bounds: f32,
    pub difficulty_scaling: f32,
    pub loot_chance_multiplier: f32,
    pub experience_multiplier: f32,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            arena_bounds: 400.0,
            difficulty_scaling: 1.2,
            loot_chance_multiplier: 1.0,
            experience_multiplier: 1.0,
        }
    }
}

#[derive(Resource, Serialize, Deserialize, Clone, Debug)]
pub struct PlayerConfig {
    pub base_health: f32,
    pub base_shields: f32,
    pub base_speed: f32,
    pub base_damage: f32,
    pub base_fire_rate: f32,
    pub shield_regen_rate: f32,
    pub shield_regen_delay: f32,
}

impl Default for PlayerConfig {
    fn default() -> Self {
        Self {
            base_health: 100.0,
            base_shields: 50.0,
            base_speed: 300.0,
            base_damage: 20.0,
            base_fire_rate: 5.0,
            shield_regen_rate: 10.0,
            shield_regen_delay: 3.0,
        }
    }
}

#[derive(Resource, Serialize, Deserialize, Clone, Debug)]
pub struct EnemyConfig {
    pub chaser: EnemyStats,
    pub shooter: EnemyStats,
    pub tank: EnemyStats,
    pub swarm: EnemyStats,
    pub elite: EnemyStats,
    pub boss: EnemyStats,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EnemyStats {
    pub health: f32,
    pub damage: f32,
    pub speed: f32,
    pub experience_value: f32,
    pub loot_chance: f32,
}

impl Default for EnemyConfig {
    fn default() -> Self {
        Self {
            chaser: EnemyStats {
                health: 50.0,
                damage: 20.0,
                speed: 100.0,
                experience_value: 10.0,
                loot_chance: 0.1,
            },
            shooter: EnemyStats {
                health: 30.0,
                damage: 15.0,
                speed: 60.0,
                experience_value: 15.0,
                loot_chance: 0.15,
            },
            tank: EnemyStats {
                health: 150.0,
                damage: 35.0,
                speed: 40.0,
                experience_value: 25.0,
                loot_chance: 0.3,
            },
            swarm: EnemyStats {
                health: 20.0,
                damage: 10.0,
                speed: 150.0,
                experience_value: 5.0,
                loot_chance: 0.05,
            },
            elite: EnemyStats {
                health: 100.0,
                damage: 40.0,
                speed: 80.0,
                experience_value: 50.0,
                loot_chance: 0.5,
            },
            boss: EnemyStats {
                health: 500.0,
                damage: 60.0,
                speed: 50.0,
                experience_value: 200.0,
                loot_chance: 0.9,
            },
        }
    }
}

#[derive(Resource, Serialize, Deserialize, Clone, Debug)]
pub struct WaveConfig {
    pub base_enemies_per_wave: u32,
    pub enemies_scaling_per_wave: u32,
    pub spawn_interval_base: f32,
    pub spawn_interval_reduction: f32,
    pub break_duration: f32,
    pub difficulty_multiplier_per_wave: f32,
}

impl Default for WaveConfig {
    fn default() -> Self {
        Self {
            base_enemies_per_wave: 5,
            enemies_scaling_per_wave: 3,
            spawn_interval_base: 2.0,
            spawn_interval_reduction: 0.1,
            break_duration: 10.0,
            difficulty_multiplier_per_wave: 0.2,
        }
    }
}

// Configuration loading and saving functions
impl GameConfig {
    pub fn load_from_file() -> Result<Self, Box<dyn std::error::Error>> {
        let config_str = std::fs::read_to_string("config/game_config.ron")?;
        let config: GameConfig = ron::de::from_str(&config_str)?;
        Ok(config)
    }
    
    pub fn save_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        std::fs::create_dir_all("config")?;
        let config_str = ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default())?;
        std::fs::write("config/game_config.ron", config_str)?;
        Ok(())
    }
}

impl PlayerConfig {
    pub fn load_from_file() -> Result<Self, Box<dyn std::error::Error>> {
        let config_str = std::fs::read_to_string("config/player_config.ron")?;
        let config: PlayerConfig = ron::de::from_str(&config_str)?;
        Ok(config)
    }
    
    pub fn save_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        std::fs::create_dir_all("config")?;
        let config_str = ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default())?;
        std::fs::write("config/player_config.ron", config_str)?;
        Ok(())
    }
}

impl EnemyConfig {
    pub fn load_from_file() -> Result<Self, Box<dyn std::error::Error>> {
        let config_str = std::fs::read_to_string("config/enemy_config.ron")?;
        let config: EnemyConfig = ron::de::from_str(&config_str)?;
        Ok(config)
    }
    
    pub fn save_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        std::fs::create_dir_all("config")?;
        let config_str = ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default())?;
        std::fs::write("config/enemy_config.ron", config_str)?;
        Ok(())
    }
}

impl WaveConfig {
    pub fn load_from_file() -> Result<Self, Box<dyn std::error::Error>> {
        let config_str = std::fs::read_to_string("config/wave_config.ron")?;
        let config: WaveConfig = ron::de::from_str(&config_str)?;
        Ok(config)
    }
    
    pub fn save_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        std::fs::create_dir_all("config")?;
        let config_str = ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default())?;
        std::fs::write("config/wave_config.ron", config_str)?;
        Ok(())
    }
}
