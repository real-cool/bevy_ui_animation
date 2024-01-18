use crate::ani_utils::*;
use bevy::prelude::*;
use interpolation::*;
use std::{
    ops::{Add, Sub},
    time::Duration,
};

#[derive(Component, Default, Debug)]
pub struct Animator {
    total_times: f32,
    exectable: bool,
    exec_index: usize,
    exec_loop: bool,
    exec_step: bool,
    forward: bool,
    progress: f32,
    fract: f32,
    init_style: bool,
    init_text: bool,
    init_background: bool,
    init_transform: bool,
    init_sprite: bool,
    start: Anim,
    animations: Vec<Animation>,
}

impl Animator {
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

    pub fn add_change_start(&mut self, index: usize) {
        self.start = self.start + self.animations[self.exec_index].change;
        self.start(index);
    }

    pub fn progress(&self) -> f32 {
        self.progress
    }

    pub fn start(&mut self, index: usize) {
        self.reset_progress();
        if self.exec_index == index {
            return;
        }
        if self.exec_index > index {
            for i in index..self.exec_index {
                self.start = self.start - (&self.animations[i]).change;
            }
        } else if self.exec_index < index {
            for i in self.exec_index..index {
                self.start = self.start + (&self.animations[i]).change;
            }
        }
        self.exec_index = index;
    }

    fn add_delta(&mut self, delta: Duration) {
        if let Some(animation) = self.animations.get(self.exec_index) {
            self.progress =
                self.progress + (delta.as_secs_f64() / animation.per.as_secs_f64()) as f32;
            self.fract = self.progress.fract();
            self.forward = self.progress.trunc() as u8 % 2 == 0;
        }
    }

    fn reset_progress(&mut self) {
        self.progress = 0.;
        self.fract = 0.;
        self.forward = true;
    }

    pub fn total_times(&self) -> f32 {
        self.total_times
    }

    pub fn tick_progress(&mut self, delta: Duration) {
        if !self.exectable {
            return;
        }
        if self.progress == 0. {
            self.total_times = self.animations[self.exec_index].total_times;
            self.add_delta(delta);
        } else if self.progress < self.total_times {
            self.add_delta(delta);
        } else if self.exec_step {
            self.progress = self.total_times;
        } else if self.exec_loop {
            self.start((self.exec_index + 1) % self.animations.len());
        } else if self.exec_index + 1 == self.animations.len() {
            self.progress = self.total_times;
        } else {
            self.start(self.exec_index + 1);
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

    pub fn init_sprite(&mut self, sprite: &mut Sprite) {
        if !self.init_sprite {
            if let Some(size) = sprite.custom_size {
                self.start.sprite_width = size.x;
                self.start.sprite_height = size.y;
            }
            self.start.sprite_color = sprite.color;
            self.init_sprite = true;
        }
    }

    pub fn tick_style(&mut self, style: &mut Style) {
        self.init_style(style);
        if let Some(ani) = self.animations.get(self.exec_index) {
            self.start.lerp_style(
                style,
                &ani.change,
                ani.ease_method.tick(if self.forward {
                    self.fract
                } else {
                    1. - self.fract
                }),
            );
        }
    }

    pub fn tick_text(&mut self, text: &mut Text) {
        self.init_text(text);
        if let Some(ani) = self.animations.get(self.exec_index) {
            self.start.lerp_text(
                text,
                &ani.change,
                ani.ease_method.tick(if self.forward {
                    self.fract
                } else {
                    1. - self.fract
                }),
            );
        }
    }

    pub fn tick_background(&mut self, background: &mut BackgroundColor) {
        self.init_background(background);
        if let Some(ani) = self.animations.get(self.exec_index) {
            self.start.lerp_background(
                background,
                &ani.change,
                ani.ease_method.tick(if self.forward {
                    self.fract
                } else {
                    1. - self.fract
                }),
            );
        }
    }

    pub fn tick_transform(&mut self, transform: &mut Transform) {
        self.init_transform(transform);
        if let Some(ani) = self.animations.get(self.exec_index) {
            self.start.lerp_transform(
                transform,
                &ani.change,
                ani.ease_method.tick(if self.forward {
                    self.fract
                } else {
                    1. - self.fract
                }),
            );
        }
    }

    pub fn tick_sprite(&mut self, sprite: &mut Sprite) {
        self.init_sprite(sprite);
        if let Some(ani) = self.animations.get(self.exec_index) {
            self.start.lerp_sprite(
                sprite,
                &ani.change,
                ani.ease_method.tick(if self.forward {
                    self.fract
                } else {
                    1. - self.fract
                }),
            );
        }
    }
}

#[derive(Default, Debug)]
pub struct Animation {
    per: Duration,
    repeat: Repeat,
    total_times: f32,
    mirror: bool,
    ease_method: EaseMethod,
    change: Anim,
}

#[derive(Clone, Copy, Debug, PartialEq)]
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
    sprite_width: f32,
    sprite_height: f32,
    sprite_color: Color,
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
            sprite_width: 0.,
            sprite_height: 0.,
            sprite_color: Color::NONE,
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
        if change.style_width != Val::Auto {
            target.width = lerp_val!(self, change, style_width, ratio);
        }
        if change.style_height != Val::Auto {
            target.height = lerp_val!(self, change, style_height, ratio);
        }
        if change.style_left != Val::Auto {
            target.left = lerp_val!(self, change, style_left, ratio);
        }
        if change.style_right != Val::Auto {
            target.right = lerp_val!(self, change, style_right, ratio);
        }
        if change.style_top != Val::Auto {
            target.top = lerp_val!(self, change, style_top, ratio);
        }
        if change.style_bottom != Val::Auto {
            target.bottom = lerp_val!(self, change, style_bottom, ratio);
        }
    }

