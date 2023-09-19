use bevy::math::vec3;
use bevy::prelude::*;

use crate::timer_bar::TimerResource;
use crate::GameState;
use crate::PuzzleState;
use crate::Score;
use crate::ScoreChange;
use crate::SoundEvent;
use crate::SoundsEnum;

const ICON_SIZE_WIDTH: f32 = 100.;
const ICON_SIZE_HEIGHT: f32 = 125.;

#[derive(Component)]
struct AnswerSlot {
    x_pos: f32,
    y_pos: f32,
}

#[derive(Component)]
struct Staff;

#[derive(Component)]
struct SpriteData {
    sprite_placed: bool,
    starting_location: Vec3,
    correct_location: Vec3,
    filename: String,
}

#[derive(Resource)]
struct SpriteSelectionState {
    cursor_pos: Vec2,
    sprite: Option<(Entity, Vec3)>,
}
impl Default for SpriteSelectionState {
    fn default() -> Self {
        Self {
            cursor_pos: Vec2::MAX,
            sprite: None, // to prevent cursor position from being zero and selecting an icon (ugh)
        }
    }
}

pub struct WaltzPlugin;

impl Plugin for WaltzPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpriteSelectionState>()
            .add_systems(OnEnter(GameState::PuzzleWaltz), setup)
            .add_systems(OnEnter(PuzzleState::InProgress), begin_waltz_puzzle)
            .add_systems(
                Update,
                play_waltz_puzzle.run_if(in_state(GameState::PuzzleWaltz)),
            )
            .add_systems(
                Update,
                check_for_puzzle_completion.run_if(in_state(PuzzleState::InProgress)),
            )
            .add_systems(OnExit(GameState::PuzzleWaltz), cleanup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // answer slots
    let mut answer_slot_vector: Vec<AnswerSlot> = Vec::new();

    #[rustfmt::skip] answer_slot_vector.push(AnswerSlot {x_pos: -70., y_pos: -25.});
    #[rustfmt::skip] answer_slot_vector.push(AnswerSlot {x_pos: -70., y_pos: -150.});

    // sprites to move
    let mut tile_vector: Vec<SpriteData> = Vec::new();

    #[rustfmt::skip] tile_vector.push(SpriteData {starting_location: vec3(-600.0, 100.0, 1.0),
    #[rustfmt::skip]                              correct_location:  Vec3::MAX,
    #[rustfmt::skip]                              filename: "snowglobe/icons/waltz/2.png".to_owned(), sprite_placed: false});
    #[rustfmt::skip] tile_vector.push(SpriteData {starting_location: vec3(-600.0, -100.0, 1.0),
    #[rustfmt::skip]                              correct_location:  Vec3::MAX,
    #[rustfmt::skip]                              filename: "snowglobe/icons/waltz/8.png".to_owned(), sprite_placed: false});
    #[rustfmt::skip] tile_vector.push(SpriteData {starting_location: vec3(-600.0, -300.0, 1.0),
    #[rustfmt::skip]                              correct_location:  vec3(answer_slot_vector[1].x_pos, answer_slot_vector[1].y_pos, 1.0),
    #[rustfmt::skip]                              filename: "snowglobe/icons/waltz/4.png".to_owned(), sprite_placed: false});
    #[rustfmt::skip] tile_vector.push(SpriteData {starting_location: vec3(600.0,  100.0, 1.0),
    #[rustfmt::skip]                              correct_location:  vec3(answer_slot_vector[1].x_pos, answer_slot_vector[1].y_pos, 1.0),
    #[rustfmt::skip]                              filename: "snowglobe/icons/waltz/4.png".to_owned(), sprite_placed: false});
    #[rustfmt::skip] tile_vector.push(SpriteData {starting_location: vec3(600.0, -100.0, 1.0),
    #[rustfmt::skip]                              correct_location:  Vec3::MAX,
    #[rustfmt::skip]                              filename: "snowglobe/icons/waltz/5.png".to_owned(), sprite_placed: false});
    #[rustfmt::skip] tile_vector.push(SpriteData {starting_location: vec3(600.0, -300.0, 1.0),
    #[rustfmt::skip]                              correct_location:  vec3(answer_slot_vector[0].x_pos, answer_slot_vector[0].y_pos, 1.0),
    #[rustfmt::skip]                              filename: "snowglobe/icons/waltz/3.png".to_owned(), sprite_placed: false});

    // generate the answer slots
    for answer_slot in answer_slot_vector.iter() {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("snowglobe/icons/element background.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2 {
                        x: ICON_SIZE_WIDTH,
                        y: ICON_SIZE_HEIGHT,
                    }),
                    color: Color::GRAY,
                    ..default()
                },
                visibility: Visibility::Hidden,

                transform: Transform {
                    translation: Vec3::new(answer_slot.x_pos, answer_slot.y_pos, 0.0),
                    ..default()
                },

                ..default()
            },
            AnswerSlot {
                x_pos: answer_slot.x_pos,
                y_pos: answer_slot.y_pos,
            },
        ));
    }

    // place the tiles
    for tile in tile_vector.iter() {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load(tile.filename.clone()),
                sprite: Sprite {
                    custom_size: Some(Vec2 {
                        x: ICON_SIZE_WIDTH,
                        y: ICON_SIZE_HEIGHT,
                    }),
                    ..default()
                },
                visibility: Visibility::Hidden,

                transform: Transform {
                    translation: tile.starting_location,
                    ..default()
                },

                ..default()
            },
            SpriteData {
                starting_location: tile.starting_location,
                correct_location: tile.correct_location,
                filename: tile.filename.clone(),
                sprite_placed: false,
            },
        ));
    }

    // generate the staff
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.8, 0.8, 0.8),
                custom_size: Some(Vec2 { x: 800., y: 400. }),
                ..default()
            },
            texture: asset_server.load("snowglobe/backgrounds/staff2.png"),
            visibility: Visibility::Hidden,
            transform: Transform {
                translation: Vec3::new(0.0, -90.0, -10.0),
                ..default()
            },

            ..default()
        },
        Staff,
    ));
}

