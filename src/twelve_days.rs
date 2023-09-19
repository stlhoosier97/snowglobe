use bevy::math::vec3;
use bevy::prelude::*;
use bevy::text::BreakLineOn;

use crate::timer_bar::TimerResource;
use crate::GameState;
use crate::PuzzleState;
use crate::Score;
use crate::ScoreChange;
use crate::SoundEvent;
use crate::SoundsEnum;

const ICON_SIZE_WIDTH: f32 = 100.;
const ICON_SIZE_HEIGHT: f32 = 150.;
const FONT_SIZE: f32 = 100.;

#[derive(Component)]
struct AnswerSlot {
    x_pos: f32,
    y_pos: f32,
    text: String,
}

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
            sprite: None, // to prevent cursor position from being zero and selecting an icon s(ugh)
        }
    }
}

pub struct TwelveDaysPlugin;

impl Plugin for TwelveDaysPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpriteSelectionState>()
            .add_systems(OnEnter(GameState::PuzzleTwelveDays), setup)
            .add_systems(OnEnter(PuzzleState::InProgress), begin_twelve_days_puzzle)
            .add_systems(
                Update,
                play_twelve_days_puzzle.run_if(in_state(GameState::PuzzleTwelveDays)),
            )
            .add_systems(
                Update,
                check_for_puzzle_completion.run_if(in_state(PuzzleState::InProgress)),
            )
            .add_systems(OnExit(GameState::PuzzleTwelveDays), cleanup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    const STARTING_X_POS: f32 = -700.0;
    const X_POS_GAP: f32 = 20.0;
    const STARTING_Y_POS_ANSWER: f32 = -200.0;
    const STARTING_Y_POS_TILE_OFFSET: f32 = 200.0;

    // answer slots
    let mut answer_slot_vector: Vec<AnswerSlot> = Vec::new();

    #[rustfmt::skip] answer_slot_vector.push(AnswerSlot {x_pos: STARTING_X_POS + (0.0  * ICON_SIZE_WIDTH) + (0.0 * X_POS_GAP), y_pos: STARTING_Y_POS_ANSWER, text: "1".to_owned(),});
    #[rustfmt::skip] answer_slot_vector.push(AnswerSlot {x_pos: STARTING_X_POS + (1.0  * ICON_SIZE_WIDTH) + (1.0 * X_POS_GAP), y_pos: STARTING_Y_POS_ANSWER, text: "2".to_owned(),});
    #[rustfmt::skip] answer_slot_vector.push(AnswerSlot {x_pos: STARTING_X_POS + (2.0  * ICON_SIZE_WIDTH) + (2.0 * X_POS_GAP), y_pos: STARTING_Y_POS_ANSWER, text: "3".to_owned(),});
    #[rustfmt::skip] answer_slot_vector.push(AnswerSlot {x_pos: STARTING_X_POS + (3.0  * ICON_SIZE_WIDTH) + (3.0 * X_POS_GAP), y_pos: STARTING_Y_POS_ANSWER, text: "4".to_owned(),});
    #[rustfmt::skip] answer_slot_vector.push(AnswerSlot {x_pos: STARTING_X_POS + (4.0  * ICON_SIZE_WIDTH) + (4.0 * X_POS_GAP), y_pos: STARTING_Y_POS_ANSWER, text: "5".to_owned(),});
    #[rustfmt::skip] answer_slot_vector.push(AnswerSlot {x_pos: STARTING_X_POS + (5.0  * ICON_SIZE_WIDTH) + (5.0 * X_POS_GAP), y_pos: STARTING_Y_POS_ANSWER, text: "6".to_owned(),});
    #[rustfmt::skip] answer_slot_vector.push(AnswerSlot {x_pos: STARTING_X_POS + (6.0  * ICON_SIZE_WIDTH) + (6.0 * X_POS_GAP), y_pos: STARTING_Y_POS_ANSWER, text: "7".to_owned(),});
    #[rustfmt::skip] answer_slot_vector.push(AnswerSlot {x_pos: STARTING_X_POS + (7.0  * ICON_SIZE_WIDTH) + (7.0 * X_POS_GAP), y_pos: STARTING_Y_POS_ANSWER, text: "8".to_owned(),});
    #[rustfmt::skip] answer_slot_vector.push(AnswerSlot {x_pos: STARTING_X_POS + (8.0  * ICON_SIZE_WIDTH) + (8.0 * X_POS_GAP), y_pos: STARTING_Y_POS_ANSWER, text: "9".to_owned(),});
    #[rustfmt::skip] answer_slot_vector.push(AnswerSlot {x_pos: STARTING_X_POS + (9.0  * ICON_SIZE_WIDTH) + (9.0 * X_POS_GAP), y_pos: STARTING_Y_POS_ANSWER, text: "10".to_owned(),});
    #[rustfmt::skip] answer_slot_vector.push(AnswerSlot {x_pos: STARTING_X_POS + (10.0 * ICON_SIZE_WIDTH) + (10.0 * X_POS_GAP), y_pos: STARTING_Y_POS_ANSWER, text: "11".to_owned(),});
    #[rustfmt::skip] answer_slot_vector.push(AnswerSlot {x_pos: STARTING_X_POS + (11.0 * ICON_SIZE_WIDTH) + (11.0 * X_POS_GAP), y_pos: STARTING_Y_POS_ANSWER, text: "12".to_owned(),});

    // sprites to move
    let mut tile_vector: Vec<SpriteData> = Vec::new();

    // uncomment this to put in order
    //let shuffle_slots:Vec<usize> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    let shuffle_slots: Vec<usize> = vec![7, 3, 8, 0, 2, 6, 5, 10, 11, 9, 4, 1];

    #[rustfmt::skip] tile_vector.push(SpriteData {starting_location: vec3(answer_slot_vector[shuffle_slots[0]].x_pos, answer_slot_vector[shuffle_slots[0]].y_pos + STARTING_Y_POS_TILE_OFFSET, 1.0), 
    #[rustfmt::skip]                              correct_location:  vec3(answer_slot_vector[0].x_pos, answer_slot_vector[0].y_pos, 0.0),
    #[rustfmt::skip]                              filename: "snowglobe/icons/twelve/1.png".to_owned(), sprite_placed: false});
    #[rustfmt::skip] tile_vector.push(SpriteData {starting_location: vec3(answer_slot_vector[shuffle_slots[1]].x_pos, answer_slot_vector[shuffle_slots[1]].y_pos + STARTING_Y_POS_TILE_OFFSET, 1.0), 
    #[rustfmt::skip]                              correct_location:  vec3(answer_slot_vector[1].x_pos, answer_slot_vector[1].y_pos, 0.0),
    #[rustfmt::skip]                              filename: "snowglobe/icons/twelve/2.png".to_owned(), sprite_placed: false});
    #[rustfmt::skip] tile_vector.push(SpriteData {starting_location: vec3(answer_slot_vector[shuffle_slots[2]].x_pos, answer_slot_vector[shuffle_slots[2]].y_pos + STARTING_Y_POS_TILE_OFFSET, 1.0), 
    #[rustfmt::skip]                              correct_location:  vec3(answer_slot_vector[2].x_pos, answer_slot_vector[2].y_pos, 0.0),
    #[rustfmt::skip]                              filename: "snowglobe/icons/twelve/3.png".to_owned(), sprite_placed: false});
    #[rustfmt::skip] tile_vector.push(SpriteData {starting_location: vec3(answer_slot_vector[shuffle_slots[3]].x_pos, answer_slot_vector[shuffle_slots[3]].y_pos + STARTING_Y_POS_TILE_OFFSET, 1.0), 
    #[rustfmt::skip]                              correct_location:  vec3(answer_slot_vector[3].x_pos, answer_slot_vector[3].y_pos, 0.0),
    #[rustfmt::skip]                              filename: "snowglobe/icons/twelve/4.png".to_owned(), sprite_placed: false});
    #[rustfmt::skip] tile_vector.push(SpriteData {starting_location: vec3(answer_slot_vector[shuffle_slots[4]].x_pos, answer_slot_vector[shuffle_slots[4]].y_pos + STARTING_Y_POS_TILE_OFFSET, 1.0), 
    #[rustfmt::skip]                              correct_location:  vec3(answer_slot_vector[4].x_pos, answer_slot_vector[4].y_pos, 0.0),
    #[rustfmt::skip]                              filename: "snowglobe/icons/twelve/5.png".to_owned(), sprite_placed: false});
    #[rustfmt::skip] tile_vector.push(SpriteData {starting_location: vec3(answer_slot_vector[shuffle_slots[5]].x_pos, answer_slot_vector[shuffle_slots[5]].y_pos + STARTING_Y_POS_TILE_OFFSET, 1.0), 
    #[rustfmt::skip]                              correct_location:  vec3(answer_slot_vector[5].x_pos, answer_slot_vector[5].y_pos, 0.0),
    #[rustfmt::skip]                              filename: "snowglobe/icons/twelve/6.png".to_owned(), sprite_placed: false});
    #[rustfmt::skip] tile_vector.push(SpriteData {starting_location: vec3(answer_slot_vector[shuffle_slots[6]].x_pos, answer_slot_vector[shuffle_slots[6]].y_pos + STARTING_Y_POS_TILE_OFFSET, 1.0), 
    #[rustfmt::skip]                              correct_location:  vec3(answer_slot_vector[6].x_pos, answer_slot_vector[6].y_pos, 0.0),
    #[rustfmt::skip]                              filename: "snowglobe/icons/twelve/7.png".to_owned(), sprite_placed: false});
    #[rustfmt::skip] tile_vector.push(SpriteData {starting_location: vec3(answer_slot_vector[shuffle_slots[7]].x_pos, answer_slot_vector[shuffle_slots[7]].y_pos + STARTING_Y_POS_TILE_OFFSET, 1.0), 
    #[rustfmt::skip]                              correct_location:  vec3(answer_slot_vector[7].x_pos, answer_slot_vector[7].y_pos, 0.0),
    #[rustfmt::skip]                              filename: "snowglobe/icons/twelve/8.png".to_owned(), sprite_placed: false});
    #[rustfmt::skip] tile_vector.push(SpriteData {starting_location: vec3(answer_slot_vector[shuffle_slots[8]].x_pos, answer_slot_vector[shuffle_slots[8]].y_pos + STARTING_Y_POS_TILE_OFFSET, 1.0), 
    #[rustfmt::skip]                              correct_location:  vec3(answer_slot_vector[8].x_pos, answer_slot_vector[8].y_pos, 0.0),
    #[rustfmt::skip]                              filename: "snowglobe/icons/twelve/9.png".to_owned(), sprite_placed: false});
    #[rustfmt::skip] tile_vector.push(SpriteData {starting_location: vec3(answer_slot_vector[shuffle_slots[9]].x_pos, answer_slot_vector[shuffle_slots[9]].y_pos + STARTING_Y_POS_TILE_OFFSET, 1.0), 
    #[rustfmt::skip]                              correct_location:  vec3(answer_slot_vector[9].x_pos, answer_slot_vector[9].y_pos, 0.0),
    #[rustfmt::skip]                              filename: "snowglobe/icons/twelve/10.png".to_owned(), sprite_placed: false});
    #[rustfmt::skip] tile_vector.push(SpriteData {starting_location: vec3(answer_slot_vector[shuffle_slots[10]].x_pos, answer_slot_vector[shuffle_slots[10]].y_pos + STARTING_Y_POS_TILE_OFFSET, 1.0), 
    #[rustfmt::skip]                              correct_location:  vec3(answer_slot_vector[10].x_pos, answer_slot_vector[10].y_pos, 0.0),
    #[rustfmt::skip]                              filename: "snowglobe/icons/twelve/11.png".to_owned(), sprite_placed: false});
    #[rustfmt::skip] tile_vector.push(SpriteData {starting_location: vec3(answer_slot_vector[shuffle_slots[11]].x_pos, answer_slot_vector[shuffle_slots[11]].y_pos + STARTING_Y_POS_TILE_OFFSET, 1.0), 
    #[rustfmt::skip]                              correct_location:  vec3(answer_slot_vector[11].x_pos, answer_slot_vector[11].y_pos, 0.0),
    #[rustfmt::skip]                              filename: "snowglobe/icons/twelve/12.png".to_owned(), sprite_placed: false});

    // generate the answer slots
    for answer_slot in answer_slot_vector.iter() {
        commands
            .spawn((
                SpriteBundle {
                    texture: asset_server.load("snowglobe/icons/element background.png"),
                    sprite: Sprite {
                        custom_size: Some(Vec2 {
                            x: ICON_SIZE_WIDTH,
                            y: ICON_SIZE_HEIGHT,
                        }),
                        color: Color::rgb(0.8, 1.0, 0.8),
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
                    text: answer_slot.text.clone(),
                },
            ))
            .with_children(|parent| {
                parent.spawn(Text2dBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            answer_slot.text.clone(),
                            TextStyle {
                                font: asset_server.load("snowglobe/fonts/MTF Dear Santa.ttf"),
                                font_size: FONT_SIZE,
                                color: Color::BLACK,
                            },
                        )],
                        alignment: TextAlignment::Center,
                        linebreak_behavior: BreakLineOn::NoWrap,
                    },
                    // ensure the text is drawn on top of the box
                    transform: Transform::from_translation(Vec3::Z),
                    ..default()
                });
            });
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
}

