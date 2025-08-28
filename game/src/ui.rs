//! UI system - handles HUD, menus, and user interface
use bevy::prelude::*;
use crate::{GameState};
use crate::player::Player;
use crate::wave::WaveManager;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Playing), setup_game_ui)
            .add_systems(Update, (
                update_health_bar,
                update_wave_info,
                update_player_stats,
            ).run_if(in_state(GameState::Playing)))
            .add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
            .add_systems(Update, main_menu_system.run_if(in_state(GameState::MainMenu)))
            .add_systems(OnExit(GameState::MainMenu), cleanup_main_menu);
    }
}

// UI Components
#[derive(Component)]
pub struct GameUI;

#[derive(Component)]
pub struct HealthBar;

#[derive(Component)]
pub struct ShieldBar;

#[derive(Component)]
pub struct WaveText;

#[derive(Component)]
pub struct LevelText;

#[derive(Component)]
pub struct ExperienceBar;

#[derive(Component)]
pub struct MainMenuUI;

#[derive(Component)]
pub struct PlayButton;

// Game UI Setup
fn setup_game_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Root UI node
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                ..default()
            },
            GameUI,
        ))
        .with_children(|parent| {
            // Top bar (wave info, level)
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(60.0),
                        padding: UiRect::all(Val::Px(10.0)),
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::rgba(0.0, 0.0, 0.0, 0.5).into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Wave info
                    parent.spawn((
                        TextBundle::from_section(
                            "Wave: 1",
                            TextStyle {
                                font: asset_server.load("fonts/game_font.ttf"),
                                font_size: 24.0,
                                color: Color::WHITE,
                            },
                        ),
                        WaveText,
                    ));
                    
                    // Level info
                    parent.spawn((
                        TextBundle::from_section(
                            "Level: 1",
                            TextStyle {
                                font: asset_server.load("fonts/game_font.ttf"),
                                font_size: 24.0,
                                color: Color::YELLOW,
                            },
                        ),
                        LevelText,
                    ));
                });
            
            // Bottom bar (health, shields, experience)
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(80.0),
                        padding: UiRect::all(Val::Px(10.0)),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::rgba(0.0, 0.0, 0.0, 0.5).into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Health bar container
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(60.0),
                                height: Val::Px(20.0),
                                margin: UiRect::bottom(Val::Px(5.0)),
                                border: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            background_color: Color::DARK_GRAY.into(),
                            border_color: Color::WHITE.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                NodeBundle {
                                    style: Style {
                                        width: Val::Percent(100.0),
                                        height: Val::Percent(100.0),
                                        ..default()
                                    },
                                    background_color: Color::RED.into(),
                                    ..default()
                                },
                                HealthBar,
                            ));
                        });
                    
                    // Shield bar container
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(60.0),
                                height: Val::Px(15.0),
                                margin: UiRect::bottom(Val::Px(5.0)),
                                border: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            background_color: Color::DARK_GRAY.into(),
                            border_color: Color::CYAN.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                NodeBundle {
                                    style: Style {
                                        width: Val::Percent(100.0),
                                        height: Val::Percent(100.0),
                                        ..default()
                                    },
                                    background_color: Color::CYAN.into(),
                                    ..default()
                                },
                                ShieldBar,
                            ));
                        });
                    
                    // Experience bar container
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(60.0),
                                height: Val::Px(10.0),
                                border: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            background_color: Color::DARK_GRAY.into(),
                            border_color: Color::YELLOW.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                NodeBundle {
                                    style: Style {
                                        width: Val::Percent(0.0),
                                        height: Val::Percent(100.0),
                                        ..default()
                                    },
                                    background_color: Color::YELLOW.into(),
                                    ..default()
                                },
                                ExperienceBar,
                            ));
                        });
                });
        });
}

