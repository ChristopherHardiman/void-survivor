//! Player system - handles player movement, shooting, and health
use bevy::prelude::*;
use bevy::input::mouse::MouseButton;
use bevy_rapier3d::prelude::*;
use crate::{GameState, GameEntity};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(Update, (
                player_movement_system,
                player_shooting_system,
                player_stats_system,
                update_projectiles_system,
            ).run_if(in_state(GameState::Playing)));
    }
}

#[derive(Component)]
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
    
    // Weapons
    pub primary_weapon: WeaponType,
    pub fire_rate: f32,
    pub last_shot_time: f32,
    pub damage_multiplier: f32,
    
    // State
    pub is_alive: bool,
}

#[derive(Component)]
pub struct PlayerMovement {
    pub velocity: Vec3,
    pub max_speed: f32,
    pub thrust_force: f32,
    pub turn_speed: f32,
    pub current_direction: f32, // Angle in radians, 0 = forward (positive Z)
}

#[derive(Component)]
pub struct PlayerWeapon {
    pub weapon_type: WeaponType,
    pub last_shot: f32,
    pub fire_rate: f32,
    pub damage: f32,
}

#[derive(Clone, Debug, PartialEq)]
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

impl Default for Player {
    fn default() -> Self {
        Self::new()
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
            speed: 300.0, // pixels per second
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

// Projectile system
#[derive(Component)]
pub struct Projectile {
    pub velocity: Vec3,
    pub damage: f32,
    pub lifetime: f32,
    pub max_lifetime: f32,
}

// Systems
fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Spawning player");
    
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("models/player_fighter.gltf#Scene0"),
            transform: Transform::from_xyz(0.0, 0.5, 0.0) // Lift off ground
                .with_scale(Vec3::splat(0.5)), // Scale down the fighter model
            ..default()
        },
        // Physics components
        RigidBody::Dynamic,
        Collider::capsule_y(0.5, 0.3), // Capsule collider for the ship
        ColliderMassProperties::Density(2.0),
        Velocity::zero(),
        Damping {
            linear_damping: 0.8, // Reduced damping for smoother deceleration
            angular_damping: 5.0, // Ship stops rotating quickly
        },
        LockedAxes::ROTATION_LOCKED | LockedAxes::TRANSLATION_LOCKED_Y, // Lock Y rotation and movement
        Player::new(),
        PlayerMovement {
            velocity: Vec3::ZERO,
            max_speed: 300.0,
            thrust_force: 500.0,
            turn_speed: 3.0,
            current_direction: 0.0, // Start facing forward (positive Z)
        },
        PlayerWeapon {
            weapon_type: WeaponType::Blaster,
            last_shot: 0.0,
            fire_rate: 5.0,
            damage: 20.0,
        },
        GameEntity,
        Name::new("Player"),
    ));
}

fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut PlayerMovement, &mut Velocity, &Player)>,
) {
    for (mut transform, mut movement, mut velocity, player) in query.iter_mut() {
        if !player.is_alive {
            continue;
        }
        
        let dt = time.delta_seconds();
        
        // Handle turning with A and D keys
        if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
            movement.current_direction -= movement.turn_speed * dt;
        }
        if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
            movement.current_direction += movement.turn_speed * dt;
        }
        
        // Normalize direction to keep it between -PI and PI
        while movement.current_direction > std::f32::consts::PI {
            movement.current_direction -= 2.0 * std::f32::consts::PI;
        }
        while movement.current_direction < -std::f32::consts::PI {
            movement.current_direction += 2.0 * std::f32::consts::PI;
        }
        
        // Calculate forward direction based on current_direction
        let forward = Vec3::new(
            movement.current_direction.sin(),
            0.0,
            movement.current_direction.cos(),
        );
        
        // Apply thrust with W key
        if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
            let thrust = forward * movement.thrust_force * dt;
            velocity.linvel += thrust;
        }
        
        // Optional: Add reverse thrust with S key (much weaker)
        if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
            let reverse_thrust = forward * movement.thrust_force * 0.3 * dt;
            velocity.linvel -= reverse_thrust;
        }
        
        // Limit maximum speed
        if velocity.linvel.length() > movement.max_speed {
            velocity.linvel = velocity.linvel.normalize() * movement.max_speed;
        }
        
        // Update ship rotation to face movement direction
        transform.rotation = Quat::from_rotation_y(movement.current_direction);
        
        // Keep player in bounds (arena bounds for X and Z axes)
        let arena_bounds = 400.0;
        if transform.translation.x.abs() > arena_bounds || transform.translation.z.abs() > arena_bounds {
            // Bounce off boundaries by reversing velocity component
            if transform.translation.x.abs() > arena_bounds {
                velocity.linvel.x *= -0.5;
                transform.translation.x = transform.translation.x.signum() * arena_bounds;
            }
            if transform.translation.z.abs() > arena_bounds {
                velocity.linvel.z *= -0.5;
                transform.translation.z = transform.translation.z.signum() * arena_bounds;
            }
        }
    }
}

