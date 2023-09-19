use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use std::time::Duration;

use crate::GameState;
use crate::PuzzleState;

#[derive(Component)]
struct TimerBar {
    previous_scale: f32,
}

#[derive(Component)]
struct TimerBarParent;

#[derive(Component)]
struct TimeRemainingText;

#[derive(Resource)]
pub struct TimerResource {
    pub time_remaining: Timer,
}

pub struct TimerBarPlugin;

impl Plugin for TimerBarPlugin {
    fn build(&self, app: &mut App) {
        // setup - @rch: fix all these setups
        app.add_systems(OnEnter(GameState::PuzzleTurkeyMistletoe), setup)
            .add_systems(OnEnter(GameState::PuzzlePeriodicTable), setup)
            .add_systems(OnEnter(GameState::PuzzleTwelveDays), setup)
            .add_systems(OnEnter(GameState::PuzzleReindeer), setup)
            .add_systems(OnEnter(GameState::PuzzleWaltz), setup)
            .add_systems(OnEnter(GameState::PuzzleGrocery), setup)
            .add_systems(OnEnter(GameState::PuzzleFamilyTree), setup)
            .add_systems(OnEnter(GameState::PuzzleCranberries), setup)
            // update
            .add_systems(
                Update,
                update.run_if(in_state(GameState::PuzzleTurkeyMistletoe)),
            )
            .add_systems(
                Update,
                update.run_if(in_state(GameState::PuzzlePeriodicTable)),
            )
            .add_systems(Update, update.run_if(in_state(GameState::PuzzleTwelveDays)))
            .add_systems(Update, update.run_if(in_state(GameState::PuzzleReindeer)))
            .add_systems(Update, update.run_if(in_state(GameState::PuzzleWaltz)))
            .add_systems(Update, update.run_if(in_state(GameState::PuzzleGrocery)))
            .add_systems(Update, update.run_if(in_state(GameState::PuzzleFamilyTree)))
            .add_systems(
                Update,
                update.run_if(in_state(GameState::PuzzleCranberries)),
            )
            // cleanup
            .add_systems(OnExit(PuzzleState::Complete), cleanup);
        //.add_systems(OnExit(GameState::PuzzleCranberries), cleanup);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    current_game_state: Res<State<GameState>>,
) {
    // timer background
    let timer_background_id = commands
        .spawn((
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Quad::new(Vec2::new(1400., 30.)).into())
                    .into(),
                material: materials.add(ColorMaterial::from(Color::DARK_GREEN)),
                visibility: Visibility::Visible,
                transform: Transform::from_translation(Vec3::new(0., 300., 5.)),
                ..default()
            },
            TimerBar { previous_scale: 1. },
        ))
        .id();

    // timer bar
    let timer_bar_id = commands
        .spawn((
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Quad::new(Vec2::new(1400., 20.)).into())
                    .into(),
                material: materials.add(ColorMaterial::from(Color::RED)),
                transform: Transform::from_translation(Vec3::new(0., 0., 10.)),
                ..default()
            },
            TimerBarParent,
        ))
        .id();

    // add the timer bar child to the parent timer background bar
    commands
        .entity(timer_background_id)
        .push_children(&[timer_bar_id]);

    commands.spawn((
        TextBundle::from_section(
            "",
            TextStyle {
                font: asset_server.load("snowglobe/fonts/MTF Dear Santa.ttf"),
                font_size: 80.0,
                color: Color::WHITE,
            },
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Val::Px(275.0),
            bottom: Val::Px(750.0),
            ..default()
        }),
        TimeRemainingText,
    ));

    let total_time: u64 = match current_game_state.get() {
        GameState::Instructions | GameState::TitleScreen | GameState::ShowScore => 0,
        GameState::PuzzlePeriodicTable
        | GameState::PuzzleTurkeyMistletoe
        | GameState::PuzzleGrocery
        | GameState::PuzzleCranberries
        | GameState::PuzzleWaltz => 20,
        GameState::PuzzleFamilyTree => 27,
        GameState::PuzzleTwelveDays | GameState::PuzzleReindeer => 60,
    };

    commands.insert_resource(TimerResource {
        time_remaining: Timer::new(Duration::new(total_time, 0), TimerMode::Once),
    });
}

fn update(
    mut timer_resource: ResMut<TimerResource>,
    mut query: Query<(&mut TimerBar, &mut Transform), With<TimerBar>>,
    mut text_query: Query<&mut Text, With<TimeRemainingText>>,
    time: Res<Time>,
    puzzle_state: ResMut<State<PuzzleState>>,
    mut next_puzzle_state: ResMut<NextState<PuzzleState>>,
) {
    // decrement the remaining time - if the puzzle is in progress
    if puzzle_state.get().clone() == PuzzleState::InProgress {
        timer_resource.time_remaining.tick(time.delta());
    }

    let time_remaining =
        timer_resource.time_remaining.duration() - timer_resource.time_remaining.elapsed();

    // the first format! makes a string - the second one removes the quotes
    let time_remaining_string: String = format!("{}", format!("{:?}", time_remaining));

    let time_remaining_truncated: &str;
    let mut leading_zero = "0.".to_owned(); // has to be declared out here so it 'lives long enough'

    if time_remaining > Duration::from_secs(10) {
        time_remaining_truncated = truncate(&time_remaining_string, 4);
    } else {
        if time_remaining >= Duration::from_secs(1) {
            time_remaining_truncated = truncate(&time_remaining_string, 3);
        } else {
            leading_zero.push_str(&time_remaining_string.to_owned());
            time_remaining_truncated = truncate(&leading_zero, 3);
        }
    }

    // set the time remaining text
    for mut text in &mut text_query {
        if time_remaining > Duration::ZERO {
            text.sections[0].value = format!("time remaining: {:}", time_remaining_truncated);
        } else {
            text.sections[0].value = format!("time remaining: 0.0");
        }
    }

    // scale the bar
    for (mut timer_bar, mut transform) in &mut query {
        let scale_percentage: f32 = 1.
            - (timer_resource.time_remaining.elapsed().as_millis() as f32
                / timer_resource.time_remaining.duration().as_millis() as f32);

        // first, undo the previous scale
        transform.scale.x *= 1. / timer_bar.previous_scale;
        transform.scale.x *= scale_percentage;

        timer_bar.previous_scale = scale_percentage;
    }

    if time_remaining <= Duration::ZERO && puzzle_state.get().clone() == PuzzleState::InProgress {
        next_puzzle_state.set(PuzzleState::Complete);
    }
}

// helper function for the time remaining string (copied from somewhere)
fn truncate(s: &String, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        None => s,
        Some((idx, _)) => &s[..idx],
    }
}

fn cleanup(
    mut commands: Commands,
    button: Query<Entity, With<Button>>,
    time_remaining_text: Query<Entity, With<TimeRemainingText>>,
    timer_bar_parent_query: Query<Entity, With<TimerBarParent>>,
    timer_bar_query: Query<Entity, With<TimerBar>>,
) {
    commands.entity(button.single()).despawn_recursive();
    commands.entity(time_remaining_text.single()).despawn();
    commands
        .entity(timer_bar_parent_query.single())
        .despawn_recursive();
    commands
        .entity(timer_bar_query.single())
        .despawn_recursive();

    //commands.remove_resource::<TimerResource>();
}
