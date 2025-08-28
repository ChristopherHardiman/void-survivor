//! Audio system - sound effects, music, and audio management
use fyrox::{
    core::{
        pool::Handle,
        reflect::prelude::*,
        visitor::prelude::*,
    },
    resource::sound::{Sound, SoundSource},
    scene::sound::{SoundBuilder, Status},
    scene::Scene,
};

use crate::{
    player::WeaponType,
    enemy::EnemyType,
    loot::LootType,
};

#[derive(Clone, Debug, Reflect, Visit, Default)]
pub struct AudioManager {
    // Music
    pub background_music: Option<Handle<Sound>>,
    pub shop_music: Option<Handle<Sound>>,
    
    // Weapon sounds
    pub blaster_fire: Option<Handle<Sound>>,
    pub laser_charge: Option<Handle<Sound>>,
    pub laser_fire: Option<Handle<Sound>>,
    pub rocket_launch: Option<Handle<Sound>>,
    pub aoe_pulse: Option<Handle<Sound>>,
    
    // Impact sounds
    pub impact_metal: Option<Handle<Sound>>,
    pub impact_energy: Option<Handle<Sound>>,
    pub explosion_small: Option<Handle<Sound>>,
    pub explosion_medium: Option<Handle<Sound>>,
    pub explosion_large: Option<Handle<Sound>>,
    
    // Enemy sounds
    pub chaser_move: Option<Handle<Sound>>,
    pub chaser_attack: Option<Handle<Sound>>,
    pub shooter_fire: Option<Handle<Sound>>,
    pub tank_move: Option<Handle<Sound>>,
    pub tank_fire: Option<Handle<Sound>>,
    
    // Loot and UI sounds
    pub loot_pickup: Option<Handle<Sound>>,
    pub experience_gain: Option<Handle<Sound>>,
    pub level_up: Option<Handle<Sound>>,
    pub shop_buy: Option<Handle<Sound>>,
    pub shop_error: Option<Handle<Sound>>,
    pub ui_click: Option<Handle<Sound>>,
    pub ui_hover: Option<Handle<Sound>>,
    
    // Player feedback
    pub player_hit: Option<Handle<Sound>>,
    pub shield_down: Option<Handle<Sound>>,
    pub health_low: Option<Handle<Sound>>,
    
    // Wave events
    pub wave_start: Option<Handle<Sound>>,
    pub wave_complete: Option<Handle<Sound>>,
    
    // Audio settings
    pub master_volume: f32,
    pub music_volume: f32,
    pub sfx_volume: f32,
    pub is_music_enabled: bool,
    pub is_sfx_enabled: bool,
}

impl AudioManager {
    pub fn new() -> Self {
        Self {
            master_volume: 1.0,
            music_volume: 0.7,
            sfx_volume: 0.8,
            is_music_enabled: true,
            is_sfx_enabled: true,
            ..Default::default()
        }
    }
    
