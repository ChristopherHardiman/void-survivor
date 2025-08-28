//! Configuration and constants - game balance, settings, and tuning parameters
use fyrox::{
    core::{
        reflect::prelude::*,
        visitor::prelude::*,
    },
};

use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Reflect, Visit, Serialize, Deserialize)]
pub struct GameConfig {
    pub player: PlayerConfig,
    pub enemies: EnemyConfig,
    pub waves: WaveConfig,
    pub weapons: WeaponConfig,
    pub loot: LootConfig,
    pub upgrades: UpgradeConfig,
    pub arena: ArenaConfig,
    pub audio: AudioConfig,
    pub ui: UIConfig,
}

#[derive(Clone, Debug, Reflect, Visit, Serialize, Deserialize)]
pub struct PlayerConfig {
    pub base_health: f32,
    pub base_shields: f32,
    pub base_special_energy: f32,
    pub base_speed: f32,
    pub shield_regen_rate: f32,
    pub shield_regen_delay: f32,
    pub special_energy_regen_rate: f32,
    pub invulnerability_time: f32, // After taking damage
}

#[derive(Clone, Debug, Reflect, Visit, Serialize, Deserialize)]
pub struct EnemyConfig {
    pub chaser: ChaserConfig,
    pub shooter: ShooterConfig,
    pub tank: TankConfig,
    pub spawn_distance_min: f32,
    pub spawn_distance_max: f32,
    pub max_enemies_on_screen: u32,
}

#[derive(Clone, Debug, Reflect, Visit, Serialize, Deserialize)]
pub struct ChaserConfig {
    pub health: f32,
    pub speed: f32,
    pub damage: f32,
    pub attack_range: f32,
    pub attack_cooldown: f32,
}

#[derive(Clone, Debug, Reflect, Visit, Serialize, Deserialize)]
pub struct ShooterConfig {
    pub health: f32,
    pub speed: f32,
    pub damage: f32,
    pub attack_range: f32,
    pub attack_cooldown: f32,
    pub projectile_speed: f32,
}

#[derive(Clone, Debug, Reflect, Visit, Serialize, Deserialize)]
pub struct TankConfig {
    pub health: f32,
    pub speed: f32,
    pub damage: f32,
    pub attack_range: f32,
    pub attack_cooldown: f32,
    pub armor: f32, // Damage reduction
}

#[derive(Clone, Debug, Reflect, Visit, Serialize, Deserialize)]
pub struct WaveConfig {
    pub base_enemy_count: u32,
    pub enemy_scaling_per_wave: f32,
    pub difficulty_scaling_per_wave: f32,
    pub base_spawn_interval: f32,
    pub min_spawn_interval: f32,
    pub spawn_interval_reduction: f32,
    pub prep_time: f32, // Time between waves
}

#[derive(Clone, Debug, Reflect, Visit, Serialize, Deserialize)]
pub struct WeaponConfig {
    pub blaster: BlasterConfig,
    pub laser: LaserConfig,
    pub rocket: RocketConfig,
    pub aoe_pulse: AoePulseConfig,
}

#[derive(Clone, Debug, Reflect, Visit, Serialize, Deserialize)]
pub struct BlasterConfig {
    pub damage: f32,
    pub fire_rate: f32,
    pub projectile_speed: f32,
    pub range: f32,
    pub energy_cost: f32,
}

#[derive(Clone, Debug, Reflect, Visit, Serialize, Deserialize)]
pub struct LaserConfig {
    pub damage_per_second: f32,
    pub max_range: f32,
    pub energy_cost_per_second: f32,
    pub charge_time: f32,
    pub pierce_count: u32,
}

#[derive(Clone, Debug, Reflect, Visit, Serialize, Deserialize)]
pub struct RocketConfig {
    pub damage: f32,
    pub explosion_radius: f32,
    pub fire_rate: f32,
    pub projectile_speed: f32,
    pub energy_cost: f32,
}

#[derive(Clone, Debug, Reflect, Visit, Serialize, Deserialize)]
pub struct AoePulseConfig {
    pub damage: f32,
    pub radius: f32,
    pub energy_cost: f32,
    pub cooldown: f32,
}

#[derive(Clone, Debug, Reflect, Visit, Serialize, Deserialize)]
pub struct LootConfig {
    pub exp_drop_base: f32,
    pub exp_drop_scaling: f32,
    pub health_pack_heal: f32,
    pub energy_cell_restore: f32,
    pub currency_base: u32,
    pub drop_chances: DropChances,
    pub attraction_range: f32,
    pub attraction_speed: f32,
    pub lifetime: f32,
}

