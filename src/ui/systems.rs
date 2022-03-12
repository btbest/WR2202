use bevy::prelude::*;
use crate::ui::components::*;
use crate::players::components::*;


pub fn ui_start_up_system(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    commands.spawn_bundle(UiCameraBundle::default());
    // Text with one section
    let style = Style {
        align_self: AlignSelf::Center,
        align_content: AlignContent::Center,
        justify_content: JustifyContent::Center,
        ..Default::default()
    };
    let text = Text::with_section(
        // Accepts a `String` or any type that converts into a `String`, such as `&str`
        "0 - 0",
        TextStyle {
            font: assets.load("fonts/FiraSans-Bold.ttf"),
            font_size: 100.0,
            color: Color::WHITE,
        },
        // Note: You can use `Default::default()` in place of the `TextAlignment`
        TextAlignment {
            horizontal: HorizontalAlign::Center,
            ..Default::default()
        },
    );
    commands
        .spawn_bundle(TextBundle {
            style,
            text,
            ..Default::default()
        })
        .insert(CounterText);
}



pub fn text_update_system(
    mut query_text: Query<&mut Text, With<CounterText>>,
    query_player_l: Query<&PlayerL>,
    query_player_r: Query<&PlayerR>,
) {
    let points_l = query_player_l.single().points;
    let points_r = query_player_r.single().points;
    let mut text = query_text.single_mut();
    // Update the value of the second section
    text.sections[0].value = format!("{} - {}", points_l, points_r);
}
