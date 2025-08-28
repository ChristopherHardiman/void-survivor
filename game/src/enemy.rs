//! Enemy system - handles different enemy types, AI behavior, and spawning
use bevy::prelude::*;
use crate::{GameState, GameEntity};
use crate::player::{Player, Projectile};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                enemy_ai_system,
                enemy_combat_system,
                enemy_health_system,
                projectile_collision_system,
            ).run_if(in_state(GameState::Playing)));
    }
}

#[derive(Component)]
pub struct Enemy {
    pub health: f32,
    pub max_health: f32,
    pub damage: f32,
    pub speed: f32,
    pub enemy_type: EnemyType,
    pub last_damage_time: f32,
    pub experience_value: f32,
    pub loot_chance: f32,
}

#[derive(Component)]
pub struct EnemyMovement {
    pub velocity: Vec3,
    pub target_position: Vec3,
    pub behavior: MovementBehavior,
}

#[derive(Clone, Debug, PartialEq)]
pub enum EnemyType {
    Chaser,
    Shooter,
    Tank,
    Swarm,
    Elite,
    Boss,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MovementBehavior {
    ChasePlayer,
    Patrol(Vec3, Vec3), // Start point, end point
    Circle(Vec3, f32),  // Center, radius
    Stationary,
}

impl Default for Enemy {
    fn default() -> Self {
        Self::new(EnemyType::Chaser)
    }
}

impl Enemy {
    pub fn new(enemy_type: EnemyType) -> Self {
        match enemy_type {
            EnemyType::Chaser => Self {
                health: 50.0,
                max_health: 50.0,
                damage: 20.0,
                speed: 100.0,
                enemy_type,
                last_damage_time: 0.0,
                experience_value: 10.0,
                loot_chance: 0.1,
            },
            EnemyType::Shooter => Self {
                health: 30.0,
                max_health: 30.0,
                damage: 15.0,
                speed: 60.0,
                enemy_type,
                last_damage_time: 0.0,
                experience_value: 15.0,
                loot_chance: 0.15,
            },
            EnemyType::Tank => Self {
                health: 150.0,
                max_health: 150.0,
                damage: 35.0,
                speed: 40.0,
                enemy_type,
                last_damage_time: 0.0,
                experience_value: 25.0,
                loot_chance: 0.3,
            },
            EnemyType::Swarm => Self {
                health: 20.0,
                max_health: 20.0,
                damage: 10.0,
                speed: 150.0,
                enemy_type,
                last_damage_time: 0.0,
                experience_value: 5.0,
                loot_chance: 0.05,
            },
            EnemyType::Elite => Self {
                health: 100.0,
                max_health: 100.0,
                damage: 40.0,
                speed: 80.0,
                enemy_type,
                last_damage_time: 0.0,
                experience_value: 50.0,
                loot_chance: 0.5,
            },
            EnemyType::Boss => Self {
                health: 500.0,
                max_health: 500.0,
                damage: 60.0,
                speed: 50.0,
                enemy_type,
                last_damage_time: 0.0,
                experience_value: 200.0,
                loot_chance: 0.9,
            },
        }
    }
    
    pub fn take_damage(&mut self, damage: f32, current_time: f32) -> bool {
        self.health -= damage;
        self.last_damage_time = current_time;
        self.health <= 0.0
    }
    
    pub fn is_dead(&self) -> bool {
        self.health <= 0.0
    }
}

// Helper function to spawn different enemy types
pub fn spawn_enemy(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    enemy_type: EnemyType,
    position: Vec3,
) {
    let enemy = Enemy::new(enemy_type.clone());
    let texture_path = match enemy_type {
        EnemyType::Chaser => "sprites/enemy_chaser.png",
        EnemyType::Shooter => "sprites/enemy_shooter.png",
        EnemyType::Tank => "sprites/enemy_tank.png",
        EnemyType::Swarm => "sprites/enemy_swarm.png",
        EnemyType::Elite => "sprites/enemy_elite.png",
        EnemyType::Boss => "sprites/enemy_boss.png",
    };
    
    let scale = match enemy_type {
        EnemyType::Swarm => 0.3,
        EnemyType::Boss => 1.5,
        _ => 0.5,
    };
    
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(texture_path),
            transform: Transform::from_translation(position)
                .with_scale(Vec3::splat(scale)),
            ..default()
        },
        enemy,
        EnemyMovement {
            velocity: Vec3::ZERO,
            target_position: position,
            behavior: MovementBehavior::ChasePlayer,
        },
        GameEntity,
        Name::new(format!("Enemy_{:?}", enemy_type)),
    ));
}