fn begin_waltz_puzzle(
    mut answer_slot_visibility_query: Query<(
        &mut Visibility,
        With<AnswerSlot>,
        Without<SpriteData>,
        Without<Staff>,
    )>,
    mut sprite_visibility_query: Query<(
        &mut Visibility,
        With<SpriteData>,
        Without<AnswerSlot>,
        Without<Staff>,
    )>,
    mut staff_visibility_query: Query<(
        &mut Visibility,
        Without<SpriteData>,
        Without<AnswerSlot>,
        With<Staff>,
    )>,
) {
    for mut answer_slot_visibility in &mut answer_slot_visibility_query.iter_mut() {
        *answer_slot_visibility.0 = Visibility::Visible;
    }

    for mut sprite_visibility in &mut sprite_visibility_query.iter_mut() {
        *sprite_visibility.0 = Visibility::Visible;
    }

    for mut staff_visibility in &mut staff_visibility_query.iter_mut() {
        *staff_visibility.0 = Visibility::Visible;
    }
}

fn play_waltz_puzzle(
    mut sprite_selection_state: ResMut<SpriteSelectionState>,
    mut sound_event_writer: EventWriter<SoundEvent>,
    windows: Query<&Window>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut sprites: Query<(Entity, &mut SpriteData)>,
    mut transforms: Query<&mut Transform>,
    mut score: ResMut<Score>,
) {
    let window = windows.single();
    let half_window = Vec2::new(window.resolution.width() / 2.0, window.height() / 2.0);

    for ev in cursor_moved_events.iter() {
        sprite_selection_state.cursor_pos = ev.position - half_window;
    }

    if mouse_button_input.just_released(MouseButton::Left) {
        if sprite_selection_state.sprite.is_some() {
            let released_entity = sprite_selection_state.sprite.unwrap().0;

            for (entity, mut sprite) in sprites.iter_mut() {
                if entity == released_entity && !sprite.sprite_placed {
                    let sprite_pos = transforms.get_mut(released_entity).unwrap().translation;

                    // determine if sprite is placed correctly here
                    let sprite_to_answer_vector = Vec2::new(
                        sprite_pos.x - sprite.correct_location.x,
                        sprite_pos.y - sprite.correct_location.y,
                    );

                    //println!("sprite position: ({}, {})", sprite_pos.x, sprite_pos.y);

                    if sprite_to_answer_vector.length() < 75. {
                        sound_event_writer.send(SoundEvent(SoundsEnum::Success));
                        sprite.sprite_placed = true;

                        *transforms.get_mut(entity).unwrap() = Transform::from_xyz(
                            sprite.correct_location.x,
                            sprite.correct_location.y,
                            sprite.correct_location.z + 2.0, // to put it on top of the answer slot
                        );
                    } else {
                        sound_event_writer.send(SoundEvent(SoundsEnum::Failure));

                        let current_score = score.current_score;
                        score.score_change_vector.push(ScoreChange {
                            initial_score: current_score,
                            delta_score: -1250.0,
                        });

                        *transforms.get_mut(entity).unwrap() = Transform::from_xyz(
                            sprite.starting_location.x,
                            sprite.starting_location.y,
                            sprite.starting_location.z,
                        );
                    }
                }
            }
        }

        sprite_selection_state.sprite = None;

        return;
    }
    if mouse_button_input.pressed(MouseButton::Left) && sprite_selection_state.sprite.is_some() {
        let sprite = sprite_selection_state.sprite.unwrap();

        let mut sprite_pos = transforms.get_mut(sprite.0).unwrap();

        sprite_pos.translation.x = sprite_selection_state.cursor_pos.x + sprite.1.x;
        sprite_pos.translation.y = -(sprite_selection_state.cursor_pos.y + sprite.1.y);
    }

    if mouse_button_input.just_pressed(MouseButton::Left) {
        for (entity, _sprite) in sprites.iter_mut() {
            let sprite_pos = transforms.get_mut(entity).unwrap().translation;

            let vector_to_sprite = Vec3::new(
                sprite_pos.x - sprite_selection_state.cursor_pos.x,
                -sprite_pos.y - sprite_selection_state.cursor_pos.y,
                0.0,
            );

            if vector_to_sprite.length() < 100. {
                sprite_selection_state.sprite = Some((entity, vector_to_sprite));
            }
        }
    }
}

fn check_for_puzzle_completion(
    mut next_puzzle_state: ResMut<NextState<PuzzleState>>,
    sprites: Query<&SpriteData>,
    mut score: ResMut<Score>,
    timer_resource: Res<TimerResource>,
    mut sound_event_writer: EventWriter<SoundEvent>,
) {
    let mut successful_sprite_count = 0;
    let mut success_found = false;
    for sprite in sprites.iter() {
        if sprite.sprite_placed {
            successful_sprite_count += 1;
        }

        if successful_sprite_count == 2 && !success_found {
            success_found = true;

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
        }
    }
}

fn cleanup(
    mut commands: Commands,
    answer_slot_query: Query<Entity, With<AnswerSlot>>,
    sprite_query: Query<Entity, With<SpriteData>>,
    staff_query: Query<Entity, With<Staff>>,
) {
    for answer_slot in answer_slot_query.iter() {
        commands.entity(answer_slot).despawn_recursive();
    }
    for sprite in sprite_query.iter() {
        commands.entity(sprite).despawn(); // no children
    }
    for staff in staff_query.iter() {
        commands.entity(staff).despawn(); // no children
    }

    commands.remove_resource::<SpriteSelectionState>();
}
