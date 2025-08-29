//! Asteroid system - handles physics-based asteroids that can be pushed around
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::Rng;
use crate::{GameState, GameEntity};

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Playing), spawn_asteroids)
            .add_systems(Update, (
                asteroid_system,
            ).run_if(in_state(GameState::Playing)));
    }
}

#[derive(Component)]
pub struct Asteroid {
    pub size: f32,
    pub mass: f32,
}

impl Asteroid {
    pub fn new(size: f32) -> Self {
        Self {
            size,
            mass: size * size * 0.5, // Mass scales with size squared
        }
    }
}

fn spawn_asteroids(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let mut rng = rand::thread_rng();
    
    // Spawn asteroids in a scattered pattern around the arena
    for _ in 0..20 {
        let size = rng.gen_range(0.5..3.0); // Random size between 0.5 and 3.0
        let x: f32 = rng.gen_range(-300.0..300.0);
        let z: f32 = rng.gen_range(-300.0..300.0);
        
        // Make sure asteroids don't spawn too close to player start position
        let distance_squared = x * x + z * z;
        if distance_squared.sqrt() < 50.0 {
            continue;
        }
        
        commands.spawn((
            SceneBundle {
                scene: asset_server.load("models/orb.gltf#Scene0"),
                transform: Transform::from_xyz(x, size * 0.5, z)
                    .with_scale(Vec3::splat(size)),
                ..default()
            },
            // Physics components
            RigidBody::Dynamic,
            Collider::ball(size * 0.5), // Collision sphere
            ColliderMassProperties::Density(2.0), // Heavier asteroids
            Velocity::zero(),
            Damping {
                linear_damping: 0.5, // More damping to reduce unwanted movement
                angular_damping: 0.8,
            },
            // Lock Y movement to keep asteroids on the same plane
            LockedAxes::TRANSLATION_LOCKED_Y | LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z,
            Asteroid::new(size),
            GameEntity,
            Name::new(format!("Asteroid_{}", size)),
        ));
    }
    
    info!("Spawned {} asteroids", 20);
}

fn asteroid_system(
    mut asteroid_query: Query<(&mut Velocity, &Transform, &Asteroid), With<Asteroid>>,
) {
    // Keep asteroids within arena bounds using physics forces instead of direct transform manipulation
    for (mut velocity, transform, _asteroid) in asteroid_query.iter_mut() {
        let arena_bounds = 500.0;
        let soft_boundary = 450.0;
        
        // Apply gentle force to keep asteroids within bounds
        let mut boundary_force = Vec3::ZERO;
        
        if transform.translation.x.abs() > soft_boundary {
            boundary_force.x = -transform.translation.x.signum() * 50.0;
        }
        if transform.translation.z.abs() > soft_boundary {
            boundary_force.z = -transform.translation.z.signum() * 50.0;
        }
        
        // Apply boundary force
        velocity.linvel += boundary_force * 0.01; // Small force to guide asteroids back
        
        // Hard boundary - teleport if needed (emergency only)
        if transform.translation.x.abs() > arena_bounds || transform.translation.z.abs() > arena_bounds {
            // Stop the asteroid and apply strong reverse force
            velocity.linvel *= 0.1; // Nearly stop the asteroid
        }
    }
}
