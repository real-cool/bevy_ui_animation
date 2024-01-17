use bevy::prelude::*;

pub fn add_val(a: Val, b: Val) -> Val {
    match (a, b) {
        (Val::Px(a), Val::Px(b)) => Val::Px(a + b),
        (Val::Percent(a), Val::Percent(b)) => Val::Percent(a + b),
        (Val::Vw(a), Val::Vw(b)) => Val::Vw(a + b),
        (Val::Vh(a), Val::Vh(b)) => Val::Vh(a + b),
        (Val::VMin(a), Val::VMin(b)) => Val::VMin(a + b),
        (Val::VMax(a), Val::VMax(b)) => Val::VMax(a + b),
        _ => a,
    }
}

pub fn sub_val(a: Val, b: Val) -> Val {
    match (a, b) {
        (Val::Px(a), Val::Px(b)) => Val::Px(a - b),
        (Val::Percent(a), Val::Percent(b)) => Val::Percent(a - b),
        (Val::Vw(a), Val::Vw(b)) => Val::Vw(a - b),
        (Val::Vh(a), Val::Vh(b)) => Val::Vh(a - b),
        (Val::VMin(a), Val::VMin(b)) => Val::VMin(a - b),
        (Val::VMax(a), Val::VMax(b)) => Val::VMax(a - b),
        _ => a,
    }
}

pub fn sub_color(a: Color, b: Color) -> Color {
    let a_:Vec4 = a.into();
    let b_:Vec4 = b.into();
    (a_ - b_).into()
}

