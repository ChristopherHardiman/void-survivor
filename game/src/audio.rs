//! Audio system - handles sound effects and music
use bevy::prelude::*;
use crate::GameState;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<AudioManager>()
            .add_systems(Startup, setup_audio)
            .add_systems(Update, (
                update_audio_system,
            ).run_if(in_state(GameState::Playing)));
    }
}

#[derive(Resource, Default)]
pub struct AudioManager {
    pub master_volume: f32,
    pub sfx_volume: f32,
    pub music_volume: f32,
    pub current_music: Option<Handle<AudioSource>>,
    pub loaded_sounds: std::collections::HashMap<String, Handle<AudioSource>>,
}

#[derive(Component)]
pub struct AudioSource3D {
    pub position: Vec3,
    pub max_distance: f32,
    pub volume: f32,
}

impl AudioManager {
    pub fn new() -> Self {
        Self {
            master_volume: 1.0,
            sfx_volume: 0.8,
            music_volume: 0.6,
            current_music: None,
            loaded_sounds: std::collections::HashMap::new(),
        }
    }
    
    pub fn play_sound_effect(
        &self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        sound_name: &str,
        volume: f32,
    ) {
        // For now, use placeholder sounds since we don't have actual audio files
        // In a real implementation, you would load audio files from assets
        info!("Playing sound effect: {} at volume {}", sound_name, volume);
        
        // TODO: Implement actual audio playback when audio assets are available
        // let sound_handle = asset_server.load(format!("audio/sfx/{}.ogg", sound_name));
        // commands.spawn(AudioBundle {
        //     source: sound_handle,
        //     settings: PlaybackSettings::ONCE.with_volume(Volume::new_relative(volume * self.sfx_volume * self.master_volume)),
        // });
    }
    
    pub fn play_music(
        &mut self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        music_name: &str,
    ) {
        info!("Playing music: {}", music_name);
        
        // TODO: Implement actual music playback when audio assets are available
        // let music_handle = asset_server.load(format!("audio/music/{}.ogg", music_name));
        // self.current_music = Some(music_handle.clone());
        // commands.spawn(AudioBundle {
        //     source: music_handle,
        //     settings: PlaybackSettings::LOOP.with_volume(Volume::new_relative(self.music_volume * self.master_volume)),
        // });
    }
    
    pub fn stop_music(&mut self, commands: &mut Commands) {
        if self.current_music.is_some() {
            info!("Stopping current music");
            self.current_music = None;
            // TODO: Implement actual music stopping
        }
    }
    
    pub fn set_master_volume(&mut self, volume: f32) {
        self.master_volume = volume.clamp(0.0, 1.0);
        info!("Master volume set to: {}", self.master_volume);
    }
    
    pub fn set_sfx_volume(&mut self, volume: f32) {
        self.sfx_volume = volume.clamp(0.0, 1.0);
        info!("SFX volume set to: {}", self.sfx_volume);
    }
    
    pub fn set_music_volume(&mut self, volume: f32) {
        self.music_volume = volume.clamp(0.0, 1.0);
        info!("Music volume set to: {}", self.music_volume);
    }
}

// Predefined sound effect types for easy reference
pub enum SoundEffect {
    PlayerShoot,
    PlayerHit,
    PlayerDeath,
    EnemyHit,
    EnemyDeath,
    PickupItem,
    LevelUp,
    WaveComplete,
    ButtonClick,
    MenuNavigate,
}

impl SoundEffect {
    pub fn as_str(&self) -> &'static str {
        match self {
            SoundEffect::PlayerShoot => "player_shoot",
            SoundEffect::PlayerHit => "player_hit",
            SoundEffect::PlayerDeath => "player_death",
            SoundEffect::EnemyHit => "enemy_hit",
            SoundEffect::EnemyDeath => "enemy_death",
            SoundEffect::PickupItem => "pickup_item",
            SoundEffect::LevelUp => "level_up",
            SoundEffect::WaveComplete => "wave_complete",
            SoundEffect::ButtonClick => "button_click",
            SoundEffect::MenuNavigate => "menu_navigate",
        }
    }
}

pub enum MusicTrack {
    MainMenu,
    GameplayAmbient,
    GameplayIntense,
    GameOver,
}

impl MusicTrack {
    pub fn as_str(&self) -> &'static str {
        match self {
            MusicTrack::MainMenu => "main_menu",
            MusicTrack::GameplayAmbient => "gameplay_ambient",
            MusicTrack::GameplayIntense => "gameplay_intense",
            MusicTrack::GameOver => "game_over",
        }
    }
}

fn setup_audio(mut audio_manager: ResMut<AudioManager>) {
    *audio_manager = AudioManager::new();
    info!("Audio system initialized");
}

fn update_audio_system(
    // For now, this system is mostly placeholder
    // In the future, it could handle 3D audio positioning, dynamic music switching, etc.
    audio_manager: Res<AudioManager>,
) {
    // TODO: Implement dynamic audio updates
    // - Update 3D audio positions based on player/camera position
    // - Handle music transitions based on game state
    // - Manage audio memory usage
    
    // Placeholder for now
    if audio_manager.is_changed() {
        // React to audio manager changes
    }
}
