use crate::{
    animator::Animator,
    ui::builder::{Class, UiBuilder},
};
use bevy::prelude::*;

pub fn root<P>(
    class: impl Class<P, In = NodeBundle>,
    world: &World,
    commands: &mut Commands,
    children: impl FnOnce(&mut UiBuilder),
) -> Entity {
    let mut bundle = NodeBundle::default();
    class.apply(&mut bundle, world);
    commands
        .spawn(bundle)
        .with_children(|p| {
            children(&mut UiBuilder { builder: p, world });
        })
        .id()
}

pub fn na<P, A>(
    class: impl Class<P, In = NodeBundle>,
    animate_class: impl Class<A, In = Animator>,
    commands: &mut UiBuilder,
    children: impl FnOnce(&mut UiBuilder),
) -> Entity {
    let mut bundle = NodeBundle::default();
    class.apply(&mut bundle, commands.world);
    let mut animator = Animator::default();
    animate_class.apply(&mut animator, commands.world);
    commands
        .spawn((bundle, animator))
        .with_children(children)
        .id()
}

pub fn sa<P, A>(
    class: impl Class<P, In = SpriteBundle>,
    animate_class: impl Class<A, In = Animator>,
    ext: impl Bundle,
    world: &World,
    commands: &mut Commands,
) -> Entity {
    let mut bundle = SpriteBundle::default();
    class.apply(&mut bundle, world);
    let mut animator = Animator::default();
    animate_class.apply(&mut animator, world);
    commands.spawn((bundle, animator, ext)).id()
}

pub fn nodei<P>(
    class: impl Class<P, In = NodeBundle>,
    ext: impl Bundle,
    commands: &mut UiBuilder,
    children: impl FnOnce(&mut UiBuilder),
) -> Entity {
    let mut bundle = NodeBundle::default();
    class.apply(&mut bundle, commands.world);
    commands.spawn((bundle, ext)).with_children(children).id()
}

pub fn node<P>(
    class: impl Class<P, In = NodeBundle>,
    commands: &mut UiBuilder,
    children: impl FnOnce(&mut UiBuilder),
) -> Entity {
    nodei(class, (), commands, children)
}

pub fn texti<P, P1>(
    text: impl Into<String>,
    class: impl Class<P1, In = TextBundle>,
    text_class: impl Class<P, In = TextStyle>,
    ext: impl Bundle,
    commands: &mut UiBuilder,
) -> Entity {
    let mut style = TextStyle::default();
    text_class.apply(&mut style, commands.world);
    let mut bundle = TextBundle::from_section(text, style);
    class.apply(&mut bundle, commands.world);
    commands.spawn((bundle, ext)).id()
}

pub fn text<P, P1>(
    text: impl Into<String>,
    class: impl Class<P1, In = TextBundle>,
    text_class: impl Class<P, In = TextStyle>,
    commands: &mut UiBuilder,
) -> Entity {
    texti(text, class, text_class, (), commands)
}

pub fn text_2d<P, P1>(
    text: impl Into<String>,
    class: impl Class<P1, In = Text2dBundle>,
    text_class: impl Class<P, In = TextStyle>,
    world: &World,
    commands: &mut Commands,
) -> Entity {
    let mut style = TextStyle::default();
    text_class.apply(&mut style, world);
    let mut bundle = Text2dBundle::default();
    bundle.text = Text::from_section(text, style);
    class.apply(&mut bundle, world);
    commands.spawn(bundle).id()
}

pub fn buttoni<P>(
    class: impl Class<P, In = ButtonBundle>,
    ext: impl Bundle,
    commands: &mut UiBuilder,
    children: impl FnOnce(&mut UiBuilder),
) -> Entity {
    let mut bundle = ButtonBundle::default();
    class.apply(&mut bundle, commands.world);
    commands.spawn((bundle, ext)).with_children(children).id()
}

pub fn button<P>(
    class: impl Class<P, In = ButtonBundle>,
    commands: &mut UiBuilder,
    children: impl FnOnce(&mut UiBuilder),
) -> Entity {
    buttoni(class, (), commands, children)
}

pub fn text_buttoni<P, P1>(
    txt: impl Into<String>,
    class: impl Class<P, In = ButtonBundle>,
    text_class: impl Class<P1, In = TextStyle>,
    ext: impl Bundle,
    commands: &mut UiBuilder,
) -> Entity {
    buttoni(class, ext, commands, |p| {
        text(txt, (), text_class, p);
    })
}

pub fn text_button<P, P1>(
    txt: impl Into<String>,
    class: impl Class<P, In = ButtonBundle>,
    text_class: impl Class<P1, In = TextStyle>,
    commands: &mut UiBuilder,
) -> Entity {
    text_buttoni(txt, class, text_class, (), commands)
}

pub fn atlas_imagei<P>(
    class: impl Class<P, In = AtlasImageBundle>,
    ext: impl Bundle,
    commands: &mut UiBuilder,
) -> Entity {
    let mut bundle = AtlasImageBundle::default();
    class.apply(&mut bundle, commands.world);
    commands.spawn((bundle, ext)).id()
}

pub fn atlas_image<P>(
    class: impl Class<P, In = AtlasImageBundle>,
    commands: &mut UiBuilder,
) -> Entity {
    atlas_imagei(class, (), commands)
}

pub fn imagei<P>(
    class: impl Class<P, In = ImageBundle>,
    ext: impl Bundle,
    commands: &mut UiBuilder,
    children: impl FnOnce(&mut UiBuilder),
) -> Entity {
    let mut bundle = ImageBundle::default();
    class.apply(&mut bundle, commands.world);
    commands.spawn((bundle, ext)).with_children(children).id()
}

pub fn image<P>(
    class: impl Class<P, In = ImageBundle>,
    commands: &mut UiBuilder,
    children: impl FnOnce(&mut UiBuilder),
) -> Entity {
    imagei(class, (), commands, children)
}
