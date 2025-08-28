//! Bevy Arena Survivor - Main Entry Point
use bevy::prelude::*;

mod player;
mod enemy;
mod wave;
mod loot;
mod upgrade;
mod ui;
mod fx;
mod audio;
mod config;

use player::PlayerPlugin;
use enemy::EnemyPlugin;
use wave::WavePlugin;
use loot::LootPlugin;
use upgrade::UpgradePlugin;
use ui::UIPlugin;
use fx::FXPlugin;
use audio::AudioPlugin;
use config::ConfigPlugin;

// Game states
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    MainMenu,
    Playing,
    Paused,
    GameOver,
}

// Core components
#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct GameEntity;

// Resources
#[derive(Resource)]
pub struct GameData {
    pub score: u32,
    pub current_wave: u32,
    pub game_time: f32,
}

impl Default for GameData {
    fn default() -> Self {
        Self {
            score: 0,
            current_wave: 1,
            game_time: 0.0,
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(
            WindowPlugin {
                primary_window: Some(Window {
                    title: "Void Survivor - Arena Shooter".to_string(),
                    resolution: (1280.0, 720.0).into(),
                    resizable: true,
                    ..default()
                }),
                ..default()
            }
        ).set(ImagePlugin::default_nearest())) // Pixel-perfect rendering
        .add_state::<GameState>()
        .insert_resource(GameData::default())
        .add_plugins((
            ConfigPlugin,
            PlayerPlugin,
            EnemyPlugin,
            WavePlugin,
            LootPlugin,
            UpgradePlugin,
            UIPlugin,
            FXPlugin,
            AudioPlugin,
        ))
        .add_systems(Startup, setup_game)
        .add_systems(Update, (
            handle_game_state_transitions,
            update_camera_system,
        ))
        .run();
}

fn setup_game(mut commands: Commands) {
    // Setup camera for top-down view
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                order: 0,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 100.0),
            ..default()
        },
        MainCamera,
    ));
    
    info!("Game setup complete - Void Survivor initialized");
}

fn handle_game_state_transitions(
    keyboard_input: Res<Input<KeyCode>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    match current_state.get() {
        GameState::MainMenu => {
            if keyboard_input.just_pressed(KeyCode::Space) {
                next_state.set(GameState::Playing);
                info!("Starting game");
            }
        }
        GameState::Playing => {
            if keyboard_input.just_pressed(KeyCode::Escape) {
                next_state.set(GameState::Paused);
                info!("Game paused");
            }
        }
        GameState::Paused => {
            if keyboard_input.just_pressed(KeyCode::Escape) {
                next_state.set(GameState::Playing);
                info!("Game resumed");
            }
        }
        GameState::GameOver => {
            if keyboard_input.just_pressed(KeyCode::Space) {
                next_state.set(GameState::MainMenu);
                info!("Returning to main menu");
            }
        }
    }
}

fn update_camera_system(
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<player::Player>)>,
    player_query: Query<&Transform, (With<player::Player>, Without<MainCamera>)>,
) {
    if let Ok(mut camera_transform) = camera_query.get_single_mut() {
        if let Ok(player_transform) = player_query.get_single() {
            // Smoothly follow player
            let target = player_transform.translation;
            camera_transform.translation = camera_transform.translation.lerp(target, 0.05);
        }
    }
}