    pub fn load_audio_assets(&mut self, scene: &mut Scene) {
        // Load music tracks
        // self.background_music = Some(scene.load_sound("assets/audio/music/background.ogg"));
        // self.shop_music = Some(scene.load_sound("assets/audio/music/shop.ogg"));
        
        // Load weapon sounds
        // self.blaster_fire = Some(scene.load_sound("assets/audio/weapons/blaster_fire.wav"));
        // self.laser_charge = Some(scene.load_sound("assets/audio/weapons/laser_charge.wav"));
        // self.laser_fire = Some(scene.load_sound("assets/audio/weapons/laser_fire.wav"));
        // self.rocket_launch = Some(scene.load_sound("assets/audio/weapons/rocket_launch.wav"));
        // self.aoe_pulse = Some(scene.load_sound("assets/audio/weapons/aoe_pulse.wav"));
        
        // Load impact sounds
        // self.impact_metal = Some(scene.load_sound("assets/audio/impacts/metal_hit.wav"));
        // self.impact_energy = Some(scene.load_sound("assets/audio/impacts/energy_hit.wav"));
        // self.explosion_small = Some(scene.load_sound("assets/audio/explosions/small_explosion.wav"));
        // self.explosion_medium = Some(scene.load_sound("assets/audio/explosions/medium_explosion.wav"));
        // self.explosion_large = Some(scene.load_sound("assets/audio/explosions/large_explosion.wav"));
        
        // Load UI sounds
        // self.loot_pickup = Some(scene.load_sound("assets/audio/ui/loot_pickup.wav"));
        // self.experience_gain = Some(scene.load_sound("assets/audio/ui/exp_gain.wav"));
        // self.level_up = Some(scene.load_sound("assets/audio/ui/level_up.wav"));
        // self.shop_buy = Some(scene.load_sound("assets/audio/ui/shop_buy.wav"));
        // self.ui_click = Some(scene.load_sound("assets/audio/ui/click.wav"));
        
        println!("Audio assets loaded (placeholder implementation)");
    }
    
    pub fn play_weapon_sound(&self, weapon_type: WeaponType, scene: &mut Scene) {
        if !self.is_sfx_enabled {
            return;
        }
        
        let sound_handle = match weapon_type {
            WeaponType::Blaster => self.blaster_fire,
            WeaponType::Laser => self.laser_fire,
            WeaponType::Rocket => self.rocket_launch,
            WeaponType::AoePulse => self.aoe_pulse,
        };
        
        if let Some(handle) = sound_handle {
            self.play_sound_effect(handle, scene, 1.0);
        }
    }
    
    pub fn play_impact_sound(&self, impact_type: ImpactType, scene: &mut Scene) {
        if !self.is_sfx_enabled {
            return;
        }
        
        let sound_handle = match impact_type {
            ImpactType::Metal => self.impact_metal,
            ImpactType::Energy => self.impact_energy,
            ImpactType::ExplosionSmall => self.explosion_small,
            ImpactType::ExplosionMedium => self.explosion_medium,
            ImpactType::ExplosionLarge => self.explosion_large,
        };
        
        if let Some(handle) = sound_handle {
            self.play_sound_effect(handle, scene, 1.0);
        }
    }
    
    pub fn play_enemy_sound(&self, enemy_type: EnemyType, action: EnemyAction, scene: &mut Scene) {
        if !self.is_sfx_enabled {
            return;
        }
        
        let sound_handle = match (enemy_type, action) {
            (EnemyType::Chaser, EnemyAction::Move) => self.chaser_move,
            (EnemyType::Chaser, EnemyAction::Attack) => self.chaser_attack,
            (EnemyType::Shooter, EnemyAction::Attack) => self.shooter_fire,
            (EnemyType::Tank, EnemyAction::Move) => self.tank_move,
            (EnemyType::Tank, EnemyAction::Attack) => self.tank_fire,
            _ => None,
        };
        
        if let Some(handle) = sound_handle {
            self.play_sound_effect(handle, scene, 0.8);
        }
    }
    
    pub fn play_loot_sound(&self, loot_type: LootType, scene: &mut Scene) {
        if !self.is_sfx_enabled {
            return;
        }
        
        let (sound_handle, volume) = match loot_type {
            LootType::Experience => (self.experience_gain, 0.6),
            LootType::Health | LootType::Energy | LootType::Currency => (self.loot_pickup, 0.8),
        };
        
        if let Some(handle) = sound_handle {
            self.play_sound_effect(handle, scene, volume);
        }
    }
    
    pub fn play_ui_sound(&self, ui_sound: UISound, scene: &mut Scene) {
        if !self.is_sfx_enabled {
            return;
        }
        
        let sound_handle = match ui_sound {
            UISound::Click => self.ui_click,
            UISound::Hover => self.ui_hover,
            UISound::LevelUp => self.level_up,
            UISound::ShopBuy => self.shop_buy,
            UISound::ShopError => self.shop_error,
        };
        
        if let Some(handle) = sound_handle {
            self.play_sound_effect(handle, scene, 0.7);
        }
    }
    
