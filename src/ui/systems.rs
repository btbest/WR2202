use bevy::app::Events;
use bevy::prelude::*;
use crate::ui::components::*;
use crate::players::components::*;
use crate::rockets::components::*;
use crate::interaction::components::*;


pub fn ui_start_up_system(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    commands.spawn_bundle(UiCameraBundle::default());
    // Text with one section
    let style = Style {
        align_self: AlignSelf::Center,
        // align_content: AlignContent::Center,
        // justify_content: JustifyContent::Center,
        position_type: PositionType::Absolute,
        position: Rect {
            top: Val::Px(10.0),
            left: Val::Px(220.0),
            ..Default::default()
        },
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
    mut commands: Commands,
    mut query_text: Query<&mut Text, With<CounterText>>,
    query_player: Query<(&Player, &Team)>,
    query_objects: Query<Entity, Or<(With<Player>, With<Rocket>)>>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>
) {
    let mut text = query_text.single_mut();
    let (mut points_l, mut points_r) = (0, 0);
    for (player, team) in query_player.iter() {
        if team.side == 'L' {
            points_l = player.hp;
        } else if team.side == 'R' {
            points_r = player.hp;
        }
    }
    // Update the value of the second section
    text.sections[0].value = format!("{} - {}", points_l, points_r);
    let mut game_over = false;
    if points_l == 0 {
        text.sections[0].value = format!("R WIN");
        game_over = true;
    } else if points_r == 0 {
        text.sections[0].value = format!("L WIN");
        game_over = true;
    }
    if game_over {
        query_objects.for_each(|entity| {
            commands.entity(entity).despawn();
        });
        app_exit_events.send(bevy::app::AppExit)
    }
}
