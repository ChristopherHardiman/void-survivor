//! Wave system - manages enemy spawning, difficulty scaling, and wave progression
use bevy::prelude::*;
use crate::{GameState, GameEntity};
use crate::enemy::{spawn_enemy, EnemyType};

pub struct WavePlugin;

impl Plugin for WavePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<WaveManager>()
            .add_systems(OnEnter(GameState::Playing), initialize_wave_system)
            .add_systems(Update, (
                wave_spawn_system,
                wave_progression_system,
                check_wave_completion_system,
            ).run_if(in_state(GameState::Playing)));
    }
}

#[derive(Resource)]
pub struct WaveManager {
    pub current_wave: u32,
    pub enemies_spawned: u32,
    pub enemies_to_spawn: u32,
    pub time_since_last_spawn: f32,
    pub spawn_interval: f32,
    pub wave_active: bool,
    pub break_time: f32,
    pub time_in_break: f32,
    pub difficulty_multiplier: f32,
}

impl Default for WaveManager {
    fn default() -> Self {
        Self {
            current_wave: 1,
            enemies_spawned: 0,
            enemies_to_spawn: 5, // Start with 5 enemies
            time_since_last_spawn: 0.0,
            spawn_interval: 2.0, // Spawn every 2 seconds
            wave_active: true,
            break_time: 10.0, // 10 second break between waves
            time_in_break: 0.0,
            difficulty_multiplier: 1.0,
        }
    }
}

impl WaveManager {
    pub fn start_new_wave(&mut self) {
        self.current_wave += 1;
        self.enemies_spawned = 0;
        
        // Scale difficulty
        self.difficulty_multiplier = 1.0 + (self.current_wave as f32 - 1.0) * 0.2;
        self.enemies_to_spawn = 5 + self.current_wave * 3; // More enemies each wave
        self.spawn_interval = (2.0 - (self.current_wave as f32 * 0.1)).max(0.5); // Faster spawning
        
        self.wave_active = true;
        self.time_in_break = 0.0;
        
        info!("Starting wave {} with {} enemies", self.current_wave, self.enemies_to_spawn);
    }
    
    pub fn end_wave(&mut self) {
        self.wave_active = false;
        self.time_in_break = 0.0;
        info!("Wave {} completed! Next wave in {} seconds", self.current_wave, self.break_time);
    }
    
    pub fn get_spawn_position(&self, arena_bounds: f32) -> Vec3 {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        // Spawn at random position around the arena edge for top-down view
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let distance = arena_bounds + 50.0; // Just outside the arena
        
        Vec3::new(
            angle.cos() * distance,
            0.0, // Keep Y at ground level
            angle.sin() * distance,
        )
    }
    
    pub fn get_enemy_type_for_wave(&self) -> EnemyType {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        match self.current_wave {
            1..=2 => EnemyType::Chaser,
            3..=5 => {
                if rng.gen_bool(0.7) {
                    EnemyType::Chaser
                } else {
                    EnemyType::Swarm
                }
            }
            6..=10 => {
                let roll = rng.gen_range(0.0..1.0);
                if roll < 0.4 {
                    EnemyType::Chaser
                } else if roll < 0.7 {
                    EnemyType::Swarm
                } else {
                    EnemyType::Shooter
                }
            }
            11..=15 => {
                let roll = rng.gen_range(0.0..1.0);
                if roll < 0.3 {
                    EnemyType::Chaser
                } else if roll < 0.5 {
                    EnemyType::Swarm
                } else if roll < 0.8 {
                    EnemyType::Shooter
                } else {
                    EnemyType::Tank
                }
            }
            16..=20 => {
                let roll = rng.gen_range(0.0..1.0);
                if roll < 0.2 {
                    EnemyType::Chaser
                } else if roll < 0.4 {
                    EnemyType::Swarm
                } else if roll < 0.6 {
                    EnemyType::Shooter
                } else if roll < 0.8 {
                    EnemyType::Tank
                } else {
                    EnemyType::Elite
                }
            }
            _ => {
                // Boss waves every 10 waves after wave 20
                if self.current_wave % 10 == 0 {
                    EnemyType::Boss
                } else {
                    let roll = rng.gen_range(0.0..1.0);
                    if roll < 0.15 {
                        EnemyType::Chaser
                    } else if roll < 0.3 {
                        EnemyType::Swarm
                    } else if roll < 0.5 {
                        EnemyType::Shooter
                    } else if roll < 0.7 {
                        EnemyType::Tank
                    } else {
                        EnemyType::Elite
                    }
                }
            }
        }
    }
}

