mod player;
mod combat;
mod enemy;
mod damage;
mod camera;

use bevy::prelude::*;
use enemy::Enemy;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Flappy Puncher".to_string(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .add_plugins((
            player::PlayerPlugin,
            combat::CombatPlugin,
            enemy::EnemyPlugin,
            damage::DamagePlugin,
            camera::CameraPlugin,
        ))
        .add_systems(Startup, (setup, spawn_health_bar))
        .add_systems(Update, (check_win, update_health_bar, check_player_death).run_if(in_state(GameState::Playing)))
        .add_systems(OnEnter(GameState::Won), show_win_screen)
        .add_systems(OnExit(GameState::Won), despawn_win_screen)
        .add_systems(OnEnter(GameState::Dead), show_death_screen)
        .add_systems(OnExit(GameState::Dead), despawn_win_screen)
        .add_systems(Update, reset_game)
        .run();
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Playing,
    Won,
    Dead,
}

#[derive(Component)]
struct WinScreen;

#[derive(Component)]
struct HealthBarFill;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        Sprite {
            color: Color::srgb(0.3, 0.3, 0.3),
            custom_size: Some(Vec2::new(2000.0, 50.0)),
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, -200.0, 0.0)),
    ));
}

fn check_win(
    enemy_query: Query<&Enemy>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if enemy_query.iter().count() == 0 {
        next_state.set(GameState::Won);
    }
}

fn show_win_screen(mut commands: Commands) {
    commands.spawn((
        WinScreen,
        Text::new("YOU WIN!\n\nPress R to restart"),
        TextFont {
            font_size: 60.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.9, 0.0)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(35.0),
            left: Val::Percent(30.0),
            ..default()
        },
    ));
}

fn despawn_win_screen(mut commands: Commands, query: Query<Entity, With<WinScreen>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn reset_game(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    all_enemies: Query<Entity, With<Enemy>>,
    all_players: Query<Entity, With<player::Player>>,
    all_hitboxes: Query<Entity, With<combat::Hitbox>>,
) {
    if !keyboard.just_pressed(KeyCode::KeyR) {
        return;
    }

    for e in all_enemies.iter().chain(all_players.iter()).chain(all_hitboxes.iter()) {
        commands.entity(e).despawn();
    }

    player::spawn_player_pub(&mut commands);
    enemy::spawn_enemies_pub(&mut commands);

    if *state.get() != GameState::Playing {
        next_state.set(GameState::Playing);
    }
}

fn spawn_health_bar(mut commands: Commands) {
    // Background
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            left: Val::Px(20.0),
            width: Val::Px(200.0),
            height: Val::Px(20.0),
            ..default()
        },
        BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
    )).with_children(|parent| {
        // Fill
        parent.spawn((
            HealthBarFill,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.9, 0.1)),
        ));
    });
}

fn update_health_bar(
    player_query: Query<&damage::Health, With<player::Player>>,
    mut bar_query: Query<&mut Node, With<HealthBarFill>>,
) {
    let health = player_query.single();
    let mut node = bar_query.single_mut();
    let pct = (health.current / health.max * 100.0).clamp(0.0, 100.0);
    node.width = Val::Percent(pct);
}

fn check_player_death(
    player_query: Query<&damage::Health, With<player::Player>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let health = player_query.single();
    if health.current <= 0.0 {
        next_state.set(GameState::Dead);
    }
}

fn show_death_screen(mut commands: Commands) {
    commands.spawn((
        WinScreen,
        Text::new("YOU DIED!\n\nPress R to restart"),
        TextFont {
            font_size: 60.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.2, 0.2)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(35.0),
            left: Val::Percent(30.0),
            ..default()
        },
    ));
}
