use crate::timer_bar::TimerResource;
use crate::BackgroundImage;
use crate::ButtonColors;
use crate::GameState;
use crate::PuzzleState;
use crate::SoundEvent;
use crate::SoundsEnum;
use bevy::prelude::*;

#[derive(Component)]
struct StageInstructions;

#[derive(Component)]
struct StageAnswers;

const STAGE_TEXT_Z_OFFSET: f32 = 10.0;
pub struct StageCommonPlugin;

impl Plugin for StageCommonPlugin {
    fn build(&self, app: &mut App) {
        // setup stage
        app.add_systems(
            OnEnter(GameState::PuzzleTurkeyMistletoe),
            setup_stage_common,
        )
        .add_systems(OnEnter(GameState::PuzzlePeriodicTable), setup_stage_common)
        .add_systems(OnEnter(GameState::PuzzleTwelveDays), setup_stage_common)
        .add_systems(OnEnter(GameState::PuzzleReindeer), setup_stage_common)
        .add_systems(OnEnter(GameState::PuzzleWaltz), setup_stage_common)
        .add_systems(OnEnter(GameState::PuzzleGrocery), setup_stage_common)
        .add_systems(OnEnter(GameState::PuzzleFamilyTree), setup_stage_common)
        .add_systems(OnEnter(GameState::PuzzleCranberries), setup_stage_common)
        // start song
        .add_systems(OnEnter(PuzzleState::InProgress), start_song)
        // update
        .add_systems(
            Update,
            update_stage_common.run_if(in_state(GameState::PuzzleTurkeyMistletoe)),
        )
        .add_systems(
            Update,
            update_stage_common.run_if(in_state(GameState::PuzzlePeriodicTable)),
        )
        .add_systems(
            Update,
            update_stage_common.run_if(in_state(GameState::PuzzleTwelveDays)),
        )
        .add_systems(
            Update,
            update_stage_common.run_if(in_state(GameState::PuzzleReindeer)),
        )
        .add_systems(
            Update,
            update_stage_common.run_if(in_state(GameState::PuzzleWaltz)),
        )
        .add_systems(
            Update,
            update_stage_common.run_if(in_state(GameState::PuzzleGrocery)),
        )
        .add_systems(
            Update,
            update_stage_common.run_if(in_state(GameState::PuzzleFamilyTree)),
        )
        .add_systems(
            Update,
            update_stage_common.run_if(in_state(GameState::PuzzleCranberries)),
        )
        // puzzle complete
        .add_systems(OnEnter(PuzzleState::Complete), puzzle_state_now_complete)
        // cleanup stage
        .add_systems(
            OnExit(GameState::PuzzleTurkeyMistletoe),
            cleanup_stage_common,
        )
        .add_systems(OnExit(GameState::PuzzlePeriodicTable), cleanup_stage_common)
        .add_systems(OnExit(GameState::PuzzleReindeer), cleanup_stage_common)
        .add_systems(OnExit(GameState::PuzzleWaltz), cleanup_stage_common)
        .add_systems(OnExit(GameState::PuzzleTwelveDays), cleanup_stage_common)
        .add_systems(OnExit(GameState::PuzzleGrocery), cleanup_stage_common)
        .add_systems(OnExit(GameState::PuzzleFamilyTree), cleanup_stage_common)
        .add_systems(OnExit(GameState::PuzzleCranberries), cleanup_stage_common)
        .add_systems(Update, skip_puzzle); // @rch: temporary to allow easy skips of puzzles for testing
    }
}

fn start_song(
    mut sound_event_writer: EventWriter<SoundEvent>,
    current_game_state: Res<State<GameState>>,
) {
    match current_game_state.get() {
        GameState::TitleScreen | GameState::Instructions | GameState::ShowScore => (),
        GameState::PuzzleTurkeyMistletoe => {
            sound_event_writer.send(SoundEvent(SoundsEnum::TurkeyMistletoeSong))
        }
        GameState::PuzzlePeriodicTable => {
            sound_event_writer.send(SoundEvent(SoundsEnum::PeriodicTableSong))
        }
        GameState::PuzzleTwelveDays => {
            sound_event_writer.send(SoundEvent(SoundsEnum::TwelveDaysSong))
        }
        GameState::PuzzleWaltz => sound_event_writer.send(SoundEvent(SoundsEnum::WaltzSong)),
        GameState::PuzzleReindeer => sound_event_writer.send(SoundEvent(SoundsEnum::ReindeerSong)),
        GameState::PuzzleGrocery => sound_event_writer.send(SoundEvent(SoundsEnum::GrocerySong)),
        GameState::PuzzleCranberries => {
            sound_event_writer.send(SoundEvent(SoundsEnum::CranberriesSong))
        }
        GameState::PuzzleFamilyTree => {
            sound_event_writer.send(SoundEvent(SoundsEnum::FamilyTreeSong))
        }
    }
}

