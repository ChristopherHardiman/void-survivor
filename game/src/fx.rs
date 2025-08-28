//! Visual effects system - handles particles, explosions, and other visual feedback
use bevy::prelude::*;
use crate::GameState;

pub struct FXPlugin;

impl Plugin for FXPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ParticleManager>()
            .add_systems(Update, (
                update_particles_system,
                cleanup_particles_system,
            ).run_if(in_state(GameState::Playing)));
    }
}

#[derive(Resource, Default)]
pub struct ParticleManager {
    pub particles: Vec<Entity>,
}

#[derive(Component)]
pub struct Particle {
    pub velocity: Vec3,
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub size: f32,
    pub color: Color,
}

#[derive(Component)]
pub struct Explosion {
    pub radius: f32,
    pub max_radius: f32,
    pub lifetime: f32,
    pub max_lifetime: f32,
}

impl ParticleManager {
    pub fn spawn_explosion(
        &mut self,
        commands: &mut Commands,
        position: Vec3,
        particle_count: u32,
    ) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        for _ in 0..particle_count {
            let velocity = Vec3::new(
                rng.gen_range(-200.0..200.0),
                rng.gen_range(-200.0..200.0),
                0.0,
            );
            
            let particle = commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(1.0, 0.5, 0.0), // Orange explosion
                        custom_size: Some(Vec2::new(4.0, 4.0)),
                        ..default()
                    },
                    transform: Transform::from_translation(position),
                    ..default()
                },
                Particle {
                    velocity,
                    lifetime: 0.0,
                    max_lifetime: rng.gen_range(0.5..1.5),
                    size: rng.gen_range(2.0..6.0),
                    color: Color::rgb(1.0, rng.gen_range(0.3..0.8), 0.0),
                },
                crate::GameEntity,
            )).id();
            
            self.particles.push(particle);
        }
    }
    
    pub fn spawn_damage_numbers(
        &mut self,
        commands: &mut Commands,
        position: Vec3,
        damage: f32,
    ) {
        // TODO: Implement damage number text spawning
        info!("Damage: {} at position {:?}", damage, position);
    }
    
    pub fn spawn_pickup_effect(
        &mut self,
        commands: &mut Commands,
        position: Vec3,
        effect_type: PickupEffectType,
    ) {
        let color = match effect_type {
            PickupEffectType::Health => Color::GREEN,
            PickupEffectType::Ammo => Color::BLUE,
            PickupEffectType::XP => Color::YELLOW,
        };
        
        // Spawn rising particle effect
        let particle = commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(8.0, 8.0)),
                    ..default()
                },
                transform: Transform::from_translation(position),
                ..default()
            },
            Particle {
                velocity: Vec3::new(0.0, 100.0, 0.0), // Rise upward
                lifetime: 0.0,
                max_lifetime: 1.0,
                size: 8.0,
                color,
            },
            crate::GameEntity,
        )).id();
        
        self.particles.push(particle);
    }
}

#[derive(Clone, Debug)]
pub enum PickupEffectType {
    Health,
    Ammo,
    XP,
}

pub fn update_particles_system(
    time: Res<Time>,
    mut particle_query: Query<(Entity, &mut Particle, &mut Transform, &mut Sprite)>,
    mut particle_manager: ResMut<ParticleManager>,
) {
    let dt = time.delta_seconds();
    
    for (entity, mut particle, mut transform, mut sprite) in particle_query.iter_mut() {
        // Update particle lifetime
        particle.lifetime += dt;
        
        // Update position based on velocity
        transform.translation += particle.velocity * dt;
        
        // Update visual properties based on lifetime progress
        let life_progress = particle.lifetime / particle.max_lifetime;
        
        // Fade out over time
        let alpha = (1.0 - life_progress).max(0.0);
        particle.color.set_a(alpha);
        sprite.color = particle.color;
        
        // Shrink size over time
        let size_factor = (1.0 - life_progress * 0.5).max(0.1);
        let current_size = particle.size * size_factor;
        sprite.custom_size = Some(Vec2::new(current_size, current_size));
        
        // Apply gravity/drag to velocity
        particle.velocity *= 0.98; // Drag
        particle.velocity.y -= 200.0 * dt; // Gravity
    }
}

pub fn cleanup_particles_system(
    mut commands: Commands,
    particle_query: Query<(Entity, &Particle)>,
    mut particle_manager: ResMut<ParticleManager>,
) {
    let mut entities_to_remove = Vec::new();
    
    for (entity, particle) in particle_query.iter() {
        if particle.lifetime >= particle.max_lifetime {
            commands.entity(entity).despawn();
            entities_to_remove.push(entity);
        }
    }
    
    // Remove despawned entities from particle manager
    particle_manager.particles.retain(|&e| !entities_to_remove.contains(&e));
}
