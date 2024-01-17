use crate::ani_utils::*;
use bevy::prelude::*;
use interpolation::*;
use std::{
    ops::{Add, Sub},
    time::Duration,
};

#[derive(Component, Default, Debug)]
pub struct Animator {
    exectable: bool,
    exec_index: usize,
    exec_loop: bool,
    exec_step: bool,
    progress: Duration,
    init_style: bool,
    init_text: bool,
    init_background: bool,
    init_transform: bool,
    start: Anim,
    animations: Vec<Animation>,
}

impl Animator {
    pub fn get_cur_total_duration(&self) -> Duration {
        match self.animations.get(self.exec_index) {
            Some(animation) => match animation.repeat {
                Repeat::Finite(count) => animation.per * count as u32,
                Repeat::Infinite => Duration::MAX,
                Repeat::Duration(duration) => duration,
            },
            None => Duration::ZERO,
        }
    }

    pub fn set_exec(&mut self, exectable: bool) -> &mut Self {
        self.exectable = exectable;
        self
    }

    pub fn set_loop(&mut self, loop_: bool) -> &mut Self {
        self.exec_loop = loop_;
        self
    }

    pub fn set_step(&mut self, step: bool) -> &mut Self {
        self.exec_step = step;
        self
    }

    pub fn add_change(&mut self) -> &mut Animation {
        self.animations.push(Animation::default());
        self.animations.last_mut().unwrap()
    }

    pub fn restart(&mut self, index: usize) {
        self.start_animation(index, self.animations.len(), true);
    }

    fn start_animation(&mut self, index: usize, len: usize, restart: bool) {
        if index < len {
            let pre_index = self.exec_index;
            self.exec_index = index;
            self.progress = Duration::ZERO;
            if pre_index > index {
                for i in index..pre_index {
                    let animation = self.animations.get(i).unwrap();
                    self.start = self.start - animation.change;
                }
            } else if pre_index < index {
                for i in pre_index..index {
                    let animation = self.animations.get(i).unwrap();
                    self.start = self.start + animation.change;
                }
            } else if !restart {
                self.progress = self.get_cur_total_duration();
            }
        }
    }

    pub fn tick_progress(&mut self, delta: Duration) {
        if !self.exectable {
            return;
        }
        self.progress += delta;
        let cur_total_duration = self.get_cur_total_duration();
        if self.progress > cur_total_duration {
            if self.exec_step {
                self.progress = cur_total_duration;
            } else {
                let len = self.animations.len();
                if self.exec_loop {
                    self.start_animation((self.exec_index + 1) % len, len, true);
                } else {
                    self.start_animation((self.exec_index + 1).min(len - 1), len, false);
                }
            }
        }
    }

    pub fn get_finished(&self, animation: &Animation) -> bool {
        self.progress >= self.get_total(animation)
    }

    fn get_total(&self, animation: &Animation) -> Duration {
        match animation.repeat {
            Repeat::Finite(count) => animation.per * count as u32,
            Repeat::Infinite => Duration::MAX,
            Repeat::Duration(duration) => duration,
        }
    }

    fn get_forward(&self, mirror: bool, animation: &Animation) -> bool {
        !mirror || (mirror && (self.progress.as_nanos() / animation.per.as_nanos()) % 2 == 0)
    }

    fn get_fract(&self, animation: &Animation) -> f32 {
        (self.progress.as_secs_f32() / animation.per.as_secs_f32()).fract()
    }

    fn get_factor(&self, mirror: bool, animation: &Animation) -> f32 {
        let fac = self.get_fract(animation);
        if self.get_forward(mirror, animation) {
            if self.get_finished(animation) {
                return 1.;
            }
            fac
        } else {
            1. - fac
        }
    }

    pub fn init_style(&mut self, style: &mut Style) {
        if !self.init_style {
            self.start.style_width = style.width;
            self.start.style_height = style.height;
            self.start.style_left = style.left;
            self.start.style_right = style.right;
            self.start.style_top = style.top;
            self.start.style_bottom = style.bottom;
            self.init_style = true;
        }
    }

    pub fn init_text(&mut self, text: &mut Text) {
        if !self.init_text {
            if let Some(section) = text.sections.get(0) {
                self.start.text_font_size = section.style.font_size;
                self.start.text_font_color = section.style.color;
            }
            self.init_text = true;
        }
    }

    pub fn init_background(&mut self, background: &mut BackgroundColor) {
        if !self.init_background {
            self.start.background_color = background.0;
            self.init_background = true;
        }
    }

    pub fn init_transform(&mut self, transform: &mut Transform) {
        if !self.init_transform {
            self.start.transform_translation = transform.translation;
            self.start.transform_rotation = transform.rotation;
            self.start.transform_scale = transform.scale;
            self.init_transform = true;
        }
    }

    pub fn tick_style(&mut self, style: &mut Style) {
        self.init_style(style);
        if let Some(animation) = self.animations.get(self.exec_index) {
            self.start.lerp_style(
                style,
                &animation.change,
                animation
                    .ease_method
                    .tick(self.get_factor(animation.mirror, animation)),
            );
        }
    }

