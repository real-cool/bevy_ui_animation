use bevy::prelude::*;

use crate::animator::Animator;
pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_style, update_text, update_transform, update_background, update_progress));
    }
}

fn update_progress(mut query: Query<&mut Animator>, time: Res<Time>) {
    for mut animator in query.iter_mut() {
        animator.tick_progress(time.delta());
    }
}

fn update_style(
    mut query: Query<(&mut Animator, &mut Style)>,
) {
    for (mut animator, mut style) in query.iter_mut() {
        animator.tick_style(&mut style);
    }
}

fn update_text(
    mut query: Query<(&mut Animator, &mut Text)>,
) {
    for (mut animator, mut text) in query.iter_mut() {
        animator.tick_text(&mut text);
    }
}

fn update_transform(
    mut query: Query<(&mut Animator, &mut Transform)>,
) {
    for (mut animator, mut transform) in query.iter_mut() {
        animator.tick_transform(&mut transform);
    }
}

fn update_background(
    mut query: Query<(&mut Animator, &mut BackgroundColor)>,
) {
    for (mut animator, mut background) in query.iter_mut() {
        animator.tick_background(&mut background);
    }
}
