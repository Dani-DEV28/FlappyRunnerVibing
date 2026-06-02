use bevy::prelude::*;
use crate::combat::AttackState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (player_movement, player_jump, apply_gravity));
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct FacingDirection(pub f32); // 1.0 = right, -1.0 = left

#[derive(Component)]
pub struct Grounded(pub bool);

const MOVE_SPEED: f32 = 300.0;
const JUMP_FORCE: f32 = 500.0;
const GRAVITY: f32 = -1200.0;
const GROUND_Y: f32 = -175.0;

fn spawn_player(mut commands: Commands) {
    spawn_player_pub(&mut commands);
}

pub fn spawn_player_pub(commands: &mut Commands) {
    commands.spawn((
        Player,
        Sprite {
            color: Color::srgb(0.2, 0.8, 0.2),
            custom_size: Some(Vec2::new(40.0, 60.0)),
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, GROUND_Y, 1.0)),
        Velocity(Vec2::ZERO),
        FacingDirection(1.0),
        Grounded(true),
        AttackState::default(),
        crate::damage::Health { current: 100.0, max: 100.0 },
    ));
}

fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Velocity, &mut FacingDirection, &mut Sprite), With<Player>>,
) {
    let (mut transform, mut vel, mut facing, mut sprite) = query.single_mut();

    let mut dir = 0.0;
    if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
        dir -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
        dir += 1.0;
    }

    vel.0.x = dir * MOVE_SPEED;
    transform.translation.x += vel.0.x * time.delta_secs();

    if dir != 0.0 {
        facing.0 = dir;
        sprite.flip_x = dir < 0.0;
    }
}

fn player_jump(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Grounded), With<Player>>,
) {
    let (mut vel, mut grounded) = query.single_mut();

    if keyboard.just_pressed(KeyCode::Space) && grounded.0 {
        vel.0.y = JUMP_FORCE;
        grounded.0 = false;
    }
}

fn apply_gravity(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Velocity, &mut Grounded), With<Player>>,
) {
    let (mut transform, mut vel, mut grounded) = query.single_mut();

    vel.0.y += GRAVITY * time.delta_secs();
    transform.translation.y += vel.0.y * time.delta_secs();

    if transform.translation.y <= GROUND_Y {
        transform.translation.y = GROUND_Y;
        vel.0.y = 0.0;
        grounded.0 = true;
    }
}
