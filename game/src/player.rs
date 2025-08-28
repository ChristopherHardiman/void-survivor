//! Player system - handles movement, weapons, and stats
use fyrox::{
    core::{
        algebra::{Vector2, Vector3},
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

#[derive(Clone, Debug, Reflect, Visit, Default)]
pub struct Player {
    // Core stats
    pub health: f32,
    pub max_health: f32,
    pub shields: f32,
    pub max_shields: f32,
    pub special_energy: f32,
    pub max_special_energy: f32,
    pub experience: f32,
    pub level: u32,
    
    // Movement
    pub speed: f32,
    pub movement_input: Vector2<f32>,
    pub aim_direction: Vector3<f32>,
    
    // Weapons
    pub primary_weapon: WeaponType,
    pub fire_rate: f32,
    pub last_shot_time: f32,
    pub damage_multiplier: f32,
    
    // State
    pub is_alive: bool,
}

#[derive(Clone, Debug, Reflect, Visit)]
pub enum WeaponType {
    Blaster,
    Laser,
    Rocket,
    AoePulse,
}

impl Default for WeaponType {
    fn default() -> Self {
        WeaponType::Blaster
    }
}

impl Player {
    pub fn new() -> Self {
        Self {
            health: 100.0,
            max_health: 100.0,
            shields: 50.0,
            max_shields: 50.0,
            special_energy: 100.0,
            max_special_energy: 100.0,
            experience: 0.0,
            level: 1,
            speed: 5.0,
            movement_input: Vector2::default(),
            aim_direction: Vector3::new(0.0, 0.0, 1.0),
            primary_weapon: WeaponType::Blaster,
            fire_rate: 5.0, // shots per second
            last_shot_time: 0.0,
            damage_multiplier: 1.0,
            is_alive: true,
        }
    }
    
    pub fn take_damage(&mut self, damage: f32) {
        if self.shields > 0.0 {
            let shield_damage = damage.min(self.shields);
            self.shields -= shield_damage;
            let remaining_damage = damage - shield_damage;
            if remaining_damage > 0.0 {
                self.health -= remaining_damage;
            }
        } else {
            self.health -= damage;
        }
        
        if self.health <= 0.0 {
            self.is_alive = false;
        }
    }
    
    pub fn add_experience(&mut self, exp: f32) -> bool {
        self.experience += exp;
        let exp_needed = self.level as f32 * 100.0; // Simple leveling formula
        
        if self.experience >= exp_needed {
            self.level += 1;
            self.experience -= exp_needed;
            true // Level up occurred
        } else {
            false
        }
    }
    
    pub fn can_shoot(&self, current_time: f32) -> bool {
        current_time - self.last_shot_time >= 1.0 / self.fire_rate
    }
}

impl ScriptTrait for Player {
    fn on_init(&mut self, _ctx: &mut ScriptContext) {
        // Initialize player
    }
    
    fn on_start(&mut self, _ctx: &mut ScriptContext) {
        // Start logic
    }
    
    fn on_update(&mut self, ctx: &mut ScriptContext) {
        // Update player logic - movement, input handling, etc.
        self.update_movement(ctx);
        self.update_weapons(ctx);
    }
}

impl Player {
    fn update_movement(&mut self, ctx: &mut ScriptContext) {
        // Movement logic will be implemented here
        if let Some(transform) = ctx.scene.graph[ctx.handle].cast_mut::<Transform>() {
            let movement = Vector3::new(
                self.movement_input.x * self.speed * ctx.dt,
                0.0,
                self.movement_input.y * self.speed * ctx.dt,
            );
            transform.set_position(transform.position() + movement);
        }
    }
    
    fn update_weapons(&mut self, ctx: &mut ScriptContext) {
        // Weapon logic will be implemented here
        // Handle firing, cooldowns, etc.
    }
}
