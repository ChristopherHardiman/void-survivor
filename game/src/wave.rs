//! Wave manager - controls enemy spawning, difficulty scaling, and wave progression
use fyrox::{
    core::{
        algebra::Vector3,
        pool::Handle,
        reflect::prelude::*,
        visitor::prelude::*,
    },
    scene::Scene,
};

use crate::enemy::{Enemy, EnemyType};

#[derive(Clone, Debug, Reflect, Visit, Default)]
pub struct WaveManager {
    pub current_wave: u32,
    pub wave_state: WaveState,
    pub wave_timer: f32,
    pub wave_duration: f32,
    pub enemies_spawned: u32,
    pub enemies_to_spawn: u32,
    pub spawn_timer: f32,
    pub spawn_interval: f32,
    pub enemies_alive: u32,
    
    // Wave configuration
    pub base_enemy_count: u32,
    pub enemy_count_scaling: f32,
    pub difficulty_multiplier: f32,
    
    // Spawn configuration
    pub spawn_radius: f32,
    pub arena_center: Vector3<f32>,
    
    // Manual pause system
    pub is_paused: bool,
    pub can_pause: bool,
}

#[derive(Clone, Debug, Reflect, Visit)]
pub enum WaveState {
    Preparing,
    Active,
    Paused,      // Player manually paused for level-up
    Complete,
    ShopPhase,
}

impl Default for WaveState {
    fn default() -> Self {
        WaveState::Preparing
    }
}

impl WaveManager {
    pub fn new() -> Self {
        Self {
            current_wave: 1,
            wave_state: WaveState::Preparing,
            wave_duration: 60.0, // 60 seconds per wave base
            base_enemy_count: 10,
            enemy_count_scaling: 1.5,
            difficulty_multiplier: 1.0,
            spawn_interval: 2.0, // 2 seconds between spawns
            spawn_radius: 15.0,
            can_pause: true,
            ..Default::default()
        }
    }
    
    pub fn start_wave(&mut self) {
        self.wave_state = WaveState::Active;
        self.wave_timer = 0.0;
        self.enemies_spawned = 0;
        self.spawn_timer = 0.0;
        
        // Calculate enemies for this wave
        self.enemies_to_spawn = (self.base_enemy_count as f32 * 
            (self.enemy_count_scaling.powf(self.current_wave as f32 - 1.0))) as u32;
        
        // Update difficulty
        self.difficulty_multiplier = 1.0 + (self.current_wave - 1) as f32 * 0.2;
        
        println!("Wave {} started! Enemies to spawn: {}", self.current_wave, self.enemies_to_spawn);
    }
    
    pub fn update(&mut self, dt: f32) -> Vec<WaveEvent> {
        let mut events = Vec::new();
        
        match self.wave_state {
            WaveState::Preparing => {
                // Brief pause before wave starts
                self.wave_timer += dt;
                if self.wave_timer >= 3.0 {
                    self.start_wave();
                    events.push(WaveEvent::WaveStarted(self.current_wave));
                }
            },
            
            WaveState::Active => {
                self.wave_timer += dt;
                self.spawn_timer += dt;
                
                // Spawn enemies
                if self.enemies_spawned < self.enemies_to_spawn && self.spawn_timer >= self.spawn_interval {
                    if let Some(enemy_data) = self.get_next_enemy_spawn() {
                        events.push(WaveEvent::SpawnEnemy(enemy_data));
                        self.enemies_spawned += 1;
                        self.enemies_alive += 1;
                        self.spawn_timer = 0.0;
                    }
                }
                
                // Check if wave is complete
                if self.enemies_spawned >= self.enemies_to_spawn && self.enemies_alive == 0 {
                    self.complete_wave();
                    events.push(WaveEvent::WaveComplete(self.current_wave));
                }
            },
            
            WaveState::Paused => {
                // Player has paused for level-up
                // Wait for player to resume
            },
            
            WaveState::Complete => {
                // Transition to shop phase
                self.wave_state = WaveState::ShopPhase;
                events.push(WaveEvent::ShopPhaseStarted);
            },
            
            WaveState::ShopPhase => {
                // Wait for player to finish shopping
            },
        }
        
        events
    }
    
    pub fn pause_wave(&mut self) -> bool {
        if self.can_pause && matches!(self.wave_state, WaveState::Active) {
            self.wave_state = WaveState::Paused;
            self.can_pause = false; // Can only pause once per wave
            true
        } else {
            false
        }
    }
    
    pub fn resume_wave(&mut self) {
        if matches!(self.wave_state, WaveState::Paused) {
            self.wave_state = WaveState::Active;
        }
    }
    
    pub fn complete_shop_phase(&mut self) {
        if matches!(self.wave_state, WaveState::ShopPhase) {
            self.next_wave();
        }
    }
    
    pub fn enemy_killed(&mut self) {
        if self.enemies_alive > 0 {
            self.enemies_alive -= 1;
        }
    }
    
    fn complete_wave(&mut self) {
        self.wave_state = WaveState::Complete;
        println!("Wave {} complete!", self.current_wave);
    }
    
    fn next_wave(&mut self) {
        self.current_wave += 1;
        self.wave_state = WaveState::Preparing;
        self.wave_timer = 0.0;
        self.can_pause = true;
        
        // Adjust spawn rate for higher waves
        self.spawn_interval = (2.0 - (self.current_wave as f32 * 0.1)).max(0.5);
    }
    
    fn get_next_enemy_spawn(&self) -> Option<EnemySpawnData> {
        let enemy_type = self.choose_enemy_type();
        let spawn_position = self.get_spawn_position();
        
        Some(EnemySpawnData {
            enemy_type,
            position: spawn_position,
            difficulty_multiplier: self.difficulty_multiplier,
        })
    }
    
    fn choose_enemy_type(&self) -> EnemyType {
        // Simple enemy distribution based on wave
        let rand_val = self.pseudo_random();
        
        match self.current_wave {
            1..=2 => {
                // Early waves: mostly chasers
                if rand_val < 0.8 { EnemyType::Chaser } else { EnemyType::Shooter }
            },
            3..=5 => {
                // Mid waves: mixed
                if rand_val < 0.5 { EnemyType::Chaser } 
                else if rand_val < 0.8 { EnemyType::Shooter } 
                else { EnemyType::Tank }
            },
            _ => {
                // Late waves: more variety and tanks
                if rand_val < 0.4 { EnemyType::Chaser } 
                else if rand_val < 0.7 { EnemyType::Shooter } 
                else { EnemyType::Tank }
            }
        }
    }
    
    fn get_spawn_position(&self) -> Vector3<f32> {
        // Spawn enemies around the arena perimeter
        let angle = self.pseudo_random() * 2.0 * std::f32::consts::PI;
        let x = self.arena_center.x + self.spawn_radius * angle.cos();
        let z = self.arena_center.z + self.spawn_radius * angle.sin();
        
        Vector3::new(x, self.arena_center.y, z)
    }
    
    fn pseudo_random(&self) -> f32 {
        // Simple pseudo-random based on wave timer and spawn count
        ((self.wave_timer * 1000.0) as u32 + self.enemies_spawned * 17) as f32 / 1000.0 % 1.0
    }
}

#[derive(Clone, Debug)]
pub enum WaveEvent {
    WaveStarted(u32),
    WaveComplete(u32),
    SpawnEnemy(EnemySpawnData),
    ShopPhaseStarted,
}

#[derive(Clone, Debug)]
pub struct EnemySpawnData {
    pub enemy_type: EnemyType,
    pub position: Vector3<f32>,
    pub difficulty_multiplier: f32,
}
