use bevy::prelude::*;

pub struct GameTexture{
    pub world: Handle<Image>,
    pub logo: Handle<Image>,
    pub tree: Handle<Image>,
    pub ice: Handle<Image>,
    pub brick: Handle<Image>,
    pub iron: Handle<Image>,
    pub water: Handle<Image>,
    pub font: Handle<Font>,
    pub p1_1: Handle<TextureAtlas>,
}

pub fn c_root(node: &mut NodeBundle) {
    node.style.width = Val::Percent(100.);
    node.style.height = Val::Percent(100.);
}

pub fn c_container(img: &mut ImageBundle, game_texture: &GameTexture) {
    img.image.texture = game_texture.world.clone();
    img.style.width = Val::Px(630.);
    img.style.height = Val::Px(630.);
    img.style.padding = UiRect::all(Val::Px(3.));
    img.style.display = Display::Flex;
    img.style.flex_wrap = FlexWrap::Wrap;
    img.style.align_items = AlignItems::Start;
    img.style.justify_content = JustifyContent::Start;
}

pub fn c_container_logo(img: &mut ImageBundle, game_texture: &GameTexture) {
    img.image.texture = game_texture.world.clone();
    img.style.width = Val::Px(630.);
    img.style.height = Val::Px(630.);
    img.style.padding = UiRect::all(Val::Px(3.));
    img.style.display = Display::Flex;
    img.style.flex_wrap = FlexWrap::Wrap;
    img.style.align_items = AlignItems::Start;
    img.style.justify_content = JustifyContent::Center;
}

pub fn c_container_div_top(node: &mut NodeBundle) {
    node.style.width = Val::Percent(100.);
    node.style.height = Val::Percent(25.);
    node.style.display = Display::Flex;
    node.style.align_items = AlignItems::FlexEnd;
    node.style.justify_content = JustifyContent::Center;
}

pub fn c_container_div_bottom(node: &mut NodeBundle) {
    node.style.width = Val::Percent(100.);
    node.style.height = Val::Percent(65.);
    node.style.display = Display::Flex;
    node.style.flex_direction = FlexDirection::Column;
}

pub fn c_logo(img: &mut ImageBundle, game_texture: &GameTexture) {
    img.image.texture = game_texture.logo.clone();
    img.style.width = Val::Percent(80.);
    img.style.display = Display::Flex;
    img.style.flex_wrap = FlexWrap::Wrap;
    img.style.align_items = AlignItems::Start;
    img.style.justify_content = JustifyContent::Center;
}

pub fn c_tree(img: &mut ImageBundle, game_texture: &GameTexture) {
    img.image.texture = game_texture.tree.clone();
}

pub fn c_ice(img: &mut ImageBundle, game_texture: &GameTexture) {
    img.image.texture = game_texture.ice.clone();
}

pub fn c_brick(img: &mut ImageBundle, game_texture: &GameTexture) {
    img.image.texture = game_texture.brick.clone();
}

pub fn c_iron(img: &mut ImageBundle, game_texture: &GameTexture) {
    img.image.texture = game_texture.iron.clone();
}

pub fn c_river(img: &mut ImageBundle, game_texture: &GameTexture) {
    img.image.texture = game_texture.water.clone();
}

pub fn c_empty(img: &mut NodeBundle) {
    img.background_color = Color::rgb_u8(0, 0, 0).into();
}

pub fn c_item(img: &mut ImageBundle) {
    img.style.width = Val::Px(12.);
    img.style.height = Val::Px(12.);
}

pub fn c_item_empty(img: &mut NodeBundle) {
    img.style.width = Val::Px(12.);
    img.style.height = Val::Px(12.);
}

pub fn c_font(text: &mut TextStyle, game_texture: &GameTexture) {
    text.font_size = 20.;
    text.color = Color::rgb_u8(255, 255, 255);
    text.font = game_texture.font.clone();
}

pub fn c_select_text(text: &mut NodeBundle) {
    text.style.width = Val::Percent(100.);
    text.style.height = Val::Px(85.);
    text.style.display = Display::Flex;
    text.style.align_items = AlignItems::Center;
    text.style.justify_content = JustifyContent::Start;
    text.style.padding = UiRect::left(Val::Percent(40.));
}

pub fn c_tank(img: &mut AtlasImageBundle, game_texture: &GameTexture) {
    img.texture_atlas = game_texture.p1_1.clone();
    img.style.width = Val::Px(48.);
    img.style.height = Val::Px(48.);
    img.style.position_type = PositionType::Absolute;
    img.style.left = Val::Px(150.);
    img.style.top = Val::Px(18.5);
}
