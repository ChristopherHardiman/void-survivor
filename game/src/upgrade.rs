//! Upgrade system - level-up choices, shop items, and player progression
use fyrox::{
    core::{
        reflect::prelude::*,
        visitor::prelude::*,
    },
};

use crate::player::{Player, WeaponType};

#[derive(Clone, Debug, Reflect, Visit)]
pub enum UpgradeType {
    // Weapon upgrades
    WeaponDamage(f32),
    WeaponFireRate(f32),
    WeaponProjectileCount(u32),
    WeaponRange(f32),
    WeaponNewType(WeaponType),
    
    // Ship upgrades
    ShipSpeed(f32),
    ShipHealth(f32),
    ShipShields(f32),
    ShipArmor(f32),
    
    // Special abilities
    SpecialEnergyMax(f32),
    SpecialEnergyRegen(f32),
    AbilityAoe,
    AbilityDrone,
    AbilityShield,
    
    // Utility
    ExpMultiplier(f32),
    LootMagnet(f32),
    HealthRegen(f32),
}

#[derive(Clone, Debug, Reflect, Visit, Default)]
pub struct Upgrade {
    pub upgrade_type: UpgradeType,
    pub name: String,
    pub description: String,
    pub cost: u32,
    pub level: u32,
    pub max_level: u32,
    pub is_shop_item: bool, // true for shop, false for level-up rewards
}

impl Default for UpgradeType {
    fn default() -> Self {
        UpgradeType::WeaponDamage(0.0)
    }
}

impl Upgrade {
    // Level-up upgrade constructors
    pub fn weapon_damage_boost() -> Self {
        Self {
            upgrade_type: UpgradeType::WeaponDamage(0.2),
            name: "Weapon Power".to_string(),
            description: "+20% weapon damage".to_string(),
            cost: 0,
            level: 0,
            max_level: 10,
            is_shop_item: false,
        }
    }
    
    pub fn fire_rate_boost() -> Self {
        Self {
            upgrade_type: UpgradeType::WeaponFireRate(0.3),
            name: "Rapid Fire".to_string(),
            description: "+30% fire rate".to_string(),
            cost: 0,
            level: 0,
            max_level: 8,
            is_shop_item: false,
        }
    }
    
    pub fn multishot() -> Self {
        Self {
            upgrade_type: UpgradeType::WeaponProjectileCount(1),
            name: "Multi-Shot".to_string(),
            description: "+1 projectile per shot".to_string(),
            cost: 0,
            level: 0,
            max_level: 3,
            is_shop_item: false,
        }
    }
    
    pub fn speed_boost() -> Self {
        Self {
            upgrade_type: UpgradeType::ShipSpeed(0.15),
            name: "Engine Boost".to_string(),
            description: "+15% movement speed".to_string(),
            cost: 0,
            level: 0,
            max_level: 6,
            is_shop_item: false,
        }
    }
    
    pub fn health_boost() -> Self {
        Self {
            upgrade_type: UpgradeType::ShipHealth(25.0),
            name: "Hull Reinforcement".to_string(),
            description: "+25 max health".to_string(),
            cost: 0,
            level: 0,
            max_level: 8,
            is_shop_item: false,
        }
    }
    
    pub fn shield_boost() -> Self {
        Self {
            upgrade_type: UpgradeType::ShipShields(20.0),
            name: "Shield Upgrade".to_string(),
            description: "+20 max shields".to_string(),
            cost: 0,
            level: 0,
            max_level: 6,
            is_shop_item: false,
        }
    }
    
    // Shop item constructors
    pub fn shop_laser_weapon() -> Self {
        Self {
            upgrade_type: UpgradeType::WeaponNewType(WeaponType::Laser),
            name: "Laser Cannon".to_string(),
            description: "Continuous beam weapon with high damage".to_string(),
            cost: 100,
            level: 0,
            max_level: 1,
            is_shop_item: true,
        }
    }
    
    pub fn shop_rocket_weapon() -> Self {
        Self {
            upgrade_type: UpgradeType::WeaponNewType(WeaponType::Rocket),
            name: "Rocket Launcher".to_string(),
            description: "Explosive projectiles with area damage".to_string(),
            cost: 150,
            level: 0,
            max_level: 1,
            is_shop_item: true,
        }
    }
    
    pub fn shop_aoe_ability() -> Self {
        Self {
            upgrade_type: UpgradeType::AbilityAoe,
            name: "EMP Pulse".to_string(),
            description: "Area of effect damage ability".to_string(),
            cost: 200,
            level: 0,
            max_level: 3,
            is_shop_item: true,
        }
    }
    
    pub fn shop_drone() -> Self {
        Self {
            upgrade_type: UpgradeType::AbilityDrone,
            name: "Combat Drone".to_string(),
            description: "Autonomous drone that fights alongside you".to_string(),
            cost: 250,
            level: 0,
            max_level: 2,
            is_shop_item: true,
        }
    }
    
