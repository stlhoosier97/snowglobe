use bevy::prelude::*;

use crate::timer_bar::TimerResource;
use crate::GameState;
use crate::PuzzleState;
use crate::Score;
use crate::ScoreChange;
use crate::SoundEvent;
use crate::SoundsEnum;

const ICON_SIZE: f32 = 150.;

#[derive(Component)]
struct SpriteData {
    correct_sprite: bool,
    sprite_placed: bool,
    starting_location: Vec3,
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
            sprite: None, // to prevent cursor position from being zero and selecting an icon(ugh)
        }
    }
}

pub struct TurkeyMistletoePlugin;

impl Plugin for TurkeyMistletoePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpriteSelectionState>()
            .add_systems(OnEnter(GameState::PuzzleTurkeyMistletoe), setup)
            .add_systems(
                OnEnter(PuzzleState::InProgress),
                begin_turkey_mistletoe_puzzle,
            )
            .add_systems(
                Update,
                play_turkey_mistletoe_puzzle.run_if(in_state(GameState::PuzzleTurkeyMistletoe)),
            )
            .add_systems(
                Update,
                check_for_puzzle_completion.run_if(in_state(PuzzleState::InProgress)),
            )
            .add_systems(OnExit(GameState::PuzzleTurkeyMistletoe), cleanup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("snowglobe/icons/wreath.png"),
            sprite: Sprite {
                custom_size: Some(Vec2 {
                    x: ICON_SIZE,
                    y: ICON_SIZE,
                }),
                ..default()
            },
            visibility: Visibility::Hidden,
            transform: Transform {
                translation: Vec3::new(-600.0, 0.0, 0.0),
                ..default()
            },

            ..default()
        },
        SpriteData {
            sprite_placed: false,
            correct_sprite: false,
            starting_location: Vec3::new(-600.0, 0.0, 0.0),
        },
    ));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("snowglobe/icons/turkey.png"),
            sprite: Sprite {
                custom_size: Some(Vec2 {
                    x: ICON_SIZE,
                    y: ICON_SIZE,
                }),
                ..default()
            },
            visibility: Visibility::Hidden,
            transform: Transform {
                translation: Vec3::new(-400.0, 0.0, 0.0),
                ..default()
            },

            ..default()
        },
        SpriteData {
            sprite_placed: false,
            correct_sprite: true,
            starting_location: Vec3::new(-400.0, 0.0, 0.0),
        },
    ));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("snowglobe/icons/elf.png"),
            sprite: Sprite {
                custom_size: Some(Vec2 {
                    x: ICON_SIZE,
                    y: ICON_SIZE,
                }),
                ..default()
            },
            visibility: Visibility::Hidden,
            transform: Transform {
                translation: Vec3::new(-200.0, 0.0, 0.0),
                ..default()
            },

            ..default()
        },
        SpriteData {
            sprite_placed: false,
            correct_sprite: false,
            starting_location: Vec3::new(-200.0, 0.0, 0.0),
        },
    ));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("snowglobe/icons/ornament.png"),
            sprite: Sprite {
                custom_size: Some(Vec2 {
                    x: ICON_SIZE,
                    y: ICON_SIZE,
                }),
                ..default()
            },
            visibility: Visibility::Hidden,
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..default()
            },

            ..default()
        },
        SpriteData {
            sprite_placed: false,
            correct_sprite: false,
            starting_location: Vec3::new(0.0, 0.0, 0.0),
        },
    ));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("snowglobe/icons/gift.png"),
            sprite: Sprite {
                custom_size: Some(Vec2 {
                    x: ICON_SIZE,
                    y: ICON_SIZE,
                }),
                ..default()
            },
            visibility: Visibility::Hidden,
            transform: Transform {
                translation: Vec3::new(200.0, 0.0, 0.0),
                ..default()
            },

            ..default()
        },
        SpriteData {
            sprite_placed: false,
            correct_sprite: false,
            starting_location: Vec3::new(200.0, 0.0, 0.0),
        },
    ));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("snowglobe/icons/mistletoe.png"),
            sprite: Sprite {
                custom_size: Some(Vec2 {
                    x: ICON_SIZE,
                    y: ICON_SIZE,
                }),
                ..default()
            },
            visibility: Visibility::Hidden,
            transform: Transform {
                translation: Vec3::new(400.0, 0.0, 0.0),
                ..default()
            },

            ..default()
        },
        SpriteData {
            sprite_placed: false,
            correct_sprite: true,
            starting_location: Vec3::new(400.0, 0.0, 0.0),
        },
    ));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("snowglobe/icons/sleigh.png"),
            sprite: Sprite {
                custom_size: Some(Vec2 {
                    x: ICON_SIZE,
                    y: ICON_SIZE,
                }),
                ..default()
            },
            visibility: Visibility::Hidden,
            transform: Transform {
                translation: Vec3::new(600.0, 0.0, 0.0),
                ..default()
            },

            ..default()
        },
        SpriteData {
            sprite_placed: false,
            correct_sprite: false,
            starting_location: Vec3::new(600.0, 0.0, 0.0),
        },
    ));
}

fn begin_turkey_mistletoe_puzzle(
    mut sprite_visibility_query: Query<(&mut Visibility, With<SpriteData>)>,
) {
    for mut sprite_visibility in &mut sprite_visibility_query.iter_mut() {
        *sprite_visibility.0 = Visibility::Visible;
    }
}

fn play_turkey_mistletoe_puzzle(
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
            //let sprite = sprite_selection_state.sprite.unwrap();
            //let sprite_pos = transforms.get_mut(sprite.0).unwrap();
            //println!("sprite position is: ({}, {})", sprite_pos.translation.x, sprite_pos.translation.y);

            for (entity, mut sprite) in sprites.iter_mut() {
                let sprite_pos = transforms.get_mut(entity).unwrap().translation;

                let vector_to_sprite = Vec3::new(
                    sprite_pos.x - sprite_selection_state.cursor_pos.x,
                    -sprite_pos.y - sprite_selection_state.cursor_pos.y,
                    0.0,
                );

                if vector_to_sprite.length() < 100. {
                    if sprite.correct_sprite {
                        sound_event_writer.send(SoundEvent(SoundsEnum::Success));
                        sprite.sprite_placed = true;
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
    button_query: Query<Entity, With<Button>>,
    sprite_query: Query<Entity, With<SpriteData>>,
) {
    for button in button_query.iter() {
        commands.entity(button).despawn();
    }

    for sprite in sprite_query.iter() {
        commands.entity(sprite).despawn();
    }
}