    pub fn play_player_feedback(&self, feedback: PlayerFeedback, scene: &mut Scene) {
        if !self.is_sfx_enabled {
            return;
        }
        
        let sound_handle = match feedback {
            PlayerFeedback::Hit => self.player_hit,
            PlayerFeedback::ShieldDown => self.shield_down,
            PlayerFeedback::HealthLow => self.health_low,
        };
        
        if let Some(handle) = sound_handle {
            self.play_sound_effect(handle, scene, 1.0);
        }
    }
    
    pub fn play_wave_sound(&self, wave_event: WaveSound, scene: &mut Scene) {
        if !self.is_sfx_enabled {
            return;
        }
        
        let sound_handle = match wave_event {
            WaveSound::Start => self.wave_start,
            WaveSound::Complete => self.wave_complete,
        };
        
        if let Some(handle) = sound_handle {
            self.play_sound_effect(handle, scene, 0.9);
        }
    }
    
    pub fn start_background_music(&self, scene: &mut Scene) {
        if !self.is_music_enabled {
            return;
        }
        
        if let Some(handle) = self.background_music {
            // Set up looping background music
            self.play_music(handle, scene, true);
        }
    }
    
    pub fn start_shop_music(&self, scene: &mut Scene) {
        if !self.is_music_enabled {
            return;
        }
        
        // Stop background music
        self.stop_music(scene);
        
        if let Some(handle) = self.shop_music {
            self.play_music(handle, scene, true);
        }
    }
    
    pub fn stop_music(&self, _scene: &mut Scene) {
        // Stop all music
        // In a real implementation, you'd stop the currently playing music
    }
    
    pub fn set_master_volume(&mut self, volume: f32) {
        self.master_volume = volume.clamp(0.0, 1.0);
    }
    
    pub fn set_music_volume(&mut self, volume: f32) {
        self.music_volume = volume.clamp(0.0, 1.0);
    }
    
    pub fn set_sfx_volume(&mut self, volume: f32) {
        self.sfx_volume = volume.clamp(0.0, 1.0);
    }
    
    pub fn toggle_music(&mut self) {
        self.is_music_enabled = !self.is_music_enabled;
    }
    
    pub fn toggle_sfx(&mut self) {
        self.is_sfx_enabled = !self.is_sfx_enabled;
    }
    
    fn play_sound_effect(&self, _sound_handle: Handle<Sound>, _scene: &mut Scene, volume: f32) {
        // Calculate final volume
        let final_volume = self.master_volume * self.sfx_volume * volume;
        
        // Play the sound effect at the calculated volume
        // In a real implementation, you'd create a sound source and play it
        println!("Playing sound effect at volume: {}", final_volume);
    }
    
    fn play_music(&self, _music_handle: Handle<Sound>, _scene: &mut Scene, looping: bool) {
        // Calculate final volume
        let final_volume = self.master_volume * self.music_volume;
        
        // Play the music track
        // In a real implementation, you'd create a looping sound source
        println!("Playing music (looping: {}) at volume: {}", looping, final_volume);
    }
}

#[derive(Clone, Debug)]
pub enum ImpactType {
    Metal,
    Energy,
    ExplosionSmall,
    ExplosionMedium,
    ExplosionLarge,
}

#[derive(Clone, Debug)]
pub enum EnemyAction {
    Move,
    Attack,
    Death,
}

#[derive(Clone, Debug)]
pub enum UISound {
    Click,
    Hover,
    LevelUp,
    ShopBuy,
    ShopError,
}

#[derive(Clone, Debug)]
pub enum PlayerFeedback {
    Hit,
    ShieldDown,
    HealthLow,
}

#[derive(Clone, Debug)]
pub enum WaveSound {
    Start,
    Complete,
}
