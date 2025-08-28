//! Upgrade system - handles player progression and power-ups
use bevy::prelude::*;
use crate::GameState;
use crate::player::{Player, WeaponType};

pub struct UpgradePlugin;

impl Plugin for UpgradePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<UpgradeManager>()
            .add_systems(Update, (
                upgrade_selection_system,
            ).run_if(in_state(GameState::Playing)));
    }
}

#[derive(Resource, Default)]
pub struct UpgradeManager {
    pub available_upgrades: Vec<Upgrade>,
    pub upgrade_selection_active: bool,
}

#[derive(Clone, Debug)]
pub struct Upgrade {
    pub upgrade_type: UpgradeType,
    pub name: String,
    pub description: String,
    pub tier: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub enum UpgradeType {
    HealthIncrease(f32),
    ShieldIncrease(f32),
    DamageMultiplier(f32),
    FireRateIncrease(f32),
    SpeedIncrease(f32),
    WeaponUnlock(WeaponType),
    SpecialAbility(SpecialAbility),
}

#[derive(Clone, Debug, PartialEq)]
pub enum SpecialAbility {
    DoubleShot,
    PiercingShots,
    ExplosiveShots,
    ShieldRegeneration,
    Dash,
}

impl UpgradeManager {
    pub fn generate_random_upgrades(&mut self, player_level: u32) -> Vec<Upgrade> {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let mut upgrades = Vec::new();
        
        // Generate 3 random upgrades
        for _ in 0..3 {
            let upgrade_type = match rng.gen_range(0..7) {
                0 => UpgradeType::HealthIncrease(25.0),
                1 => UpgradeType::ShieldIncrease(15.0),
                2 => UpgradeType::DamageMultiplier(0.2),
                3 => UpgradeType::FireRateIncrease(1.0),
                4 => UpgradeType::SpeedIncrease(50.0),
                5 => {
                    let weapon = match rng.gen_range(0..4) {
                        0 => WeaponType::Blaster,
                        1 => WeaponType::Laser,
                        2 => WeaponType::Rocket,
                        _ => WeaponType::AoePulse,
                    };
                    UpgradeType::WeaponUnlock(weapon)
                }
                _ => {
                    let ability = match rng.gen_range(0..5) {
                        0 => SpecialAbility::DoubleShot,
                        1 => SpecialAbility::PiercingShots,
                        2 => SpecialAbility::ExplosiveShots,
                        3 => SpecialAbility::ShieldRegeneration,
                        _ => SpecialAbility::Dash,
                    };
                    UpgradeType::SpecialAbility(ability)
                }
            };
            
            let upgrade = Upgrade {
                upgrade_type: upgrade_type.clone(),
                name: get_upgrade_name(&upgrade_type),
                description: get_upgrade_description(&upgrade_type),
                tier: (player_level / 5).max(1),
            };
            
            upgrades.push(upgrade);
        }
        
        upgrades
    }
    
    pub fn apply_upgrade(&self, upgrade: &Upgrade, player: &mut Player) {
        match &upgrade.upgrade_type {
            UpgradeType::HealthIncrease(amount) => {
                player.max_health += amount;
                player.health += amount; // Also heal player
            }
            UpgradeType::ShieldIncrease(amount) => {
                player.max_shields += amount;
                player.shields += amount; // Also restore shields
            }
            UpgradeType::DamageMultiplier(multiplier) => {
                player.damage_multiplier += multiplier;
            }
            UpgradeType::FireRateIncrease(increase) => {
                player.fire_rate += increase;
            }
            UpgradeType::SpeedIncrease(increase) => {
                player.speed += increase;
            }
            UpgradeType::WeaponUnlock(weapon) => {
                player.primary_weapon = weapon.clone();
            }
            UpgradeType::SpecialAbility(_ability) => {
                // TODO: Implement special abilities
                info!("Special ability unlocked: {:?}", _ability);
            }
        }
        
        info!("Applied upgrade: {}", upgrade.name);
    }
}

fn get_upgrade_name(upgrade_type: &UpgradeType) -> String {
    match upgrade_type {
        UpgradeType::HealthIncrease(_) => "Health Boost".to_string(),
        UpgradeType::ShieldIncrease(_) => "Shield Boost".to_string(),
        UpgradeType::DamageMultiplier(_) => "Damage Boost".to_string(),
        UpgradeType::FireRateIncrease(_) => "Fire Rate Boost".to_string(),
        UpgradeType::SpeedIncrease(_) => "Speed Boost".to_string(),
        UpgradeType::WeaponUnlock(weapon) => format!("{:?} Weapon", weapon),
        UpgradeType::SpecialAbility(ability) => format!("{:?}", ability),
    }
}

fn get_upgrade_description(upgrade_type: &UpgradeType) -> String {
    match upgrade_type {
        UpgradeType::HealthIncrease(amount) => format!("Increase maximum health by {}", amount),
        UpgradeType::ShieldIncrease(amount) => format!("Increase maximum shields by {}", amount),
        UpgradeType::DamageMultiplier(multiplier) => format!("Increase damage by {}%", (multiplier * 100.0) as u32),
        UpgradeType::FireRateIncrease(increase) => format!("Increase fire rate by {}", increase),
        UpgradeType::SpeedIncrease(increase) => format!("Increase movement speed by {}", increase),
        UpgradeType::WeaponUnlock(weapon) => format!("Unlock the {:?} weapon", weapon),
        UpgradeType::SpecialAbility(ability) => format!("Unlock special ability: {:?}", ability),
    }
}

fn upgrade_selection_system(
    // TODO: Implement upgrade selection UI interaction
) {
    // This will be implemented when the upgrade UI is complete
}