#[derive(Clone, Debug, Reflect, Visit, Serialize, Deserialize)]
pub struct DropChances {
    pub health_pack: f32,
    pub energy_cell: f32,
    pub bonus_currency: f32,
}

#[derive(Clone, Debug, Reflect, Visit, Serialize, Deserialize)]
pub struct UpgradeConfig {
    pub level_exp_base: f32,
    pub level_exp_scaling: f32,
    pub max_upgrade_choices: usize,
    pub shop_item_scaling: f32,
}

#[derive(Clone, Debug, Reflect, Visit, Serialize, Deserialize)]
pub struct ArenaConfig {
    pub radius: f32,
    pub boundary_damage: f32,
    pub boundary_knockback: f32,
    pub spawn_radius: f32,
}

#[derive(Clone, Debug, Reflect, Visit, Serialize, Deserialize)]
pub struct AudioConfig {
    pub master_volume: f32,
    pub music_volume: f32,
    pub sfx_volume: f32,
    pub enable_music: bool,
    pub enable_sfx: bool,
}

#[derive(Clone, Debug, Reflect, Visit, Serialize, Deserialize)]
pub struct UIConfig {
    pub hud_opacity: f32,
    pub show_damage_numbers: bool,
    pub show_enemy_health: bool,
    pub auto_pause_on_levelup: bool,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            player: PlayerConfig::default(),
            enemies: EnemyConfig::default(),
            waves: WaveConfig::default(),
            weapons: WeaponConfig::default(),
            loot: LootConfig::default(),
            upgrades: UpgradeConfig::default(),
            arena: ArenaConfig::default(),
            audio: AudioConfig::default(),
            ui: UIConfig::default(),
        }
    }
}

impl Default for PlayerConfig {
    fn default() -> Self {
        Self {
            base_health: 100.0,
            base_shields: 50.0,
            base_special_energy: 100.0,
            base_speed: 5.0,
            shield_regen_rate: 10.0, // per second
            shield_regen_delay: 3.0, // seconds after taking damage
            special_energy_regen_rate: 20.0, // per second
            invulnerability_time: 0.5,
        }
    }
}

impl Default for EnemyConfig {
    fn default() -> Self {
        Self {
            chaser: ChaserConfig::default(),
            shooter: ShooterConfig::default(),
            tank: TankConfig::default(),
            spawn_distance_min: 12.0,
            spawn_distance_max: 20.0,
            max_enemies_on_screen: 50,
        }
    }
}

impl Default for ChaserConfig {
    fn default() -> Self {
        Self {
            health: 30.0,
            speed: 3.0,
            damage: 15.0,
            attack_range: 1.5,
            attack_cooldown: 1.0,
        }
    }
}

impl Default for ShooterConfig {
    fn default() -> Self {
        Self {
            health: 20.0,
            speed: 2.0,
            damage: 10.0,
            attack_range: 8.0,
            attack_cooldown: 2.0,
            projectile_speed: 6.0,
        }
    }
}

impl Default for TankConfig {
    fn default() -> Self {
        Self {
            health: 80.0,
            speed: 1.0,
            damage: 25.0,
            attack_range: 3.0,
            attack_cooldown: 3.0,
            armor: 0.2, // 20% damage reduction
        }
    }
}

impl Default for WaveConfig {
    fn default() -> Self {
        Self {
            base_enemy_count: 10,
            enemy_scaling_per_wave: 1.5,
            difficulty_scaling_per_wave: 0.2,
            base_spawn_interval: 2.0,
            min_spawn_interval: 0.5,
            spawn_interval_reduction: 0.1,
            prep_time: 3.0,
        }
    }
}

impl Default for WeaponConfig {
    fn default() -> Self {
        Self {
            blaster: BlasterConfig::default(),
            laser: LaserConfig::default(),
            rocket: RocketConfig::default(),
            aoe_pulse: AoePulseConfig::default(),
        }
    }
}

impl Default for BlasterConfig {
    fn default() -> Self {
        Self {
            damage: 20.0,
            fire_rate: 5.0, // shots per second
            projectile_speed: 15.0,
            range: 20.0,
            energy_cost: 0.0, // Free to use
        }
    }
}

impl Default for LaserConfig {
    fn default() -> Self {
        Self {
            damage_per_second: 50.0,
            max_range: 25.0,
            energy_cost_per_second: 30.0,
            charge_time: 0.5,
            pierce_count: 3,
        }
    }
}