fn setup_stage_common(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    button_colors: Res<ButtonColors>,
    current_game_state: Res<State<GameState>>,
) {
    commands
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Px(400.0),
                height: Val::Px(50.0),
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                left: Val::Px(0.0),
                right: Val::Px(200.0),
                top: Val::Px(325.0),
                bottom: Val::Px(400.0),
                ..Default::default()
            },
            background_color: button_colors.normal.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "click to begin puzzle",
                    TextStyle {
                        font: asset_server.load("snowglobe/fonts/MTF Dear Santa.ttf"),
                        font_size: 60.0,
                        color: Color::WHITE,
                    },
                )
                .with_text_alignment(TextAlignment::Center),
            );
        });

    let instructions_filename: &str = match current_game_state.get() {
        GameState::PuzzleTurkeyMistletoe => "snowglobe/text/instructions - stage 1.png",
        GameState::PuzzlePeriodicTable => "snowglobe/text/instructions - stage 2.png",
        GameState::PuzzleTwelveDays | GameState::PuzzleReindeer => {
            "snowglobe/text/instructions - stage 3.png"
        }
        GameState::PuzzleWaltz => "snowglobe/text/instructions - stage 5.png",
        GameState::PuzzleGrocery => "snowglobe/text/instructions - stage 6.png",
        GameState::PuzzleFamilyTree => "snowglobe/text/instructions - stage 7.png",
        GameState::PuzzleCranberries => "snowglobe/text/instructions - stage 8.png",
        GameState::TitleScreen | GameState::Instructions | GameState::ShowScore => "",
    };

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(instructions_filename),
            sprite: Sprite {
                custom_size: Some(Vec2 { x: 1600., y: 800. }),
                ..default()
            },
            visibility: Visibility::Hidden,
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, STAGE_TEXT_Z_OFFSET),
                ..default()
            },

            ..default()
        },
        StageInstructions,
    ));

    let answers_filename: &str = match current_game_state.get() {
        GameState::PuzzleTurkeyMistletoe => "snowglobe/text/answers - stage 1.png",
        GameState::PuzzlePeriodicTable => "snowglobe/text/answers - stage 2.png",
        GameState::PuzzleTwelveDays | GameState::PuzzleReindeer => {
            "snowglobe/text/instructions - stage 3.png"
        } // no answers provided
        GameState::PuzzleWaltz => "snowglobe/text/answers - stage 5.png",
        GameState::PuzzleGrocery => "snowglobe/text/answers - stage 6.png",
        GameState::PuzzleFamilyTree => "snowglobe/text/answers - stage 7.png",
        GameState::PuzzleCranberries => "snowglobe/text/answers - stage 8.png",
        GameState::TitleScreen | GameState::Instructions | GameState::ShowScore => "",
    };

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(answers_filename),
            sprite: Sprite {
                custom_size: Some(Vec2 { x: 1600., y: 800. }),
                ..default()
            },
            visibility: Visibility::Hidden,
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, STAGE_TEXT_Z_OFFSET),
                ..default()
            },

            ..default()
        },
        StageAnswers,
    ));

    let background_image: &str = match current_game_state.get() {
        GameState::PuzzleTurkeyMistletoe => "snowglobe/backgrounds/fireplace.png",
        GameState::PuzzlePeriodicTable => "snowglobe/backgrounds/chemistry.png",
        GameState::PuzzleTwelveDays => "snowglobe/backgrounds/twelve.png",
        GameState::PuzzleReindeer => "snowglobe/backgrounds/reindeer.png",
        GameState::PuzzleWaltz => "snowglobe/backgrounds/orchestra.png",
        GameState::PuzzleGrocery => "snowglobe/backgrounds/grocery.png",
        GameState::PuzzleFamilyTree => "snowglobe/backgrounds/family tree.png",
        GameState::PuzzleCranberries => "snowglobe/backgrounds/cranberries.png",
        GameState::TitleScreen | GameState::Instructions | GameState::ShowScore => "",
    };

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.2, 0.2, 0.2, 1.0),
                custom_size: Some(Vec2 { x: 1600., y: 800. }),
                ..default()
            },
            texture: asset_server.load(background_image),
            visibility: Visibility::Visible,
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, -10.0),
                ..default()
            },

            ..default()
        },
        BackgroundImage,
    ));
}

