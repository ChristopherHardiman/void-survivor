//! Loot system - handles item drops, pickups, and rewards
use bevy::prelude::*;
use crate::{GameState, GameEntity};
use crate::player::Player;

pub struct LootPlugin;

impl Plugin for LootPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                loot_pickup_system,
                loot_magnet_system,
            ).run_if(in_state(GameState::Playing)));
    }
}

#[derive(Component)]
pub struct Loot {
    pub loot_type: LootType,
    pub value: f32,
    pub magnetic: bool,
    pub pickup_range: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub enum LootType {
    Experience,
    Health,
    Shield,
    Weapon,
    PowerUp,
}

impl Loot {
    pub fn new(loot_type: LootType, value: f32) -> Self {
        Self {
            loot_type,
            value,
            magnetic: true,
            pickup_range: 30.0,
        }
    }
}

pub fn spawn_loot(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    loot_type: LootType,
    position: Vec3,
    value: f32,
) {
    let (texture_path, scale) = match loot_type {
        LootType::Experience => ("sprites/loot_exp.png", 0.3),
        LootType::Health => ("sprites/loot_health.png", 0.4),
        LootType::Shield => ("sprites/loot_shield.png", 0.4),
        LootType::Weapon => ("sprites/loot_weapon.png", 0.5),
        LootType::PowerUp => ("sprites/loot_powerup.png", 0.4),
    };
    
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(texture_path),
            transform: Transform::from_translation(position)
                .with_scale(Vec3::splat(scale)),
            ..default()
        },
        Loot::new(loot_type, value),
        GameEntity,
        Name::new("Loot"),
    ));
}

fn loot_pickup_system(
    mut commands: Commands,
    loot_query: Query<(Entity, &Transform, &Loot)>,
    mut player_query: Query<(&Transform, &mut Player), Without<Loot>>,
) {
    if let Ok((player_transform, mut player)) = player_query.get_single_mut() {
        for (loot_entity, loot_transform, loot) in loot_query.iter() {
            let distance = player_transform.translation.distance(loot_transform.translation);
            
            if distance <= loot.pickup_range {
                match loot.loot_type {
                    LootType::Experience => {
                        player.add_experience(loot.value);
                    }
                    LootType::Health => {
                        player.health = (player.health + loot.value).min(player.max_health);
                    }
                    LootType::Shield => {
                        player.shields = (player.shields + loot.value).min(player.max_shields);
                    }
                    LootType::Weapon => {
                        // TODO: Implement weapon pickup
                    }
                    LootType::PowerUp => {
                        // TODO: Implement power-up effects
                    }
                }
                
                commands.entity(loot_entity).despawn();
            }
        }
    }
}

fn loot_magnet_system(
    time: Res<Time>,
    mut loot_query: Query<(&mut Transform, &Loot)>,
    player_query: Query<&Transform, (With<Player>, Without<Loot>)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (mut loot_transform, loot) in loot_query.iter_mut() {
            if !loot.magnetic {
                continue;
            }
            
            let distance = player_transform.translation.distance(loot_transform.translation);
            let magnet_range = 100.0;
            
            if distance <= magnet_range {
                let direction = (player_transform.translation - loot_transform.translation).normalize();
                let magnet_speed = 200.0;
                loot_transform.translation += direction * magnet_speed * time.delta_seconds();
            }
        }
    }
}
