//! Enemy system - AI, types, and spawning logic
use fyrox::{
    core::{
        algebra::{Vector3, Vector2},
        pool::Handle,
        reflect::prelude::*,
        visitor::prelude::*,
    },
    scene::{
        node::Node,
        transform::Transform,
        Scene,
    },
    script::{ScriptContext, ScriptTrait},
};

#[derive(Clone, Debug, Reflect, Visit)]
pub enum EnemyType {
    Chaser,
    Shooter,
    Tank,
}

impl Default for EnemyType {
    fn default() -> Self {
        EnemyType::Chaser
    }
}

#[derive(Clone, Debug, Reflect, Visit, Default)]
pub struct Enemy {
    pub enemy_type: EnemyType,
    pub health: f32,
    pub max_health: f32,
    pub speed: f32,
    pub damage: f32,
    pub attack_range: f32,
    pub last_attack_time: f32,
    pub attack_cooldown: f32,
    pub target_position: Vector3<f32>,
    pub is_alive: bool,
    
    // AI state
    pub state: EnemyState,
    pub state_timer: f32,
}

#[derive(Clone, Debug, Reflect, Visit)]
pub enum EnemyState {
    Seeking,
    Attacking,
    Retreating,
    Dead,
}

impl Default for EnemyState {
    fn default() -> Self {
        EnemyState::Seeking
    }
}

impl Enemy {
    pub fn new_chaser() -> Self {
        Self {
            enemy_type: EnemyType::Chaser,
            health: 30.0,
            max_health: 30.0,
            speed: 3.0,
            damage: 15.0,
            attack_range: 1.5,
            attack_cooldown: 1.0,
            is_alive: true,
            ..Default::default()
        }
    }
    
    pub fn new_shooter() -> Self {
        Self {
            enemy_type: EnemyType::Shooter,
            health: 20.0,
            max_health: 20.0,
            speed: 2.0,
            damage: 10.0,
            attack_range: 8.0,
            attack_cooldown: 2.0,
            is_alive: true,
            ..Default::default()
        }
    }
    
    pub fn new_tank() -> Self {
        Self {
            enemy_type: EnemyType::Tank,
            health: 80.0,
            max_health: 80.0,
            speed: 1.0,
            damage: 25.0,
            attack_range: 3.0,
            attack_cooldown: 3.0,
            is_alive: true,
            ..Default::default()
        }
    }
    
    pub fn take_damage(&mut self, damage: f32) {
        self.health -= damage;
        if self.health <= 0.0 {
            self.is_alive = false;
            self.state = EnemyState::Dead;
        }
    }
    
    pub fn can_attack(&self, current_time: f32) -> bool {
        current_time - self.last_attack_time >= self.attack_cooldown
    }
    
    pub fn get_loot_drops(&self) -> Vec<LootDrop> {
        let mut drops = vec![
            LootDrop::Experience(10.0 + self.max_health * 0.1),
        ];
        
        // Chance for health pack (20%)
        if rand::random::<f32>() < 0.2 {
            drops.push(LootDrop::Health(25.0));
        }
        
        // Chance for energy cell (10%)
        if rand::random::<f32>() < 0.1 {
            drops.push(LootDrop::Energy(30.0));
        }
        
        // Currency based on enemy type
        let currency = match self.enemy_type {
            EnemyType::Chaser => 5,
            EnemyType::Shooter => 8,
            EnemyType::Tank => 15,
        };
        drops.push(LootDrop::Currency(currency));
        
        drops
    }
}

impl ScriptTrait for Enemy {
    fn on_init(&mut self, _ctx: &mut ScriptContext) {
        // Initialize enemy
    }
    
    fn on_start(&mut self, _ctx: &mut ScriptContext) {
        // Start logic
    }
    
    fn on_update(&mut self, ctx: &mut ScriptContext) {
        if !self.is_alive {
            return;
        }
        
        self.update_ai(ctx);
        self.update_movement(ctx);
    }
}

impl Enemy {
    fn update_ai(&mut self, ctx: &mut ScriptContext) {
        // Simple AI state machine
        match self.state {
            EnemyState::Seeking => {
                // Find and move toward player
                let distance_to_target = self.distance_to_target(ctx);
                
                if distance_to_target <= self.attack_range {
                    self.state = EnemyState::Attacking;
                    self.state_timer = 0.0;
                }
            },
            EnemyState::Attacking => {
                // Attack if in range and cooldown ready
                if self.can_attack(ctx.dt) {
                    self.attack_target(ctx);
                    self.last_attack_time = ctx.dt;
                }
                
                // Check if target moved out of range
                if self.distance_to_target(ctx) > self.attack_range * 1.5 {
                    self.state = EnemyState::Seeking;
                }
            },
            EnemyState::Retreating => {
                // Move away from target
                self.state_timer += ctx.dt;
                if self.state_timer > 2.0 {
                    self.state = EnemyState::Seeking;
                }
            },
            EnemyState::Dead => {
                // Handle death
            },
        }
    }
    
    fn update_movement(&mut self, ctx: &mut ScriptContext) {
        if let Some(transform) = ctx.scene.graph[ctx.handle].cast_mut::<Transform>() {
            let current_pos = transform.position();
            let direction = (self.target_position - current_pos).normalize();
            
            let movement = match self.state {
                EnemyState::Seeking | EnemyState::Attacking => {
                    direction * self.speed * ctx.dt
                },
                EnemyState::Retreating => {
                    -direction * self.speed * ctx.dt
                },
                EnemyState::Dead => Vector3::default(),
            };
            
            transform.set_position(current_pos + movement);
        }
    }
    
    fn distance_to_target(&self, ctx: &ScriptContext) -> f32 {
        if let Some(transform) = ctx.scene.graph[ctx.handle].cast::<Transform>() {
            (self.target_position - transform.position()).magnitude()
        } else {
            f32::MAX
        }
    }
    
    fn attack_target(&mut self, _ctx: &mut ScriptContext) {
        // Implement attack logic
        // This will trigger damage to player, spawn projectiles, etc.
    }
}

#[derive(Clone, Debug)]
pub enum LootDrop {
    Experience(f32),
    Health(f32),
    Energy(f32),
    Currency(i32),
}

// TODO: Add rand crate dependency
mod rand {
    pub fn random<T: Random>() -> T {
        T::random()
    }
    
    pub trait Random {
        fn random() -> Self;
    }
    
    impl Random for f32 {
        fn random() -> Self {
            // Placeholder - replace with actual random implementation
            0.5
        }
    }
}