    pub fn tick_text(&mut self, text: &mut Text) {
        self.init_text(text);
        if let Some(animation) = self.animations.get(self.exec_index) {
            self.start.lerp_text(
                text,
                &animation.change,
                animation
                    .ease_method
                    .tick(self.get_factor(animation.mirror, animation)),
            );
        }
    }

    pub fn tick_background(&mut self, background: &mut BackgroundColor) {
        self.init_background(background);
        if let Some(animation) = self.animations.get(self.exec_index) {
            self.start.lerp_background(
                background,
                &animation.change,
                animation
                    .ease_method
                    .tick(self.get_factor(animation.mirror, animation)),
            );
        }
    }

    pub fn tick_transform(&mut self, transform: &mut Transform) {
        self.init_transform(transform);
        if let Some(animation) = self.animations.get(self.exec_index) {
            self.start.lerp_transform(
                transform,
                &animation.change,
                animation
                    .ease_method
                    .tick(self.get_factor(animation.mirror, animation)),
            );
        }
    }
}

#[derive(Default, Debug)]
pub struct Animation {
    per: Duration,
    repeat: Repeat,
    mirror: bool,
    ease_method: EaseMethod,
    change: Anim,
}

#[derive(Clone, Copy, Debug)]
pub struct Anim {
    style_width: Val,
    style_height: Val,
    style_left: Val,
    style_right: Val,
    style_top: Val,
    style_bottom: Val,
    text_font_size: f32,
    text_font_color: Color,
    background_color: Color,
    transform_translation: Vec3,
    transform_rotation: Quat,
    transform_scale: Vec3,
}

impl Default for Anim {
    fn default() -> Self {
        Self {
            style_width: Val::Auto,
            style_height: Val::Auto,
            style_left: Val::Auto,
            style_right: Val::Auto,
            style_top: Val::Auto,
            style_bottom: Val::Auto,
            text_font_size: 0.,
            text_font_color: Color::NONE,
            background_color: Color::NONE,
            transform_translation: Vec3::ZERO,
            transform_rotation: Quat::from_vec4(Vec4::ZERO),
            transform_scale: Vec3::ZERO,
        }
    }
}

macro_rules! lerp_val {
    ($target:expr, $self:expr, $field:ident, $ratio:expr) => {
        match ($target.$field, $self.$field) {
            (Val::Px(s), Val::Px(v)) => Val::Px(s + v * $ratio),
            (Val::Percent(s), Val::Percent(v)) => Val::Percent(s + v * $ratio),
            (Val::Vw(s), Val::Vw(v)) => Val::Vw(s + v * $ratio),
            (Val::Vh(s), Val::Vh(v)) => Val::Vh(s + v * $ratio),
            (Val::VMin(s), Val::VMin(v)) => Val::VMin(s + v * $ratio),
            (Val::VMax(s), Val::VMax(v)) => Val::VMax(s + v * $ratio),
            _ => $target.$field,
        }
    };
}

impl Anim {
    fn lerp_style(&self, target: &mut Style, change: &Self, ratio: f32) {
        target.width = lerp_val!(self, change, style_width, ratio);
        target.height = lerp_val!(self, change, style_height, ratio);
        target.left = lerp_val!(self, change, style_left, ratio);
        target.right = lerp_val!(self, change, style_right, ratio);
        target.top = lerp_val!(self, change, style_top, ratio);
        target.bottom = lerp_val!(self, change, style_bottom, ratio);
    }

    fn lerp_text(&self, target: &mut Text, change: &Self, ratio: f32) {
        if let Some(section) = target.sections.get_mut(0) {
            section.style.font_size = self.text_font_size + change.text_font_size * ratio;
            section.style.color = self.text_font_color + change.text_font_color * ratio;
        }
    }

    fn lerp_background(&self, target: &mut BackgroundColor, change: &Self, ratio: f32) {
        target.0 = self.background_color + change.background_color * ratio;
    }

    fn lerp_transform(&self, target: &mut Transform, change: &Self, ratio: f32) {
        target.translation = self.transform_translation + change.transform_translation * ratio;
        target.rotation = self
            .transform_rotation
            .slerp(change.transform_rotation + self.transform_rotation, ratio);
        target.scale = self.transform_scale + change.transform_scale * ratio;
    }
}

impl Add for Anim {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            style_width: add_val(self.style_width, rhs.style_width),
            style_height: add_val(self.style_height, rhs.style_height),
            style_left: add_val(self.style_left, rhs.style_left),
            style_right: add_val(self.style_right, rhs.style_right),
            style_top: add_val(self.style_top, rhs.style_top),
            style_bottom: add_val(self.style_bottom, rhs.style_bottom),
            text_font_size: self.text_font_size + rhs.text_font_size,
            text_font_color: self.text_font_color + rhs.text_font_color,
            background_color: self.background_color + rhs.background_color,
            transform_translation: self.transform_translation + rhs.transform_translation,
            transform_rotation: self.transform_rotation + rhs.transform_rotation,
            transform_scale: self.transform_scale + rhs.transform_scale,
        }
    }
}

