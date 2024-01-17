use std::time::Duration;

use bevy::prelude::*;
use bevy_animation::{
    animator::{Animator, Repeat},
    plugin::AnimationPlugin,
    ui::{
        component::{na, root},
        game_class::c_root,
    },
};
use interpolation::EaseFunction;
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, AnimationPlugin))
        .add_systems(Startup, setup_ui)
        .run();
}

fn setup_ui(mut commands: Commands, world: &World) {
    commands.spawn(Camera2dBundle::default());

    root(c_root, world, &mut commands, |p| {
        for _ in 0..3000 {
            na(c_node_style, node_animate, p, |_| {});
        }
    });
}

fn c_node_style(node: &mut NodeBundle) {
    node.style.width = Val::Px(100.);
    node.style.height = Val::Px(100.);
    node.background_color = Color::RED.into();
    node.style.position_type = PositionType::Absolute;
    node.style.left = Val::Px(0.);
    node.style.top = Val::Px(0.);
}

fn node_animate(animator: &mut Animator) {
    animator
        .set_loop(true)
        .set_exec(true)
        .add_change()
        .set_mirror(true)
        .set_repeat(Repeat::Finite(5))
        .set_delay(Duration::from_secs(1))
        .set_ease(EaseFunction::CubicIn.into())
        //.set_wh(Val::Px(90.), Val::Px(90.))
        .set_lt(Val::Px(120.), Val::Px(200.));
    /* animator
        .add_change()
        .set_delay(Duration::from_secs(1))
        .set_ease(EaseFunction::CubicIn.into())
        .set_lt(Val::Px(120.), Val::Px(200.));
    animator.add_change().set_delay(Duration::from_secs(1)); */
}