// Main Menu Setup
fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: Color::rgba(0.0, 0.0, 0.2, 0.8).into(),
                ..default()
            },
            MainMenuUI,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn(TextBundle::from_section(
                "Bevy Arena Survivor",
                TextStyle {
                    font: asset_server.load("fonts/game_font.ttf"),
                    font_size: 48.0,
                    color: Color::WHITE,
                },
            ).with_style(Style {
                margin: UiRect::bottom(Val::Px(50.0)),
                ..default()
            }));
            
            // Play button
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(200.0),
                            height: Val::Px(60.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: UiRect::bottom(Val::Px(20.0)),
                            ..default()
                        },
                        background_color: Color::rgb(0.2, 0.4, 0.8).into(),
                        ..default()
                    },
                    PlayButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play",
                        TextStyle {
                            font: asset_server.load("fonts/game_font.ttf"),
                            font_size: 24.0,
                            color: Color::WHITE,
                        },
                    ));
                });
        });
}

// Update Systems
fn update_health_bar(
    player_query: Query<&Player>,
    mut health_bar_query: Query<&mut Style, With<HealthBar>>,
    mut shield_bar_query: Query<&mut Style, (With<ShieldBar>, Without<HealthBar>)>,
) {
    if let Ok(player) = player_query.get_single() {
        // Update health bar
        if let Ok(mut style) = health_bar_query.get_single_mut() {
            let health_percent = (player.health / player.max_health).clamp(0.0, 1.0);
            style.width = Val::Percent(health_percent * 100.0);
        }
        
        // Update shield bar
        if let Ok(mut style) = shield_bar_query.get_single_mut() {
            let shield_percent = (player.shields / player.max_shields).clamp(0.0, 1.0);
            style.width = Val::Percent(shield_percent * 100.0);
        }
    }
}

fn update_wave_info(
    wave_manager: Res<WaveManager>,
    mut wave_text_query: Query<&mut Text, With<WaveText>>,
) {
    if let Ok(mut text) = wave_text_query.get_single_mut() {
        text.sections[0].value = format!("Wave: {}", wave_manager.current_wave);
    }
}

fn update_player_stats(
    player_query: Query<&Player>,
    mut level_text_query: Query<&mut Text, (With<LevelText>, Without<ExperienceBar>)>,
    mut exp_bar_query: Query<&mut Style, With<ExperienceBar>>,
) {
    if let Ok(player) = player_query.get_single() {
        // Update level text
        if let Ok(mut text) = level_text_query.get_single_mut() {
            text.sections[0].value = format!("Level: {}", player.level);
        }
        
        // Update experience bar
        if let Ok(mut style) = exp_bar_query.get_single_mut() {
            let exp_needed = player.level as f32 * 100.0;
            let exp_percent = (player.experience / exp_needed).clamp(0.0, 1.0);
            style.width = Val::Percent(exp_percent * 100.0);
        }
    }
}

// Menu Systems
fn main_menu_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlayButton>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                next_state.set(GameState::Playing);
            }
            Interaction::Hovered => {
                *color = Color::rgb(0.3, 0.5, 0.9).into();
            }
            Interaction::None => {
                *color = Color::rgb(0.2, 0.4, 0.8).into();
            }
        }
    }
}

// Cleanup Systems
fn cleanup_main_menu(
    mut commands: Commands,
    menu_query: Query<Entity, With<MainMenuUI>>,
) {
    for entity in menu_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

// Upgrade selection UI (for later implementation)
#[derive(Component)]
pub struct UpgradeSelectionUI;

#[derive(Component)]
pub struct UpgradeButton {
    pub upgrade_type: UpgradeType,
}

#[derive(Clone, Debug)]
pub enum UpgradeType {
    HealthBoost,
    ShieldBoost,
    DamageBoost,
    FireRateBoost,
    SpeedBoost,
    NewWeapon,
}

// Death screen UI
pub fn setup_death_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: Color::rgba(0.8, 0.0, 0.0, 0.7).into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "GAME OVER",
                TextStyle {
                    font: asset_server.load("fonts/game_font.ttf"),
                    font_size: 48.0,
                    color: Color::WHITE,
                },
            ));
        });
}
