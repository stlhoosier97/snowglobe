use crate::GameState;
use crate::Score;
use bevy::prelude::*;

pub struct ScorePlugin;

const SCORE_FONT_SIZE: f32 = 80.;
const SCORE_CHANGE_SPEED: f32 = 50.0;

#[derive(Component)]
struct ScoreText;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::PuzzleTurkeyMistletoe), setup_score) // call only once
            //.add_systems(OnEnter(GameState::PuzzlePeriodicTable), unhide_score) // call only once
            .add_systems(OnEnter(GameState::PuzzleCranberries), unhide_score) // call only once - @rch: temporary
            .add_systems(
                Update,
                update_score.run_if(in_state(GameState::PuzzleTurkeyMistletoe)),
            )
            .add_systems(
                Update,
                update_score.run_if(in_state(GameState::PuzzlePeriodicTable)),
            )
            .add_systems(
                Update,
                update_score.run_if(in_state(GameState::PuzzleTwelveDays)),
            )
            .add_systems(
                Update,
                update_score.run_if(in_state(GameState::PuzzleReindeer)),
            )
            .add_systems(
                Update,
                update_score.run_if(in_state(GameState::PuzzleWaltz)),
            )
            .add_systems(
                Update,
                update_score.run_if(in_state(GameState::PuzzleGrocery)),
            )
            .add_systems(
                Update,
                update_score.run_if(in_state(GameState::PuzzleFamilyTree)),
            )
            .add_systems(
                Update,
                update_score.run_if(in_state(GameState::PuzzleCranberries)),
            )
            .add_systems(OnExit(GameState::PuzzleCranberries), hide_score);
    }
}

fn setup_score(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_section(
            "current score:",
            TextStyle {
                font: asset_server.load("snowglobe/fonts/MTF Dear Santa.ttf"),
                font_size: SCORE_FONT_SIZE,
                color: Color::WHITE,
            },
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Val::Px(875.0),
            bottom: Val::Px(750.0),
            ..default()
        }),
        ScoreText,
    ));
}

fn update_score(mut score: ResMut<Score>, mut score_text_query: Query<&mut Text, With<ScoreText>>) {
    // update the text
    for mut score_text in &mut score_text_query {
        score_text.sections[0].value = format!("current score: {:}", score.current_score as i32);
        if !score.score_change_vector.is_empty() {
            match score.score_change_vector.first().unwrap().delta_score > 0.0 {
                true => score_text.sections[0].style.color = Color::GREEN,
                false => score_text.sections[0].style.color = Color::RED,
            }
        } else {
            score_text.sections[0].style.color = Color::WHITE;
        }
    }

    // see if there is a score change to process
    if !score.score_change_vector.is_empty() {
        let delta_score = score.score_change_vector.first().unwrap().delta_score;
        let initial_score = score.score_change_vector.first().unwrap().initial_score;

        score.current_score += delta_score / SCORE_CHANGE_SPEED;

        // see if we are done changing the score
        if ((delta_score + initial_score) - score.current_score).abs() < 0.5 {
            // pop off this score change
            score.score_change_vector.pop();
            // add the mistake
            if delta_score < 0.0 {
                score.mistakes = score.mistakes + 1;
            }

            // if there is a new one - update the next elements current score from what was originally entered
            if !score.score_change_vector.is_empty() {
                score.score_change_vector.first_mut().unwrap().initial_score = score.current_score;
            }
        }
    }
}

fn unhide_score(mut score_text_query: Query<&mut Text, With<ScoreText>>) {
    for mut score_text in &mut score_text_query {
        score_text.sections[0].style.font_size = SCORE_FONT_SIZE;
    }
}

fn hide_score(mut score_text_query: Query<&mut Text, With<ScoreText>>) {
    for mut score_text in &mut score_text_query {
        score_text.sections[0].style.font_size = 0.; // can't seem to remove visibility of a text bundle so set font to 0
    }
}