    pub fn can_upgrade(&self) -> bool {
        self.level < self.max_level
    }
    
    pub fn get_current_cost(&self) -> u32 {
        if self.is_shop_item {
            self.cost * (self.level + 1)
        } else {
            0 // Level-up upgrades are free
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct UpgradeManager {
    pub available_levelup_upgrades: Vec<Upgrade>,
    pub available_shop_items: Vec<Upgrade>,
    pub applied_upgrades: Vec<Upgrade>,
}

impl UpgradeManager {
    pub fn new() -> Self {
        Self {
            available_levelup_upgrades: Self::create_levelup_pool(),
            available_shop_items: Self::create_shop_pool(),
            applied_upgrades: Vec::new(),
        }
    }
    
    pub fn get_levelup_choices(&self, count: usize) -> Vec<Upgrade> {
        // Return random selection of available upgrades
        let mut choices = Vec::new();
        let available: Vec<_> = self.available_levelup_upgrades
            .iter()
            .filter(|upgrade| upgrade.can_upgrade())
            .cloned()
            .collect();
        
        // Simple selection (would use proper randomization in real implementation)
        for i in 0..count.min(available.len()) {
            if i < available.len() {
                choices.push(available[i].clone());
            }
        }
        
        choices
    }
    
    pub fn get_shop_items(&self) -> Vec<Upgrade> {
        self.available_shop_items
            .iter()
            .filter(|item| item.can_upgrade())
            .cloned()
            .collect()
    }
    
    pub fn apply_upgrade(&mut self, upgrade: Upgrade, player: &mut Player) -> bool {
        if !upgrade.can_upgrade() {
            return false;
        }
        
        // Apply the upgrade effect to the player
        self.apply_upgrade_effect(&upgrade, player);
        
        // Update the upgrade level
        if let Some(existing) = self.applied_upgrades.iter_mut()
            .find(|u| std::mem::discriminant(&u.upgrade_type) == std::mem::discriminant(&upgrade.upgrade_type)) {
            existing.level += 1;
        } else {
            let mut new_upgrade = upgrade.clone();
            new_upgrade.level = 1;
            self.applied_upgrades.push(new_upgrade);
        }
        
        // Update available pools
        self.update_available_upgrades(&upgrade);
        
        true
    }
    
    fn apply_upgrade_effect(&self, upgrade: &Upgrade, player: &mut Player) {
        match &upgrade.upgrade_type {
            UpgradeType::WeaponDamage(bonus) => {
                player.damage_multiplier += bonus;
            },
            UpgradeType::WeaponFireRate(bonus) => {
                player.fire_rate *= 1.0 + bonus;
            },
            UpgradeType::WeaponNewType(weapon_type) => {
                player.primary_weapon = weapon_type.clone();
            },
            UpgradeType::ShipSpeed(bonus) => {
                player.speed *= 1.0 + bonus;
            },
            UpgradeType::ShipHealth(bonus) => {
                player.max_health += bonus;
                player.health += bonus; // Also heal when upgrading
            },
            UpgradeType::ShipShields(bonus) => {
                player.max_shields += bonus;
                player.shields += bonus; // Also restore shields
            },
            UpgradeType::SpecialEnergyMax(bonus) => {
                player.max_special_energy += bonus;
            },
            // Add other upgrade types as needed
            _ => {
                // Handle other upgrade types
            }
        }
    }
    
    fn update_available_upgrades(&mut self, applied_upgrade: &Upgrade) {
        // Update level-up pool
        if let Some(upgrade) = self.available_levelup_upgrades.iter_mut()
            .find(|u| std::mem::discriminant(&u.upgrade_type) == std::mem::discriminant(&applied_upgrade.upgrade_type)) {
            upgrade.level += 1;
        }
        
        // Update shop pool
        if let Some(item) = self.available_shop_items.iter_mut()
            .find(|u| std::mem::discriminant(&u.upgrade_type) == std::mem::discriminant(&applied_upgrade.upgrade_type)) {
            item.level += 1;
        }
    }
    
    fn create_levelup_pool() -> Vec<Upgrade> {
        vec![
            Upgrade::weapon_damage_boost(),
            Upgrade::fire_rate_boost(),
            Upgrade::multishot(),
            Upgrade::speed_boost(),
            Upgrade::health_boost(),
            Upgrade::shield_boost(),
        ]
    }
    
    fn create_shop_pool() -> Vec<Upgrade> {
        vec![
            Upgrade::shop_laser_weapon(),
            Upgrade::shop_rocket_weapon(),
            Upgrade::shop_aoe_ability(),
            Upgrade::shop_drone(),
        ]
    }
}
