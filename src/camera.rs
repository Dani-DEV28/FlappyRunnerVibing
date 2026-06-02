use bevy::prelude::*;
use crate::player::Player;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, camera_follow);
    }
}

fn camera_follow(
    player_query: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
) {
    let player_tf = player_query.single();
    let mut cam_tf = camera_query.single_mut();

    // Smooth follow on X, fixed Y
    let target_x = player_tf.translation.x;
    cam_tf.translation.x += (target_x - cam_tf.translation.x) * 0.05;
}
