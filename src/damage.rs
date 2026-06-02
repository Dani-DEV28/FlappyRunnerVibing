use bevy::prelude::*;
use crate::combat::Hitbox;
use crate::player::Player;
use crate::enemy::Enemy;

pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (hit_detection, apply_knockback, check_death));
    }
}

#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

#[derive(Component)]
pub struct Knockback(pub Vec2, pub Timer);

const HITBOX_HALF: Vec2 = Vec2::new(17.5, 20.0);
const PLAYER_HALF: Vec2 = Vec2::new(20.0, 30.0);
const ENEMY_HALF: Vec2 = Vec2::new(18.0, 27.0);

fn aabb_overlap(a_pos: Vec3, a_half: Vec2, b_pos: Vec3, b_half: Vec2) -> bool {
    (a_pos.x - b_pos.x).abs() < a_half.x + b_half.x
        && (a_pos.y - b_pos.y).abs() < a_half.y + b_half.y
}

fn hit_detection(
    mut commands: Commands,
    hitbox_query: Query<(Entity, &Transform, &Hitbox)>,
    mut player_query: Query<(Entity, &Transform, &mut Health), (With<Player>, Without<Enemy>)>,
    mut enemy_query: Query<(Entity, &Transform, &mut Health), (With<Enemy>, Without<Player>)>,
) {
    let (player_entity, player_tf, mut player_health) = player_query.single_mut();

    for (hb_entity, hb_tf, hitbox) in &hitbox_query {
        // Player hitbox hitting enemies
        if hitbox.owner == player_entity {
            for (enemy_entity, enemy_tf, mut health) in &mut enemy_query {
                if aabb_overlap(hb_tf.translation, HITBOX_HALF, enemy_tf.translation, ENEMY_HALF) {
                    health.current -= hitbox.damage;
                    commands.entity(enemy_entity).insert(
                        Knockback(hitbox.knockback, Timer::from_seconds(0.15, TimerMode::Once))
                    );
                    commands.entity(hb_entity).despawn();
                    break;
                }
            }
        } else {
            // Enemy hitbox hitting player
            if aabb_overlap(hb_tf.translation, HITBOX_HALF, player_tf.translation, PLAYER_HALF) {
                player_health.current -= hitbox.damage;
                commands.entity(player_entity).insert(
                    Knockback(hitbox.knockback, Timer::from_seconds(0.15, TimerMode::Once))
                );
                commands.entity(hb_entity).despawn();
            }
        }
    }
}

fn apply_knockback(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut Knockback)>,
) {
    for (entity, mut tf, mut kb) in &mut query {
        tf.translation.x += kb.0.x * time.delta_secs();
        tf.translation.y += kb.0.y * time.delta_secs();
        kb.1.tick(time.delta());
        if kb.1.finished() {
            commands.entity(entity).remove::<Knockback>();
        }
    }
}

fn check_death(
    mut commands: Commands,
    query: Query<(Entity, &Health), (With<Enemy>, Without<Player>)>,
) {
    for (entity, health) in &query {
        if health.current <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