fn update_stage_common(
    mut instructions_visibility_query: Query<(
        &mut Visibility,
        With<StageInstructions>,
        Without<Button>,
        Without<StageAnswers>,
    )>,
    mut answers_visibility_query: Query<(
        &mut Visibility,
        With<StageAnswers>,
        Without<Button>,
        Without<StageInstructions>,
    )>,
    timer_resource: Res<TimerResource>,

    current_puzzle_state: ResMut<State<PuzzleState>>,
    mut next_puzzle_state: ResMut<NextState<PuzzleState>>,
    button_colors: Res<ButtonColors>,
    mut next_game_state: ResMut<NextState<GameState>>,
    current_game_state: Res<State<GameState>>,
    mut button_visibility_query: Query<(&mut Visibility, With<Button>, Without<StageInstructions>)>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => match current_puzzle_state.get().clone() {
                PuzzleState::GetReady => {
                    // if the button is pressed in the GetReady state, make it invisible
                    // and move the puzzle state to in progress
                    next_puzzle_state.set(PuzzleState::InProgress);
                    let mut button_visibility = button_visibility_query.single_mut();
                    *button_visibility.0 = Visibility::Hidden;

                    let mut instructions_visibility = instructions_visibility_query.single_mut();
                    *instructions_visibility.0 = Visibility::Visible;
                }
                PuzzleState::InProgress => {}
                PuzzleState::Complete => {
                    // if the button is pressed in the Complete state, move to the next puzzle
                    next_puzzle_state.set(PuzzleState::GetReady);

                    match current_game_state.get() {
                        GameState::TitleScreen | GameState::Instructions | GameState::ShowScore => {
                            ()
                        }
                        #[rustfmt::skip] GameState::PuzzleTurkeyMistletoe => next_game_state.set(GameState::PuzzlePeriodicTable),
                        #[rustfmt::skip] GameState::PuzzlePeriodicTable   => next_game_state.set(GameState::PuzzleWaltz),
                        #[rustfmt::skip] GameState::PuzzleWaltz           => next_game_state.set(GameState::PuzzleReindeer),
                        #[rustfmt::skip] GameState::PuzzleReindeer        => next_game_state.set(GameState::PuzzleGrocery),
                        #[rustfmt::skip] GameState::PuzzleGrocery         => next_game_state.set(GameState::PuzzleTwelveDays),
                        #[rustfmt::skip] GameState::PuzzleTwelveDays      => next_game_state.set(GameState::PuzzleFamilyTree),
                        #[rustfmt::skip] GameState::PuzzleFamilyTree      => next_game_state.set(GameState::PuzzleCranberries),
                        #[rustfmt::skip] GameState::PuzzleCranberries     => next_game_state.set(GameState::ShowScore),
                    }
                }
            },
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
            Interaction::None => {
                *color = button_colors.normal.into();
            }
        }
    }

    let answer_time = match current_game_state.get() {
        GameState::TitleScreen => 0,
        GameState::Instructions => 0,
        GameState::ShowScore => 0,
        GameState::PuzzleTurkeyMistletoe => 3,
        GameState::PuzzlePeriodicTable => 3,
        GameState::PuzzleTwelveDays => 3,
        GameState::PuzzleReindeer => 3,
        GameState::PuzzleWaltz => 3,
        GameState::PuzzleGrocery => 3,
        GameState::PuzzleFamilyTree => 5,
        GameState::PuzzleCranberries => 4,
    };

    if (timer_resource.time_remaining.duration() - timer_resource.time_remaining.elapsed())
        .as_secs()
        < answer_time
    {
        let mut instructions_visibility = instructions_visibility_query.single_mut();
        *instructions_visibility.0 = Visibility::Hidden;

        let mut answers_visibility = answers_visibility_query.single_mut();
        *answers_visibility.0 = Visibility::Visible;
    }
}

fn puzzle_state_now_complete(
    mut visibility_query: Query<(&mut Visibility, &Button)>,

    mut non_interaction_query: Query<&Children, With<Button>>,
    mut button_text_query: Query<&mut Text>,
    current_game_state: Res<State<GameState>>,
) {
    // make the button visible again
    let mut button_visibility = visibility_query.single_mut();
    *button_visibility.0 = Visibility::Visible;

    for children in &mut non_interaction_query {
        let mut text = button_text_query.get_mut(children[0]).unwrap();
        match current_game_state.get() {
            GameState::PuzzleCranberries => text.sections[0].value = "show final score".to_string(),
            _ => text.sections[0].value = "go to next puzzle!".to_string(),
        }
    }
}

fn cleanup_stage_common(
    mut commands: Commands,
    button_query: Query<Entity, With<Button>>,
    background_image_query: Query<Entity, With<BackgroundImage>>,
    stage_instructions_query: Query<Entity, With<StageInstructions>>,
    stage_answers_query: Query<Entity, With<StageAnswers>>,
) {
    for button in button_query.iter() {
        commands.entity(button).despawn();
    }

    for background_image in background_image_query.iter() {
        commands.entity(background_image).despawn();
    }

    for stage_instructions in stage_instructions_query.iter() {
        commands.entity(stage_instructions).despawn();
    }

    for stage_answers in stage_answers_query.iter() {
        commands.entity(stage_answers).despawn();
    }
}

// @rch: only for development!
fn skip_puzzle(
    current_puzzle_state: Res<State<PuzzleState>>,
    mut next_puzzle_state: ResMut<NextState<PuzzleState>>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::S) {
        if *current_puzzle_state.get() == PuzzleState::InProgress {
            next_puzzle_state.set(PuzzleState::Complete)
        }
    }
}
