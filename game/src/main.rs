//! Void Survivor - Main Entry Point
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod player;
mod enemy;
mod wave;
mod loot;
mod upgrade;
mod ui;
mod fx;
mod audio;
mod config;
mod asteroid;

use player::PlayerPlugin;
use enemy::EnemyPlugin;
use wave::WavePlugin;
use loot::LootPlugin;
use upgrade::UpgradePlugin;
use ui::UIPlugin;
use fx::FXPlugin;
use audio::AudioPlugin;
use config::ConfigPlugin;
use asteroid::AsteroidPlugin;

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
            AsteroidPlugin,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(), // Optional: for debugging physics shapes
        ))
        .insert_resource(RapierConfiguration {
            gravity: Vec3::ZERO, // Disable gravity for top-down space game
            ..default()
        })
        .add_systems(Startup, setup_game)
        .add_systems(Update, (
            handle_game_state_transitions,
            update_camera_system,
        ))
        .run();
}

fn setup_game(mut commands: Commands) {
    // Setup 3D camera for true top-down view
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 20.0, 0.0)
                .looking_at(Vec3::ZERO, Vec3::NEG_Z), // Use negative Z as up vector for proper top-down orientation
            camera: Camera {
                order: 0,
                ..default()
            },
            ..default()
        },
        MainCamera,
    ));
    
    // Add basic lighting for 3D models
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.5, -0.5, 0.0)),
        ..default()
    });
    
    // Add ambient lighting
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.3,
    });
    
    info!("Game setup complete - Void Survivor initialized with 3D rendering");
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
            // Keep camera directly above the player for true top-down view
            let target = Vec3::new(
                player_transform.translation.x,
                20.0, // Keep camera height constant
                player_transform.translation.z,
            );
            // Use faster lerp for more responsive camera (0.15 instead of 0.05)
            camera_transform.translation = camera_transform.translation.lerp(target, 0.15);
            
            // Always look straight down at the player with correct orientation
            let look_target = Vec3::new(
                player_transform.translation.x,
                0.0, // Look at ground level
                player_transform.translation.z,
            );
            camera_transform.look_at(look_target, Vec3::NEG_Z); // Use NEG_Z as up vector for consistent orientation
        }
    }
}
