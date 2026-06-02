use bevy::prelude::*;
use rand::Rng;

// --- Constants ---
const GRAVITY: f32 = -800.0;
const FLAP_IMPULSE: f32 = 350.0;
const SCROLL_SPEED: f32 = 200.0;
const GROUND_Y: f32 = -250.0;
const CEILING_Y: f32 = 300.0;
const PLAYER_SIZE: Vec2 = Vec2::new(30.0, 30.0);
const OBSTACLE_WIDTH: f32 = 40.0;
const OBSTACLE_GAP: f32 = 160.0;
const SPAWN_INTERVAL: f32 = 2.0;
const DESPAWN_X: f32 = -400.0;

// --- Components ---
#[derive(Component)]
struct Player {
    velocity_y: f32,
}

#[derive(Component)]
struct Obstacle;

#[derive(Resource)]
struct SpawnTimer(Timer);

#[derive(Resource)]
struct GameActive(bool);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Flap Runner".to_string(),
                resolution: (800.0, 600.0).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(SpawnTimer(Timer::from_seconds(SPAWN_INTERVAL, TimerMode::Repeating)))
        .insert_resource(GameActive(true))
        .add_systems(Startup, setup)
        .add_systems(Update, (
            player_input,
            player_movement,
            camera_follow,
            spawn_obstacles,
            move_obstacles,
            check_collisions,
        ).chain().run_if(|active: Res<GameActive>| active.0))
        .add_systems(Update, restart_game)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // Player
    commands.spawn((
        Sprite {
            color: Color::srgb(0.2, 0.8, 0.4),
            custom_size: Some(PLAYER_SIZE),
            ..default()
        },
        Transform::from_xyz(-200.0, 0.0, 0.0),
        Player { velocity_y: 0.0 },
    ));

    // Ground
    commands.spawn((
        Sprite {
            color: Color::srgb(0.4, 0.3, 0.2),
            custom_size: Some(Vec2::new(2000.0, 20.0)),
            ..default()
        },
        Transform::from_xyz(0.0, GROUND_Y - 10.0, 0.0),
    ));

    // Ceiling
    commands.spawn((
        Sprite {
            color: Color::srgb(0.4, 0.3, 0.2),
            custom_size: Some(Vec2::new(2000.0, 20.0)),
            ..default()
        },
        Transform::from_xyz(0.0, CEILING_Y + 10.0, 0.0),
    ));
}

fn player_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Player>,
) {
    for mut player in &mut query {
        if keyboard.just_pressed(KeyCode::Space) {
            player.velocity_y = FLAP_IMPULSE;
        }
    }
}

fn player_movement(
    time: Res<Time>,
    mut query: Query<(&mut Player, &mut Transform)>,
) {
    for (mut player, mut transform) in &mut query {
        player.velocity_y += GRAVITY * time.delta_secs();
        transform.translation.y += player.velocity_y * time.delta_secs();
        transform.translation.y = transform.translation.y.clamp(GROUND_Y, CEILING_Y);

        if transform.translation.y <= GROUND_Y || transform.translation.y >= CEILING_Y {
            player.velocity_y = 0.0;
        }
    }
}

fn camera_follow(
    player_q: Query<&Transform, With<Player>>,
    mut camera_q: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    if let (Ok(player_tf), Ok(mut cam_tf)) = (player_q.get_single(), camera_q.get_single_mut()) {
        cam_tf.translation.x = player_tf.translation.x + 100.0;
    }
}

fn spawn_obstacles(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
    player_q: Query<&Transform, With<Player>>,
) {
    timer.0.tick(time.delta());
    if !timer.0.just_finished() {
        return;
    }
    let Ok(player_tf) = player_q.get_single() else { return };
    let spawn_x = player_tf.translation.x + 500.0;

    let mut rng = rand::thread_rng();
    let gap_center = rng.gen_range((GROUND_Y + 100.0)..(CEILING_Y - 100.0));

    // Top obstacle
    let top_height = CEILING_Y - (gap_center + OBSTACLE_GAP / 2.0);
    commands.spawn((
        Sprite {
            color: Color::srgb(0.8, 0.2, 0.2),
            custom_size: Some(Vec2::new(OBSTACLE_WIDTH, top_height)),
            ..default()
        },
        Transform::from_xyz(spawn_x, gap_center + OBSTACLE_GAP / 2.0 + top_height / 2.0, 0.0),
        Obstacle,
    ));

    // Bottom obstacle
    let bot_height = (gap_center - OBSTACLE_GAP / 2.0) - GROUND_Y;
    commands.spawn((
        Sprite {
            color: Color::srgb(0.8, 0.2, 0.2),
            custom_size: Some(Vec2::new(OBSTACLE_WIDTH, bot_height)),
            ..default()
        },
        Transform::from_xyz(spawn_x, GROUND_Y + bot_height / 2.0, 0.0),
        Obstacle,
    ));
}

fn move_obstacles(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform), With<Obstacle>>,
    player_q: Query<&Transform, (With<Player>, Without<Obstacle>)>,
) {
    let Ok(player_tf) = player_q.get_single() else { return };
    for (entity, mut tf) in &mut query {
        tf.translation.x -= SCROLL_SPEED * time.delta_secs();
        if tf.translation.x < player_tf.translation.x + DESPAWN_X {
            commands.entity(entity).despawn();
        }
    }
}

fn check_collisions(
    player_q: Query<&Transform, With<Player>>,
    obstacle_q: Query<(&Transform, &Sprite), With<Obstacle>>,
    mut active: ResMut<GameActive>,
) {
    let Ok(player_tf) = player_q.get_single() else { return };
    let player_pos = player_tf.translation.truncate();

    for (obs_tf, sprite) in &obstacle_q {
        let obs_pos = obs_tf.translation.truncate();
        let obs_size = sprite.custom_size.unwrap_or(Vec2::new(OBSTACLE_WIDTH, 100.0));

        // AABB collision
        if (player_pos.x - obs_pos.x).abs() < (PLAYER_SIZE.x + obs_size.x) / 2.0
            && (player_pos.y - obs_pos.y).abs() < (PLAYER_SIZE.y + obs_size.y) / 2.0
        {
            active.0 = false;
            info!("Collision! Press R to restart.");
            return;
        }
    }
}

fn restart_game(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut active: ResMut<GameActive>,
    mut player_q: Query<(&mut Player, &mut Transform)>,
    obstacles: Query<Entity, With<Obstacle>>,
    mut commands: Commands,
) {
    if !keyboard.just_pressed(KeyCode::KeyR) {
        return;
    }
    active.0 = true;
    for (mut player, mut tf) in &mut player_q {
        tf.translation = Vec3::new(-200.0, 0.0, 0.0);
        player.velocity_y = 0.0;
    }
    for entity in &obstacles {
        commands.entity(entity).despawn();
    }
}
