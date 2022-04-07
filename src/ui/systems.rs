use bevy::prelude::*;
use crate::ui::components::*;
use crate::players::components::*;
use crate::interaction::components::*;
use crate::states::GameState;
use std::path::Path;


pub fn ui_start_up_system(
    mut commands: Commands
) {
    commands.spawn_bundle(UiCameraBundle::default());
}

pub fn start_menu(
    mut commands: Commands,
    assets: Res<AssetServer>,
    windows: Res<Windows>
) {
    let window = windows.get_primary().unwrap();
    let menu_style = Style {
        align_self: AlignSelf::Center,
        // align_content: AlignContent::Center,
        // justify_content: JustifyContent::Center,
        position_type: PositionType::Absolute,
        position: Rect {
            top: Val::Px(10.0),
            left: Val::Px(window.width() / 2. - 220.), // definitely not empirically determined
            ..Default::default()
        },
        ..Default::default()
    };
    let menu_text = Text::with_section(
        // Accepts a `String` or any type that converts into a `String`, such as `&str`
        "Press Enter\n to start!",
        TextStyle {
            font: assets.load(Path::new("fonts").join("FiraSans-Bold.ttf")),
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
            style: menu_style,
            text: menu_text,
            ..Default::default()
        })
        .insert(MenuText);
}

pub fn setup_score_counter(
    mut commands: Commands,
    assets: Res<AssetServer>,
    windows: Res<Windows>
) {
    let window = windows.get_primary().unwrap();
    // Text with one section
    let counter_style = Style {
        align_self: AlignSelf::Center,
        // align_content: AlignContent::Center,
        // justify_content: JustifyContent::Center,
        position_type: PositionType::Absolute,
        position: Rect {
            top: Val::Px(10.0),
            left: Val::Px(window.width() / 2. - 84.), // definitely not empirically determined
            ..Default::default()
        },
        ..Default::default()
    };
    let counter_text = Text::with_section(
        // Accepts a `String` or any type that converts into a `String`, such as `&str`
        "0 - 0",
        TextStyle {
            font: assets.load(Path::new("fonts").join("FiraSans-Bold.ttf")),
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
            style: counter_style,
            text: counter_text,
            ..Default::default()
        })
        .insert(CounterText);
}

pub fn gameover_menu(
    mut commands: Commands,
    assets: Res<AssetServer>,
    windows: Res<Windows>,
    game_state: Res<State<GameState>>
) {
    let window = windows.get_primary().unwrap();
    let menu_text = match game_state.current() {
        GameState::GameOver('L') => "Left wins!\nPress Enter\n to restart!",
        GameState::GameOver('R') => "Right wins!\nPress Enter\n to restart!",
        _ => ""
    };
    let menu_style = Style {
        align_self: AlignSelf::Center,
        // align_content: AlignContent::Center,
        // justify_content: JustifyContent::Center,
        position_type: PositionType::Absolute,
        position: Rect {
            top: Val::Px(10.0),
            left: Val::Px(window.width() / 2. - 220.), // definitely not empirically determined
            ..Default::default()
        },
        ..Default::default()
    };
    let menu_text = Text::with_section(
        // Accepts a `String` or any type that converts into a `String`, such as `&str`
        menu_text,
        TextStyle {
            font: assets.load(Path::new("fonts").join("FiraSans-Bold.ttf")),
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
            style: menu_style,
            text: menu_text,
            ..Default::default()
        })
        .insert(MenuText);
}


pub fn score_counter(
    mut query_text: Query<&mut Text, With<CounterText>>,
    query_player: Query<(&Player, &Team)>,
    mut game_state: ResMut<State<GameState>>
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
    if points_l == 0 {
        game_state.set(GameState::GameOver('R')).unwrap();
    } else if points_r == 0 {
        game_state.set(GameState::GameOver('L')).unwrap();
    }
}


pub fn start_game_by_pressing_return_system(
    input: Res<Input<KeyCode>>,
    mut game_state: ResMut<State<GameState>>
) {
    if input.just_pressed(KeyCode::Return) {
        game_state.set(GameState::InGame).unwrap()
    }
}