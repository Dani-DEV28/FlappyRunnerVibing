use bevy::prelude::*;
use crate::player::Player;
use crate::combat::Hitbox;
use crate::damage::Health;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_enemies)
            .add_systems(Update, (enemy_ai, enemy_attack_cooldown));
    }
}

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct EnemyAi {
    pub aggro_range: f32,
    pub attack_range: f32,
    pub speed: f32,
    pub attack_cooldown: Timer,
}

const GROUND_Y: f32 = -175.0;

fn spawn_enemies(mut commands: Commands) {
    spawn_enemies_pub(&mut commands);
}

pub fn spawn_enemies_pub(commands: &mut Commands) {
    let positions = [200.0, 350.0, -250.0, -400.0, 500.0];
    for x in positions {
        commands.spawn((
            Enemy,
            Sprite {
                color: Color::srgb(0.8, 0.2, 0.2),
                custom_size: Some(Vec2::new(36.0, 54.0)),
                ..default()
            },
            Transform::from_translation(Vec3::new(x, GROUND_Y, 1.0)),
            Health { current: 40.0, max: 40.0 },
            EnemyAi {
                aggro_range: 250.0,
                attack_range: 50.0,
                speed: 120.0,
                attack_cooldown: Timer::from_seconds(1.5, TimerMode::Once),
            },
        ));
    }
}

fn enemy_ai(
    mut commands: Commands,
    time: Res<Time>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    mut enemy_query: Query<(Entity, &mut Transform, &mut EnemyAi, &mut Sprite), (With<Enemy>, Without<Player>)>,
) {
    let (_player_entity, player_tf) = player_query.single();

    for (enemy_entity, mut tf, mut ai, mut sprite) in &mut enemy_query {
        let diff = player_tf.translation.x - tf.translation.x;
        let dist = diff.abs();

        // Face the player
        sprite.flip_x = diff < 0.0;

        if dist < ai.aggro_range {
            // Move toward player
            if dist > ai.attack_range {
                let dir = diff.signum();
                tf.translation.x += dir * ai.speed * time.delta_secs();
            }

            // Attack when in range and cooldown ready
            ai.attack_cooldown.tick(time.delta());
            if dist < ai.attack_range && ai.attack_cooldown.finished() {
                ai.attack_cooldown.reset();
                let facing = diff.signum();
                commands.spawn((
                    Sprite {
                        color: Color::srgba(1.0, 0.6, 0.0, 0.5),
                        custom_size: Some(Vec2::new(30.0, 35.0)),
                        ..default()
                    },
                    Transform::from_translation(Vec3::new(
                        tf.translation.x + facing * 35.0,
                        tf.translation.y,
                        2.0,
                    )),
                    Hitbox {
                        damage: 8.0,
                        knockback: Vec2::new(facing * 150.0, 80.0),
                        lifetime: Timer::from_seconds(0.15, TimerMode::Once),
                        owner: enemy_entity,
                    },
                ));
            }
        }
    }
}

fn enemy_attack_cooldown(
    time: Res<Time>,
    mut query: Query<&mut EnemyAi, With<Enemy>>,
) {
    // Cooldowns are ticked in enemy_ai when in aggro range; this handles out-of-range reset
    for mut ai in &mut query {
        if !ai.attack_cooldown.finished() {
            ai.attack_cooldown.tick(time.delta());
        }
    }
}
