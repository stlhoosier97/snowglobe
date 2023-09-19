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

#[derive(Component)]
struct FamilyTreeBackground;

pub struct FamilyTreePlugin;

impl Plugin for FamilyTreePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpriteSelectionState>()
            .add_systems(OnEnter(GameState::PuzzleFamilyTree), setup)
            .add_systems(OnEnter(PuzzleState::InProgress), begin_family_tree_puzzle)
            .add_systems(
                Update,
                play_family_tree_puzzle.run_if(in_state(GameState::PuzzleFamilyTree)),
            )
            .add_systems(
                Update,
                check_for_puzzle_completion.run_if(in_state(PuzzleState::InProgress)),
            )
            .add_systems(OnExit(GameState::PuzzleFamilyTree), cleanup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // answer slots
    let mut answer_slot_vector: Vec<AnswerSlot> = Vec::new();

    #[rustfmt::skip] answer_slot_vector.push(AnswerSlot {x_pos: 285., y_pos:   -90.}); // aunt
    #[rustfmt::skip] answer_slot_vector.push(AnswerSlot {x_pos: -161., y_pos: -250.}); // sister
    #[rustfmt::skip] answer_slot_vector.push(AnswerSlot {x_pos: -288., y_pos: -250.}); // brother

    // sprites to move
    let mut tile_vector: Vec<SpriteData> = Vec::new();

    #[rustfmt::skip] tile_vector.push(SpriteData {starting_location: vec3(-550.0, -300.0, 1.0),
    #[rustfmt::skip]                              correct_location:  vec3(answer_slot_vector[2].x_pos, answer_slot_vector[2].y_pos, 1.0),
    #[rustfmt::skip]                              filename: "snowglobe/icons/family/at the door.png".to_owned(), sprite_placed: false});
    #[rustfmt::skip] tile_vector.push(SpriteData {starting_location: vec3(-550.0, -100.0, 1.0),
    #[rustfmt::skip]                              correct_location:  vec3(answer_slot_vector[1].x_pos, answer_slot_vector[1].y_pos, 1.0),
    #[rustfmt::skip]                              filename: "snowglobe/icons/family/suspicious.png".to_owned(), sprite_placed: false});
    #[rustfmt::skip] tile_vector.push(SpriteData {starting_location: vec3(-550.0, 100.0, 1.0),
    #[rustfmt::skip]                              correct_location:  vec3(answer_slot_vector[0].x_pos, answer_slot_vector[0].y_pos, 1.0),
    #[rustfmt::skip]                              filename: "snowglobe/icons/family/vicious.png".to_owned(), sprite_placed: false});

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
                    color: Color::rgba(1.0, 1.0, 1.0, 0.35),
                    ..default()
                },
                visibility: Visibility::Hidden,

                transform: Transform {
                    translation: Vec3::new(answer_slot.x_pos, answer_slot.y_pos, 2.0),
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
                    color: Color::WHITE,
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

    // generate the family tree
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                //color: Color::rgb(0.8, 0.9, 0.9),
                color: Color::WHITE,
                custom_size: Some(Vec2 { x: 800., y: 500. }),
                ..default()
            },
            texture: asset_server.load("snowglobe/icons/family tree.png"),
            visibility: Visibility::Hidden,
            transform: Transform {
                translation: Vec3::new(0.0, -100.0, 0.0),
                ..default()
            },

            ..default()
        },
        FamilyTreeBackground,
    ));
}

fn begin_family_tree_puzzle(
    mut answer_slot_visibility_query: Query<(
        &mut Visibility,
        With<AnswerSlot>,
        Without<SpriteData>,
        Without<FamilyTreeBackground>,
    )>,
    mut sprite_visibility_query: Query<(
        &mut Visibility,
        With<SpriteData>,
        Without<AnswerSlot>,
        Without<FamilyTreeBackground>,
    )>,
    mut family_tree_visibility_query: Query<(&mut Visibility, With<FamilyTreeBackground>)>,
) {
    for mut answer_slot_visibility in &mut answer_slot_visibility_query.iter_mut() {
        *answer_slot_visibility.0 = Visibility::Visible;
    }

    for mut sprite_visibility in &mut sprite_visibility_query.iter_mut() {
        *sprite_visibility.0 = Visibility::Visible;
    }

    for mut family_tree_visibility in &mut family_tree_visibility_query.iter_mut() {
        *family_tree_visibility.0 = Visibility::Visible;
    }
}

fn play_family_tree_puzzle(
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

        if successful_sprite_count == 3 && !success_found {
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
    family_tree_background_query: Query<Entity, With<FamilyTreeBackground>>,
    answer_slot_query: Query<Entity, With<AnswerSlot>>,
    sprite_query: Query<Entity, With<SpriteData>>,
) {
    for family_tree_background in family_tree_background_query.iter() {
        commands.entity(family_tree_background).despawn();
    }
    for answer_slot in answer_slot_query.iter() {
        commands.entity(answer_slot).despawn_recursive();
    }
    for sprite in sprite_query.iter() {
        commands.entity(sprite).despawn(); // no children
    }

    commands.remove_resource::<SpriteSelectionState>();
}
