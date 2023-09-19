use bevy::prelude::*;

use crate::timer_bar::TimerResource;
use crate::GameState;
use crate::PuzzleState;
use crate::Score;
use crate::ScoreChange;
use crate::SoundEvent;
use crate::SoundsEnum;

#[derive(Component)]
struct StoreLayout;

#[derive(Resource)]
struct CursorPositionState {
    cursor_pos: Vec2,
}
impl Default for CursorPositionState {
    fn default() -> Self {
        Self {
            cursor_pos: Vec2::MAX,
        }
    }
}
pub struct GroceryPlugin;

impl Plugin for GroceryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorPositionState>()
            .add_systems(OnEnter(GameState::PuzzleGrocery), setup)
            .add_systems(OnEnter(PuzzleState::InProgress), begin_grocery_puzzle)
            .add_systems(
                Update,
                play_grocery_puzzle.run_if(in_state(GameState::PuzzleGrocery)),
            )
            .add_systems(OnExit(GameState::PuzzleGrocery), cleanup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // generate the store layout
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.8, 0.8, 0.8),
                //color: Color::WHITE,
                custom_size: Some(Vec2 { x: 800., y: 400. }),
                ..default()
            },
            texture: asset_server.load("snowglobe/icons/grocery.png"),
            visibility: Visibility::Hidden,
            transform: Transform {
                translation: Vec3::new(0.0, -100.0, 10.0),
                ..default()
            },

            ..default()
        },
        StoreLayout,
    ));
}

fn begin_grocery_puzzle(
    mut store_layout_visibility_query: Query<(&mut Visibility, With<StoreLayout>)>,
) {
    for mut store_layout_visibility in &mut store_layout_visibility_query.iter_mut() {
        *store_layout_visibility.0 = Visibility::Visible;
    }
}

fn play_grocery_puzzle(
    mut sound_event_writer: EventWriter<SoundEvent>,
    windows: Query<&Window>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut cursor_position_resource: ResMut<CursorPositionState>,
    mut next_puzzle_state: ResMut<NextState<PuzzleState>>,
    current_puzzle_state: Res<State<PuzzleState>>,
    timer_resource: Res<TimerResource>,
    mut score: ResMut<Score>,
) {
    if *current_puzzle_state.get() == PuzzleState::InProgress {
        let window = windows.single();
        let half_window = Vec2::new(window.resolution.width() / 2.0, window.height() / 2.0);

        for ev in cursor_moved_events.iter() {
            cursor_position_resource.cursor_pos = ev.position - half_window;
        }

        if mouse_button_input.just_released(MouseButton::Left) {
            //println!("cursor position is: ({}, {})", cursor_position_resource.cursor_pos.x, cursor_position_resource.cursor_pos.y);

            if cursor_position_resource.cursor_pos.x > 25.
                && cursor_position_resource.cursor_pos.x < 190.
                && cursor_position_resource.cursor_pos.y > 90.
                && cursor_position_resource.cursor_pos.y < 220.
            {
                let time_remaining = (timer_resource.time_remaining.duration()
                    - timer_resource.time_remaining.elapsed())
                .as_millis() as f32;

                let current_score = score.current_score;
                score.score_change_vector.push(ScoreChange {
                    initial_score: current_score,
                    delta_score: time_remaining,
                });

                sound_event_writer.send(SoundEvent(SoundsEnum::HoHoHo));

                next_puzzle_state.set(PuzzleState::Complete);
            } else {
                if cursor_position_resource.cursor_pos.x.abs() < 5000.
                    && cursor_position_resource.cursor_pos.y.abs() < 5000.
                {
                    sound_event_writer.send(SoundEvent(SoundsEnum::Failure));

                    let current_score = score.current_score;
                    score.score_change_vector.push(ScoreChange {
                        initial_score: current_score,
                        delta_score: -1250.0,
                    });
                }
            }
        }
    }
}

fn cleanup(mut commands: Commands, store_layout_query: Query<Entity, With<StoreLayout>>) {
    for store_layout in store_layout_query.iter() {
        commands.entity(store_layout).despawn_recursive();
    }

    commands.remove_resource::<CursorPositionState>();
}