    fn lerp_text(&self, target: &mut Text, change: &Self, ratio: f32) {
        if let Some(section) = target.sections.get_mut(0) {
            if change.text_font_size != 0. {
                section.style.font_size = self.text_font_size + change.text_font_size * ratio;
            }
            if change.text_font_color != Color::NONE {
                section.style.color = self.text_font_color + change.text_font_color * ratio;
            }
        }
    }

    fn lerp_background(&self, target: &mut BackgroundColor, change: &Self, ratio: f32) {
        if target.0 != Color::NONE {
            target.0 = self.background_color + change.background_color * ratio;
        }
    }

    fn lerp_transform(&self, target: &mut Transform, change: &Self, ratio: f32) {
        if change.transform_translation != Vec3::ZERO {
            target.translation = self.transform_translation + change.transform_translation * ratio;
        }
        if change.transform_rotation != Quat::from_vec4(Vec4::ZERO) {
            target.rotation = self
                .transform_rotation
                .slerp(change.transform_rotation + self.transform_rotation, ratio);
        }
        if change.transform_scale != Vec3::ZERO {
            target.scale = self.transform_scale + change.transform_scale * ratio;
        }
    }

    fn lerp_sprite(&self, target: &mut Sprite, change: &Self, ratio: f32) {
        if change.sprite_width != 0. || change.sprite_height != 0. {
            target.custom_size = Some(Vec2::new(
                self.sprite_width + change.sprite_width * ratio,
                self.sprite_height + change.sprite_height * ratio,
            ))
        }
        if change.sprite_color != Color::NONE {
            target.color = self.sprite_color + change.sprite_color * ratio;
        }
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
            sprite_width: self.sprite_width + rhs.sprite_width,
            sprite_height: self.sprite_height + rhs.sprite_height,
            sprite_color: self.sprite_color + rhs.sprite_color,
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
            sprite_width: self.sprite_width - rhs.sprite_width,
            sprite_height: self.sprite_height - rhs.sprite_height,
            sprite_color: sub_color(self.sprite_color, rhs.sprite_color),
        }
    }
}

impl Animation {
    fn get_total_times(&self) -> f32 {
        match self.repeat {
            Repeat::Finite(count) => count as f32,
            Repeat::Infinite => f32::MAX,
            Repeat::Duration(duration) => duration.as_secs_f32() / self.per.as_secs_f32(),
        }
    }

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
        self.total_times = self.get_total_times();
        self.ease_method = ease_method;
        self
    }

    pub fn set_delay(&mut self, delay: Duration) -> &mut Self {
        self.per = delay;
        self.total_times = self.get_total_times();
        self
    }

    pub fn set_repeat(&mut self, repeat: Repeat) -> &mut Self {
        self.repeat = repeat;
        self.total_times = self.get_total_times();
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

    pub fn set_sprite(&mut self, width: f32, height: f32, color: Color) -> &mut Self {
        self.change.sprite_width = width;
        self.change.sprite_height = height;
        self.change.sprite_color = color;
        self
    }

    pub fn set_sprite_size(&mut self, width: f32, height: f32) -> &mut Self {
        self.change.sprite_width = width;
        self.change.sprite_height = height;
        self
    }

    pub fn set_sprite_color(&mut self, color: Color) -> &mut Self {
        self.change.sprite_color = color;
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