impl Default for RocketConfig {
    fn default() -> Self {
        Self {
            damage: 60.0,
            explosion_radius: 3.0,
            fire_rate: 1.5,
            projectile_speed: 8.0,
            energy_cost: 25.0,
        }
    }
}

impl Default for AoePulseConfig {
    fn default() -> Self {
        Self {
            damage: 80.0,
            radius: 8.0,
            energy_cost: 50.0,
            cooldown: 5.0,
        }
    }
}

impl Default for LootConfig {
    fn default() -> Self {
        Self {
            exp_drop_base: 10.0,
            exp_drop_scaling: 0.1, // 10% more per enemy level
            health_pack_heal: 25.0,
            energy_cell_restore: 30.0,
            currency_base: 5,
            drop_chances: DropChances::default(),
            attraction_range: 3.0,
            attraction_speed: 8.0,
            lifetime: 30.0,
        }
    }
}

impl Default for DropChances {
    fn default() -> Self {
        Self {
            health_pack: 0.2, // 20% chance
            energy_cell: 0.1, // 10% chance
            bonus_currency: 0.15, // 15% chance for extra currency
        }
    }
}

impl Default for UpgradeConfig {
    fn default() -> Self {
        Self {
            level_exp_base: 100.0,
            level_exp_scaling: 1.2, // 20% more exp needed per level
            max_upgrade_choices: 3,
            shop_item_scaling: 1.5, // Cost increases by 50% per level
        }
    }
}

impl Default for ArenaConfig {
    fn default() -> Self {
        Self {
            radius: 25.0,
            boundary_damage: 20.0, // Damage per second outside arena
            boundary_knockback: 10.0,
            spawn_radius: 30.0, // Enemies spawn outside arena
        }
    }
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            master_volume: 1.0,
            music_volume: 0.7,
            sfx_volume: 0.8,
            enable_music: true,
            enable_sfx: true,
        }
    }
}

impl Default for UIConfig {
    fn default() -> Self {
        Self {
            hud_opacity: 0.9,
            show_damage_numbers: true,
            show_enemy_health: true,
            auto_pause_on_levelup: true,
        }
    }
}

// Constants for easy access
pub mod constants {
    pub const GAME_NAME: &str = "Fyrox Arena Survivor";
    pub const GAME_VERSION: &str = "0.1.0";
    
    // Physics
    pub const GRAVITY: f32 = 0.0; // Top-down game, no gravity
    pub const COLLISION_LAYERS: u32 = 8;
    
    // Performance
    pub const MAX_PARTICLES: usize = 1000;
    pub const MAX_PROJECTILES: usize = 100;
    pub const MAX_LOOT_ITEMS: usize = 50;
    
    // File paths
    pub const CONFIG_FILE: &str = "game_config.ron";
    pub const SAVE_FILE: &str = "save_data.ron";
    pub const SETTINGS_FILE: &str = "settings.ron";
    
    // Scene paths
    pub const MAIN_SCENE: &str = "data/scenes/main_arena.rgs";
    pub const MENU_SCENE: &str = "data/scenes/main_menu.rgs";
}

impl GameConfig {
    pub fn load_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config: GameConfig = ron::from_str(&content)?;
        Ok(config)
    }
    
    pub fn save_to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let content = ron::to_string_pretty(self, ron::ser::PrettyConfig::default())?;
        std::fs::write(path, content)?;
        Ok(())
    }
    
    pub fn load_or_default() -> Self {
        Self::load_from_file(constants::CONFIG_FILE)
            .unwrap_or_else(|_| {
                println!("Could not load config file, using defaults");
                Self::default()
            })
    }
}

// Balance validation
impl GameConfig {
    pub fn validate(&self) -> Vec<String> {
        let mut warnings = Vec::new();
        
        // Validate player config
        if self.player.base_health <= 0.0 {
            warnings.push("Player base health must be positive".to_string());
        }
        
        if self.player.base_speed <= 0.0 {
            warnings.push("Player base speed must be positive".to_string());
        }
        
        // Validate enemy configs
        if self.enemies.chaser.health <= 0.0 {
            warnings.push("Chaser health must be positive".to_string());
        }
        
        // Validate weapon configs
        if self.weapons.blaster.damage <= 0.0 {
            warnings.push("Blaster damage must be positive".to_string());
        }
        
        // Validate arena
        if self.arena.radius <= 0.0 {
            warnings.push("Arena radius must be positive".to_string());
        }
        
        if self.arena.spawn_radius <= self.arena.radius {
            warnings.push("Spawn radius should be larger than arena radius".to_string());
        }
        
        warnings
    }
}
