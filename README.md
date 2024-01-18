![animations](https://github.com/real-cool/bevy_ui_animation/assets/157036900/60921d98-8ee3-483d-b0ae-6f31dbe252ff)
```rust
fn setup_ui(mut commands: Commands, world: &World) {
    commands.spawn(Camera2dBundle::default());
    let mut red_sprite = None;
    let mut blue_sprite = None;
    text_2d("progress red: 0%", text_1, text_red, world, &mut commands).set(&mut red_sprite);
    text_2d("progress blue: 0%", text_2, text_blue, world, &mut commands).set(&mut blue_sprite);
    sa(red_style, red_animate, RedBlock, world, &mut commands);
    sa(blue_style, blue_animate, BlueBlock, world, &mut commands);
    commands.entity(red_sprite.unwrap()).insert(UiText::Red);
    commands.entity(blue_sprite.unwrap()).insert(UiText::Blue);
}

fn red_style(sp: &mut SpriteBundle) {
    sp.sprite.custom_size = Some(Vec2::new(25., 25.));
    sp.sprite.color = Color::RED;
    sp.transform.translation = Vec3::new(-260., -260., 0.);
}

fn blue_style(sp: &mut SpriteBundle) {
    sp.sprite.custom_size = Some(Vec2::new(25., 25.));
    sp.sprite.color = Color::RED;
    sp.transform.translation = Vec3::new(0., 0., 0.);
}

fn text_1(bundle: &mut Text2dBundle) {
    bundle.transform.translation = Vec3::new(0., 40., 0.);
    bundle.text.alignment = TextAlignment::Center;
}

fn text_2(bundle: &mut Text2dBundle) {
    bundle.transform.translation = Vec3::new(0., -40., 0.);
    bundle.text.alignment = TextAlignment::Center;
}

fn text_blue(style: &mut TextStyle, asset_server: &AssetServer) {
    style.font = asset_server.load("fonts/FiraSans-Regular.ttf");
    style.font_size = 50.;
    style.color = Color::BLUE;
}

fn text_red(style: &mut TextStyle, asset_server: &AssetServer) {
    style.font = asset_server.load("fonts/FiraSans-Regular.ttf");
    style.font_size = 50.;
    style.color = Color::RED;
}

fn red_animate(animator: &mut Animator) {
    animator
        .set_loop(true)
        .set_exec(true)
        .add_change()
        //.set_repeat(Repeat::Finite(5))
        .set_delay(Duration::from_secs(1))
        .set_ease(EaseFunction::QuadraticInOut.into())
        .set_rotation(Quat::from_array([0., 0., 1., 0.]))
        .set_translation(Vec3::new(520., 0., 0.));
    animator
        .add_change()
        .set_delay(Duration::from_secs(1))
        .set_ease(EaseFunction::QuadraticInOut.into())
        .set_rotation(Quat::from_array([0., 0., -1., 0.]))
        .set_translation(Vec3::new(0., 520., 0.));
    animator
        .add_change()
        .set_delay(Duration::from_secs(1))
        .set_ease(EaseFunction::QuadraticInOut.into())
        .set_rotation(Quat::from_array([0., 0., 1., 0.]))
        .set_translation(Vec3::new(-520., 0., 0.));
    animator
        .add_change()
        .set_ease(EaseFunction::QuadraticInOut.into())
        .set_delay(Duration::from_secs(1))
        .set_rotation(Quat::from_array([0., 0., -1., 0.]))
        .set_translation(Vec3::new(0., -520., 0.));
}

fn blue_animate(animator: &mut Animator) {}

fn update_text(
    red_ani: Query<&Animator, With<RedBlock>>,
    blue_ani: Query<&Animator, With<BlueBlock>>,
    mut query_text: Query<(&mut Text, &UiText)>,
) {
    let red_ani = red_ani.single();
    let blue_ani = blue_ani.single();
    for (mut text, ui_text) in query_text.iter_mut() {
        match ui_text {
            UiText::Red => {
                text.sections[0].value = format!(
                    "progress red:{}%",
                    (red_ani.progress() / red_ani.total_times() * 100.).trunc()
                )
            }
            UiText::Blue => {
                text.sections[0].value = format!(
                    "progress blue:{}%",
                    blue_ani.progress() / blue_ani.total_times() * 100.
                )
            }
        }
    }
}
```
