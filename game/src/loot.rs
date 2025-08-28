//! Loot system - drops, pickups, and collectibles
use fyrox::{
    core::{
        algebra::Vector3,
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
pub enum LootType {
    Experience,
    Health,
    Energy,
    Currency,
}

impl Default for LootType {
    fn default() -> Self {
        LootType::Experience
    }
}

#[derive(Clone, Debug, Reflect, Visit, Default)]
pub struct LootItem {
    pub loot_type: LootType,
    pub value: f32,
    pub pickup_range: f32,
    pub attraction_range: f32,
    pub attraction_speed: f32,
    pub lifetime: f32,
    pub age: f32,
    pub is_collected: bool,
    pub is_attracted: bool,
    
    // Visual effects
    pub bob_speed: f32,
    pub bob_height: f32,
    pub rotation_speed: f32,
    pub initial_position: Vector3<f32>,
}

impl LootItem {
    pub fn new_experience(value: f32, position: Vector3<f32>) -> Self {
        Self {
            loot_type: LootType::Experience,
            value,
            pickup_range: 1.0,
            attraction_range: 3.0,
            attraction_speed: 8.0,
            lifetime: 30.0, // 30 seconds before despawn
            bob_speed: 2.0,
            bob_height: 0.3,
            rotation_speed: 90.0, // degrees per second
            initial_position: position,
            ..Default::default()
        }
    }
    
    pub fn new_health(value: f32, position: Vector3<f32>) -> Self {
        Self {
            loot_type: LootType::Health,
            value,
            pickup_range: 1.2,
            attraction_range: 2.5,
            attraction_speed: 6.0,
            lifetime: 25.0,
            bob_speed: 1.5,
            bob_height: 0.2,
            rotation_speed: 45.0,
            initial_position: position,
            ..Default::default()
        }
    }
    
    pub fn new_energy(value: f32, position: Vector3<f32>) -> Self {
        Self {
            loot_type: LootType::Energy,
            value,
            pickup_range: 1.1,
            attraction_range: 2.8,
            attraction_speed: 7.0,
            lifetime: 20.0,
            bob_speed: 2.5,
            bob_height: 0.4,
            rotation_speed: 120.0,
            initial_position: position,
            ..Default::default()
        }
    }
    
    pub fn new_currency(value: f32, position: Vector3<f32>) -> Self {
        Self {
            loot_type: LootType::Currency,
            value,
            pickup_range: 1.0,
            attraction_range: 2.0,
            attraction_speed: 5.0,
            lifetime: 45.0, // Currency lasts longer
            bob_speed: 1.0,
            bob_height: 0.15,
            rotation_speed: 180.0,
            initial_position: position,
            ..Default::default()
        }
    }
    
    pub fn is_expired(&self) -> bool {
        self.age >= self.lifetime
    }
    
    pub fn should_attract(&self, player_position: Vector3<f32>) -> bool {
        let distance = (player_position - self.initial_position).magnitude();
        distance <= self.attraction_range
    }
    
    pub fn can_pickup(&self, player_position: Vector3<f32>, current_position: Vector3<f32>) -> bool {
        let distance = (player_position - current_position).magnitude();
        distance <= self.pickup_range
    }
}

impl ScriptTrait for LootItem {
    fn on_init(&mut self, ctx: &mut ScriptContext) {
        if let Some(transform) = ctx.scene.graph[ctx.handle].cast_mut::<Transform>() {
            self.initial_position = transform.position();
        }
    }
    
    fn on_start(&mut self, _ctx: &mut ScriptContext) {
        // Start logic
    }
    
    fn on_update(&mut self, ctx: &mut ScriptContext) {
        if self.is_collected || self.is_expired() {
            return;
        }
        
        self.age += ctx.dt;
        
        if let Some(transform) = ctx.scene.graph[ctx.handle].cast_mut::<Transform>() {
            let current_pos = transform.position();
            
            // Find player position (this would need to be passed in or retrieved)
            let player_position = self.find_player_position(ctx);
            
            // Check for attraction
            if !self.is_attracted && self.should_attract(player_position) {
                self.is_attracted = true;
            }
            
            // Update position
            let new_position = if self.is_attracted {
                self.update_attracted_position(current_pos, player_position, ctx.dt)
            } else {
                self.update_bobbing_position(ctx.dt)
            };
            
            transform.set_position(new_position);
            
            // Update rotation
            let current_rotation = transform.rotation();
            let rotation_delta = self.rotation_speed * ctx.dt * std::f32::consts::PI / 180.0;
            let new_rotation = current_rotation * fyrox::core::algebra::UnitQuaternion::from_axis_angle(
                &fyrox::core::algebra::Vector3::y_axis(), 
                rotation_delta
            );
            transform.set_rotation(new_rotation);
            
            // Check for pickup
            if self.can_pickup(player_position, new_position) {
                self.is_collected = true;
                // Trigger pickup event
                // This would need to communicate with the game system
            }
        }
    }
}

impl LootItem {
    fn find_player_position(&self, _ctx: &ScriptContext) -> Vector3<f32> {
        // TODO: Implement proper player position finding
        // This would typically involve searching the scene for the player entity
        Vector3::default()
    }
    
    fn update_attracted_position(&self, current_pos: Vector3<f32>, player_pos: Vector3<f32>, dt: f32) -> Vector3<f32> {
        let direction = (player_pos - current_pos).normalize();
        current_pos + direction * self.attraction_speed * dt
    }
    
    fn update_bobbing_position(&self, dt: f32) -> Vector3<f32> {
        let bob_offset = (self.age * self.bob_speed).sin() * self.bob_height;
        Vector3::new(
            self.initial_position.x,
            self.initial_position.y + bob_offset,
            self.initial_position.z,
        )
    }
}

#[derive(Clone, Debug, Default)]
pub struct LootManager {
    pub experience_multiplier: f32,
    pub health_multiplier: f32,
    pub energy_multiplier: f32,
    pub currency_multiplier: f32,
    pub attraction_range_bonus: f32,
}

impl LootManager {
    pub fn new() -> Self {
        Self {
            experience_multiplier: 1.0,
            health_multiplier: 1.0,
            energy_multiplier: 1.0,
            currency_multiplier: 1.0,
            attraction_range_bonus: 0.0,
        }
    }
    
    pub fn create_loot_drop(&self, drop_type: &crate::enemy::LootDrop, position: Vector3<f32>) -> LootItem {
        match drop_type {
            crate::enemy::LootDrop::Experience(value) => {
                let mut item = LootItem::new_experience(*value * self.experience_multiplier, position);
                item.attraction_range += self.attraction_range_bonus;
                item
            },
            crate::enemy::LootDrop::Health(value) => {
                let mut item = LootItem::new_health(*value * self.health_multiplier, position);
                item.attraction_range += self.attraction_range_bonus;
                item
            },
            crate::enemy::LootDrop::Energy(value) => {
                let mut item = LootItem::new_energy(*value * self.energy_multiplier, position);
                item.attraction_range += self.attraction_range_bonus;
                item
            },
            crate::enemy::LootDrop::Currency(value) => {
                let mut item = LootItem::new_currency(*value as f32 * self.currency_multiplier, position);
                item.attraction_range += self.attraction_range_bonus;
                item
            },
        }
    }
    
    pub fn apply_magnet_upgrade(&mut self, bonus_range: f32) {
        self.attraction_range_bonus += bonus_range;
    }
    
    pub fn apply_value_multiplier(&mut self, multiplier: f32) {
        self.experience_multiplier *= multiplier;
        self.currency_multiplier *= multiplier;
    }
}