fn player_shooting_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mouse_input: Res<Input<MouseButton>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<crate::MainCamera>>,
    mut player_query: Query<(&Transform, &mut PlayerWeapon, &mut Player)>,
) {
    for (player_transform, mut weapon, mut player) in player_query.iter_mut() {
        if !player.is_alive {
            continue;
        }
        
        let current_time = time.elapsed_seconds();
        
        // Check if player wants to shoot
        let wants_to_shoot = mouse_input.pressed(MouseButton::Left) || 
                           keyboard_input.pressed(KeyCode::Space);
        
        if wants_to_shoot && player.can_shoot(current_time) {
            // Get mouse position for aiming
            if let Ok(window) = windows.get_single() {
                if let Ok((camera, camera_transform)) = camera_query.get_single() {
                    if let Some(cursor_pos) = window.cursor_position() {
                        // Convert screen coordinates to world coordinates
                        let window_size = Vec2::new(window.width(), window.height());
                        
                        // Convert screen position to world position
                        let ndc = (cursor_pos / window_size) * 2.0 - Vec2::ONE;
                        let ndc = Vec3::new(ndc.x, -ndc.y, 0.0);
                        
                        // Simple world position calculation for 2D
                        let world_pos = Vec3::new(
                            ndc.x * window_size.x * 0.5,
                            ndc.y * window_size.y * 0.5,
                            0.0
                        );
                        
                        // Calculate shooting direction
                        let direction = (world_pos - player_transform.translation).normalize();
                        
                        // Spawn projectile
                        spawn_projectile(
                            &mut commands,
                            &asset_server,
                            player_transform.translation,
                            direction,
                            &weapon.weapon_type,
                            weapon.damage,
                        );
                        
                        // Update last shot time
                        weapon.last_shot = current_time;
                        player.last_shot_time = current_time;
                    }
                }
            }
        }
    }
}

fn player_stats_system(
    time: Res<Time>,
    mut query: Query<&mut Player>,
) {
    for mut player in query.iter_mut() {
        let dt = time.delta_seconds();
        
        // Regenerate shields after delay
        if player.shields < player.max_shields {
            // Simple shield regen - could be made more sophisticated
            player.shields = (player.shields + 10.0 * dt).min(player.max_shields);
        }
        
        // Regenerate special energy
        if player.special_energy < player.max_special_energy {
            player.special_energy = (player.special_energy + 20.0 * dt).min(player.max_special_energy);
        }
    }
}

fn spawn_projectile(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    position: Vec3,
    direction: Vec3,
    weapon_type: &WeaponType,
    damage: f32,
) {
    let (speed, lifetime, scale) = match weapon_type {
        WeaponType::Blaster => (800.0, 2.0, 0.15),
        WeaponType::Laser => (1200.0, 1.5, 0.12),
        WeaponType::Rocket => (600.0, 3.0, 0.2),
        WeaponType::AoePulse => (400.0, 1.0, 0.18),
    };
    
    // Calculate rotation to face the direction of travel
    let rotation = if direction.length() > 0.0 {
        Quat::from_rotation_z(direction.y.atan2(direction.x) - std::f32::consts::FRAC_PI_2)
    } else {
        Quat::IDENTITY
    };
    
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("models/stinger_missle.gltf#Scene0"),
            transform: Transform::from_translation(position)
                .with_rotation(rotation)
                .with_scale(Vec3::splat(scale)),
            ..default()
        },
        Projectile {
            velocity: direction * speed,
            damage,
            lifetime: 0.0,
            max_lifetime: lifetime,
        },
        GameEntity,
        Name::new("Projectile"),
    ));
}

// Update projectiles
pub fn update_projectiles_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut Projectile)>,
) {
    for (entity, mut transform, mut projectile) in query.iter_mut() {
        // Move projectile
        transform.translation += projectile.velocity * time.delta_seconds();
        
        // Update lifetime
        projectile.lifetime += time.delta_seconds();
        
        // Remove if expired
        if projectile.lifetime >= projectile.max_lifetime {
            commands.entity(entity).despawn();
        }
        
        // Remove if out of bounds
        if transform.translation.length() > 1000.0 {
            commands.entity(entity).despawn();
        }
    }
}