// AI System
fn enemy_ai_system(
    time: Res<Time>,
    mut enemy_query: Query<(&mut Transform, &mut EnemyMovement, &Enemy)>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (mut transform, mut movement, enemy) in enemy_query.iter_mut() {
            let dt = time.delta_seconds();
            
            match movement.behavior {
                MovementBehavior::ChasePlayer => {
                    // Calculate direction to player
                    let direction = (player_transform.translation - transform.translation).normalize();
                    movement.velocity = direction * enemy.speed;
                    
                    // Apply movement
                    transform.translation += movement.velocity * dt;
                }
                MovementBehavior::Patrol(start, end) => {
                    // Simple patrol between two points
                    let to_target = movement.target_position - transform.translation;
                    if to_target.length() < 10.0 {
                        // Switch target
                        movement.target_position = if movement.target_position.distance(start) < 10.0 {
                            end
                        } else {
                            start
                        };
                    }
                    
                    let direction = to_target.normalize();
                    movement.velocity = direction * enemy.speed;
                    transform.translation += movement.velocity * dt;
                }
                MovementBehavior::Circle(center, radius) => {
                    // Circular movement around a point
                    let current_angle = (transform.translation - center).truncate().angle_between(Vec2::X);
                    let new_angle = current_angle + enemy.speed * dt / radius;
                    
                    let new_pos = center + Vec3::new(
                        new_angle.cos() * radius,
                        new_angle.sin() * radius,
                        transform.translation.z,
                    );
                    
                    movement.velocity = (new_pos - transform.translation) / dt;
                    transform.translation = new_pos;
                }
                MovementBehavior::Stationary => {
                    movement.velocity = Vec3::ZERO;
                }
            }
        }
    }
}

// Combat System
fn enemy_combat_system(
    mut commands: Commands,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (enemy_transform, mut enemy) in enemy_query.iter_mut() {
            let distance = enemy_transform.translation.distance(player_transform.translation);
            
            // Different combat behaviors based on enemy type
            match enemy.enemy_type {
                EnemyType::Shooter | EnemyType::Elite | EnemyType::Boss => {
                    if distance < 300.0 && time.elapsed_seconds() - enemy.last_damage_time > 2.0 {
                        // Spawn enemy projectile
                        let direction = (player_transform.translation - enemy_transform.translation).normalize();
                        spawn_enemy_projectile(
                            &mut commands,
                            &asset_server,
                            enemy_transform.translation,
                            direction,
                            enemy.damage,
                        );
                        enemy.last_damage_time = time.elapsed_seconds();
                    }
                }
                _ => {
                    // Melee enemies just chase - damage is handled by collision
                }
            }
        }
    }
}

// Health System
fn enemy_health_system(
    mut commands: Commands,
    mut enemy_query: Query<(Entity, &Enemy)>,
    mut player_query: Query<&mut Player>,
) {
    for (entity, enemy) in enemy_query.iter() {
        if enemy.is_dead() {
            // Give experience to player
            if let Ok(mut player) = player_query.get_single_mut() {
                let leveled_up = player.add_experience(enemy.experience_value);
                if leveled_up {
                    info!("Player leveled up to level {}!", player.level);
                }
            }
            
            // TODO: Spawn loot based on loot_chance
            // TODO: Spawn death effects
            
            commands.entity(entity).despawn();
        }
    }
}

// Projectile collision system
fn projectile_collision_system(
    mut commands: Commands,
    time: Res<Time>,
    projectile_query: Query<(Entity, &Transform, &Projectile)>,
    mut enemy_query: Query<(Entity, &Transform, &mut Enemy)>,
    mut player_query: Query<&mut Player>,
) {
    let current_time = time.elapsed_seconds();
    
    for (projectile_entity, projectile_transform, projectile) in projectile_query.iter() {
        for (enemy_entity, enemy_transform, mut enemy) in enemy_query.iter_mut() {
            let distance = projectile_transform.translation.distance(enemy_transform.translation);
            
            if distance < 20.0 { // Simple collision detection
                let is_dead = enemy.take_damage(projectile.damage, current_time);
                commands.entity(projectile_entity).despawn();
                
                if is_dead {
                    info!("Enemy destroyed!");
                }
                break;
            }
        }
    }
}

// Enemy projectile system
#[derive(Component)]
pub struct EnemyProjectile {
    pub velocity: Vec3,
    pub damage: f32,
    pub lifetime: f32,
    pub max_lifetime: f32,
}

fn spawn_enemy_projectile(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    position: Vec3,
    direction: Vec3,
    damage: f32,
) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/enemy_projectile.png"),
            transform: Transform::from_translation(position)
                .with_scale(Vec3::splat(0.2)),
            ..default()
        },
        EnemyProjectile {
            velocity: direction * 400.0,
            damage,
            lifetime: 0.0,
            max_lifetime: 3.0,
        },
        GameEntity,
        Name::new("EnemyProjectile"),
    ));
}

// Update enemy projectiles
pub fn update_enemy_projectiles_system(
    mut commands: Commands,
    time: Res<Time>,
    mut projectile_query: Query<(Entity, &mut Transform, &mut EnemyProjectile)>,
    mut player_query: Query<(&Transform, &mut Player), Without<EnemyProjectile>>,
) {
    if let Ok((player_transform, mut player)) = player_query.get_single_mut() {
        for (entity, mut transform, mut projectile) in projectile_query.iter_mut() {
            // Move projectile
            transform.translation += projectile.velocity * time.delta_seconds();
            
            // Update lifetime
            projectile.lifetime += time.delta_seconds();
            
            // Check collision with player
            let distance = transform.translation.distance(player_transform.translation);
            if distance < 25.0 {
                player.take_damage(projectile.damage);
                commands.entity(entity).despawn();
                continue;
            }
            
            // Remove if expired or out of bounds
            if projectile.lifetime >= projectile.max_lifetime || transform.translation.length() > 1000.0 {
                commands.entity(entity).despawn();
            }
        }
    }
}