fn initialize_wave_system(mut wave_manager: ResMut<WaveManager>) {
    info!("Initializing wave system - Wave 1 starting");
    wave_manager.wave_active = true;
}

fn wave_spawn_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut wave_manager: ResMut<WaveManager>,
) {
    if !wave_manager.wave_active {
        return;
    }
    
    wave_manager.time_since_last_spawn += time.delta_seconds();
    
    if wave_manager.time_since_last_spawn >= wave_manager.spawn_interval 
        && wave_manager.enemies_spawned < wave_manager.enemies_to_spawn {
        
        let spawn_position = wave_manager.get_spawn_position(400.0);
        let enemy_type = wave_manager.get_enemy_type_for_wave();
        
        spawn_enemy(&mut commands, &asset_server, enemy_type, spawn_position);
        
        wave_manager.enemies_spawned += 1;
        wave_manager.time_since_last_spawn = 0.0;
        
        info!("Spawned enemy {} of {}", wave_manager.enemies_spawned, wave_manager.enemies_to_spawn);
    }
}

fn wave_progression_system(
    time: Res<Time>,
    mut wave_manager: ResMut<WaveManager>,
) {
    if !wave_manager.wave_active {
        wave_manager.time_in_break += time.delta_seconds();
        
        if wave_manager.time_in_break >= wave_manager.break_time {
            wave_manager.start_new_wave();
        }
    }
}

fn check_wave_completion_system(
    enemy_query: Query<&crate::enemy::Enemy>,
    mut wave_manager: ResMut<WaveManager>,
) {
    if wave_manager.wave_active && wave_manager.enemies_spawned >= wave_manager.enemies_to_spawn {
        // Check if all enemies are dead
        let enemies_alive = enemy_query.iter().count();
        
        if enemies_alive == 0 {
            wave_manager.end_wave();
        }
    }
}

// Special wave events
#[derive(Event)]
pub struct WaveCompleteEvent {
    pub wave_number: u32,
    pub enemies_killed: u32,
}

#[derive(Event)]
pub struct BossWaveEvent {
    pub wave_number: u32,
}

// Systems for special events
pub fn handle_wave_complete(
    mut wave_events: EventReader<WaveCompleteEvent>,
) {
    for event in wave_events.read() {
        info!("Wave {} completed! {} enemies defeated", event.wave_number, event.enemies_killed);
        // TODO: Award bonus experience, loot, etc.
    }
}

pub fn handle_boss_wave(
    mut boss_events: EventReader<BossWaveEvent>,
) {
    for event in boss_events.read() {
        info!("Boss wave {} incoming!", event.wave_number);
        // TODO: Special effects, music, etc.
    }
}

// Wave statistics tracking
#[derive(Resource, Default)]
pub struct WaveStats {
    pub total_enemies_killed: u32,
    pub highest_wave_reached: u32,
    pub total_play_time: f32,
    pub damage_dealt: f32,
    pub damage_taken: f32,
}

impl WaveStats {
    pub fn enemy_killed(&mut self) {
        self.total_enemies_killed += 1;
    }
    
    pub fn wave_reached(&mut self, wave: u32) {
        if wave > self.highest_wave_reached {
            self.highest_wave_reached = wave;
        }
    }
    
    pub fn add_damage_dealt(&mut self, damage: f32) {
        self.damage_dealt += damage;
    }
    
    pub fn add_damage_taken(&mut self, damage: f32) {
        self.damage_taken += damage;
    }
}
