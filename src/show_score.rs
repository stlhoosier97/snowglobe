use crate::BackgroundImage;
use crate::GameState;
use crate::Score;
use crate::SoundEvent;
use crate::SoundsEnum;
use bevy::prelude::*;

const FINAL_SCORE_FONT_SIZE: f32 = 120.;
pub struct ShowScorePlugin;

impl Plugin for ShowScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::ShowScore), setup_show_score);
    }
}

fn setup_show_score(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut sound_event_writer: EventWriter<SoundEvent>,
    score: Res<Score>,
) {
    commands.spawn((SpriteBundle {
        texture: asset_server.load("snowglobe/text/show score.png"),
        sprite: Sprite {
            custom_size: Some(Vec2 { x: 1600., y: 800. }),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            ..default()
        },

        ..default()
    },));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.4, 0.4, 0.4, 1.0),
                custom_size: Some(Vec2 { x: 1600., y: 800. }),
                ..default()
            },
            texture: asset_server.load("snowglobe/backgrounds/title screen.png"),
            visibility: Visibility::Visible,
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, -10.0),
                ..default()
            },

            ..default()
        },
        BackgroundImage,
    ));

    let text_to_display = format!(
        "your final score: {:}\ntotal mistakes: {:}",
        score.current_score as i32, score.mistakes
    );

    commands.spawn((TextBundle::from_section(
        text_to_display,
        TextStyle {
            font: asset_server.load("snowglobe/fonts/MTF Dear Santa.ttf"),
            font_size: FINAL_SCORE_FONT_SIZE,
            color: Color::GREEN,
        },
    )
    .with_text_alignment(TextAlignment::Center)
    .with_style(Style {
        position_type: PositionType::Absolute,
        left: Val::Px(500.0),
        top: Val::Px(500.0),
        ..default()
    }),));

    sound_event_writer.send(SoundEvent(SoundsEnum::MerryChristmas));
}
