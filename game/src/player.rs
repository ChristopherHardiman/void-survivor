//! Player system - handles player movement, shooting, and health
use bevy::prelude::*;
use bevy::render::mesh::shape;
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
fn spawn_player(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    info!("Spawning player");
    
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule {
                radius: 0.3,
                depth: 1.0,
                ..default()
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::BLUE,
                metallic: 0.1,
                perceptual_roughness: 0.8,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.5, 0.0), // Lift off ground
            ..default()
        },
        Player::new(),
        PlayerMovement {
            velocity: Vec3::ZERO,
            max_speed: 300.0,
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
    mut query: Query<(&mut Transform, &mut PlayerMovement, &Player)>,
) {
    for (mut transform, mut movement, player) in query.iter_mut() {
        if !player.is_alive {
            continue;
        }
        
        let mut direction = Vec3::ZERO;
        
        // WASD movement
        if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
            direction.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
            direction.x += 1.0;
        }
        
        // Normalize diagonal movement
        if direction.length() > 0.0 {
            direction = direction.normalize();
        }
        
        // Apply movement
        movement.velocity = direction * movement.max_speed;
        transform.translation += movement.velocity * time.delta_seconds();
        
        // Keep player in bounds (simple arena bounds)
        let arena_bounds = 400.0;
        transform.translation.x = transform.translation.x.clamp(-arena_bounds, arena_bounds);
        transform.translation.y = transform.translation.y.clamp(-arena_bounds, arena_bounds);
    }
}

fn player_shooting_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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
                            &mut meshes,
                            &mut materials,
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
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
    direction: Vec3,
    weapon_type: &WeaponType,
    damage: f32,
) {
    let (speed, lifetime, scale, color) = match weapon_type {
        WeaponType::Blaster => (800.0, 2.0, 0.3, Color::YELLOW),
        WeaponType::Laser => (1200.0, 1.5, 0.25, Color::RED),
        WeaponType::Rocket => (600.0, 3.0, 0.4, Color::ORANGE),
        WeaponType::AoePulse => (400.0, 1.0, 0.35, Color::PURPLE),
    };
    
    // Calculate rotation to face the direction of travel
    let rotation = if direction.length() > 0.0 {
        Quat::from_rotation_z(direction.y.atan2(direction.x) - std::f32::consts::FRAC_PI_2)
    } else {
        Quat::IDENTITY
    };
    
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cylinder {
                radius: 0.08,
                height: 0.3,
                resolution: 8,
                segments: 1,
            })),
            material: materials.add(StandardMaterial {
                base_color: color,
                emissive: color * 0.3, // Make projectiles glow
                metallic: 0.2,
                perceptual_roughness: 0.7,
                ..default()
            }),
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