fn begin_twelve_days_puzzle(
    mut answer_slot_visibility_query: Query<(
        &mut Visibility,
        With<AnswerSlot>,
        Without<SpriteData>,
    )>,
    mut sprite_visibility_query: Query<(&mut Visibility, With<SpriteData>, Without<AnswerSlot>)>,
) {
    for mut answer_slot_visibility in &mut answer_slot_visibility_query.iter_mut() {
        *answer_slot_visibility.0 = Visibility::Visible;
    }

    for mut sprite_visibility in &mut sprite_visibility_query.iter_mut() {
        *sprite_visibility.0 = Visibility::Visible;
    }
}

fn play_twelve_days_puzzle(
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

                    if sprite_to_answer_vector.length() < 50. {
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

        //println!("Sprite position old: {:?}", sprite_pos.translation);
        sprite_pos.translation.x = sprite_selection_state.cursor_pos.x + sprite.1.x;
        sprite_pos.translation.y = -(sprite_selection_state.cursor_pos.y + sprite.1.y);
        //println!("Sprite position new: {:?}", sprite_pos.translation);
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

        if successful_sprite_count == 12 && !success_found {
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
) {
    for answer_slot in answer_slot_query.iter() {
        commands.entity(answer_slot).despawn_recursive();
    }
    for sprite in sprite_query.iter() {
        commands.entity(sprite).despawn(); // no children
    }

    commands.remove_resource::<SpriteSelectionState>();
}
