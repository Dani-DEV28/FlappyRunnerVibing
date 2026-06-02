use bevy::prelude::*;
use crate::player::{Player, FacingDirection};

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (attack_input, advance_attack_timer, despawn_hitboxes));
    }
}

#[derive(Component)]
pub struct AttackState {
    pub combo_count: u8,       // 0 = idle, 1-3 = combo hit number
    pub timer: Timer,          // duration of current attack
    pub combo_window: Timer,   // window to chain next hit
    pub can_chain: bool,
}

impl Default for AttackState {
    fn default() -> Self {
        Self {
            combo_count: 0,
            timer: Timer::from_seconds(0.25, TimerMode::Once),
            combo_window: Timer::from_seconds(0.4, TimerMode::Once),
            can_chain: false,
        }
    }
}

#[derive(Component)]
pub struct Hitbox {
    pub damage: f32,
    pub knockback: Vec2,
    pub lifetime: Timer,
    pub owner: Entity,
}

const MAX_COMBO: u8 = 3;

fn attack_input(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(Entity, &Transform, &FacingDirection, &mut AttackState), With<Player>>,
) {
    let (entity, transform, facing, mut state) = query.single_mut();

    if !keyboard.just_pressed(KeyCode::KeyJ) && !keyboard.just_pressed(KeyCode::Enter) {
        return;
    }

    // Can attack if idle or within combo window
    let can_attack = state.combo_count == 0
        || (state.can_chain && state.combo_count < MAX_COMBO);

    if !can_attack {
        return;
    }

    state.combo_count = if state.combo_count == 0 { 1 } else { state.combo_count + 1 };
    state.timer = Timer::from_seconds(0.2, TimerMode::Once);
    state.combo_window = Timer::from_seconds(0.4, TimerMode::Once);
    state.can_chain = false;

    // Spawn hitbox in front of player
    let offset_x = facing.0 * 40.0;
    let dmg = 10.0 + (state.combo_count as f32 - 1.0) * 5.0;
    let kb = Vec2::new(facing.0 * (200.0 + state.combo_count as f32 * 50.0), 100.0);

    commands.spawn((
        Sprite {
            color: Color::srgba(1.0, 0.3, 0.3, 0.5),
            custom_size: Some(Vec2::new(35.0, 40.0)),
            ..default()
        },
        Transform::from_translation(Vec3::new(
            transform.translation.x + offset_x,
            transform.translation.y,
            2.0,
        )),
        Hitbox {
            damage: dmg,
            knockback: kb,
            lifetime: Timer::from_seconds(0.12, TimerMode::Once),
            owner: entity,
        },
    ));
}

fn advance_attack_timer(
    time: Res<Time>,
    mut query: Query<&mut AttackState, With<Player>>,
) {
    let mut state = query.single_mut();

    if state.combo_count == 0 {
        return;
    }

    state.timer.tick(time.delta());
    if state.timer.finished() && !state.can_chain {
        state.can_chain = true;
    }

    state.combo_window.tick(time.delta());
    if state.combo_window.finished() {
        // Reset combo
        state.combo_count = 0;
        state.can_chain = false;
    }
}

fn despawn_hitboxes(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Hitbox)>,
) {
    for (entity, mut hitbox) in &mut query {
        hitbox.lifetime.tick(time.delta());
        if hitbox.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}