impl Sub for Anim {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            style_width: sub_val(self.style_width, rhs.style_width),
            style_height: sub_val(self.style_height, rhs.style_height),
            style_left: sub_val(self.style_left, rhs.style_left),
            style_right: sub_val(self.style_right, rhs.style_right),
            style_top: sub_val(self.style_top, rhs.style_top),
            style_bottom: sub_val(self.style_bottom, rhs.style_bottom),
            text_font_size: self.text_font_size - rhs.text_font_size,
            text_font_color: sub_color(self.text_font_color, rhs.text_font_color),
            background_color: sub_color(self.background_color, rhs.background_color),
            transform_translation: self.transform_translation - rhs.transform_translation,
            transform_rotation: self.transform_rotation - rhs.transform_rotation,
            transform_scale: self.transform_scale - rhs.transform_scale,
        }
    }
}

impl Animation {
    pub fn set_mirror(&mut self, mirror: bool) -> &mut Self {
        self.mirror = mirror;
        self
    }

    pub fn set_default(
        &mut self,
        per: Duration,
        repeat: Repeat,
        ease_method: EaseMethod,
    ) -> &mut Self {
        self.per = per;
        self.repeat = repeat;
        self.ease_method = ease_method;
        self
    }

    pub fn set_delay(&mut self, delay: Duration) -> &mut Self {
        self.per = delay;
        self
    }

    pub fn set_repeat(&mut self, repeat: Repeat) -> &mut Self {
        self.repeat = repeat;
        self
    }

    pub fn set_ease(&mut self, ease_method: EaseMethod) -> &mut Self {
        self.ease_method = ease_method;
        self
    }

    pub fn set_wh(&mut self, width: Val, height: Val) -> &mut Self {
        self.change.style_width = width;
        self.change.style_height = height;
        self
    }

    pub fn set_width(&mut self, width: Val) -> &mut Self {
        self.change.style_width = width;
        self
    }

    pub fn set_height(&mut self, height: Val) -> &mut Self {
        self.change.style_height = height;
        self
    }

    pub fn set_lt(&mut self, left: Val, top: Val) -> &mut Self {
        self.change.style_left = left;
        self.change.style_top = top;
        self
    }

    pub fn set_left(&mut self, left: Val) -> &mut Self {
        self.change.style_left = left;
        self
    }

    pub fn set_top(&mut self, top: Val) -> &mut Self {
        self.change.style_top = top;
        self
    }

    pub fn set_rb(&mut self, right: Val, bottom: Val) -> &mut Self {
        self.change.style_right = right;
        self.change.style_bottom = bottom;
        self
    }

    pub fn set_right(&mut self, right: Val) -> &mut Self {
        self.change.style_right = right;
        self
    }

    pub fn set_bottom(&mut self, bottom: Val) -> &mut Self {
        self.change.style_bottom = bottom;
        self
    }

    pub fn set_text(&mut self, font_size: f32, font_color: Color) -> &mut Self {
        self.change.text_font_size = font_size;
        self.change.text_font_color = font_color;
        self
    }

    pub fn set_background(&mut self, color: Color) -> &mut Self {
        self.change.background_color = color;
        self
    }

    pub fn set_transform(&mut self, translation: Vec3, rotation: Quat, scale: Vec3) -> &mut Self {
        self.change.transform_translation = translation;
        self.change.transform_rotation = rotation;
        self.change.transform_scale = scale;
        self
    }

    pub fn set_translation(&mut self, translation: Vec3) -> &mut Self {
        self.change.transform_translation = translation;
        self
    }

    pub fn set_rotation(&mut self, rotation: Quat) -> &mut Self {
        self.change.transform_rotation = rotation;
        self
    }

    pub fn set_scale(&mut self, scale: Vec3) -> &mut Self {
        self.change.transform_scale = scale;
        self
    }
}

#[derive(Clone, Debug)]
pub enum EaseMethod {
    EaseFun(EaseFunction),
    Linear,
    Custom(fn(f32) -> f32),
}

impl Default for EaseMethod {
    fn default() -> Self {
        EaseMethod::Linear
    }
}

impl EaseMethod {
    pub fn tick(&self, tick: f32) -> f32 {
        match self {
            EaseMethod::EaseFun(fun) => tick.calc(*fun),
            EaseMethod::Linear => tick,
            EaseMethod::Custom(fun) => fun(tick),
        }
    }
}

impl Into<EaseMethod> for EaseFunction {
    fn into(self) -> EaseMethod {
        EaseMethod::EaseFun(self)
    }
}
#[derive(Debug, Clone)]
pub enum Repeat {
    Finite(u8),
    Infinite,
    Duration(Duration),
}

impl Default for Repeat {
    fn default() -> Self {
        Self::Finite(1)
    }
}
