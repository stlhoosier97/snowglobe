use crate::BackgroundImage;
use crate::ButtonColors;
use crate::GameState;
use bevy::prelude::*;

#[derive(Component)]
struct InstructionsText {
    screen_state_to_display: InstructionScreenEnum,
}

#[derive(Resource, Default, PartialEq)]
enum InstructionScreenEnum {
    #[default]
    ScreenOne,
    ScreenTwo,
    ScreenThree,
}
#[derive(Resource, Default)]
struct InstructionScreenState {
    screen_state: InstructionScreenEnum,
}

pub struct InstructionsPlugin;

impl Plugin for InstructionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InstructionScreenState>()
            .add_systems(OnEnter(GameState::Instructions), setup_instructions)
            .add_systems(
                Update,
                update_instructions.run_if(in_state(GameState::Instructions)),
            )
            .add_systems(OnExit(GameState::Instructions), cleanup_title_screen);
    }
}

fn setup_instructions(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    button_colors: Res<ButtonColors>,
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
                    "continue",
                    TextStyle {
                        font: asset_server.load("snowglobe/fonts/MTF Dear Santa.ttf"),
                        font_size: 60.0,
                        color: Color::WHITE,
                    },
                )
                .with_text_alignment(TextAlignment::Center),
            );
        });

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("snowglobe/text/instructions 1.png"),
            sprite: Sprite {
                custom_size: Some(Vec2 { x: 1600., y: 800. }),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..default()
            },

            ..default()
        },
        InstructionsText {
            screen_state_to_display: InstructionScreenEnum::ScreenOne,
        },
    ));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("snowglobe/text/instructions 2.png"),
            sprite: Sprite {
                custom_size: Some(Vec2 { x: 1600., y: 800. }),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..default()
            },

            ..default()
        },
        InstructionsText {
            screen_state_to_display: InstructionScreenEnum::ScreenTwo,
        },
    ));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("snowglobe/text/instructions 3.png"),
            sprite: Sprite {
                custom_size: Some(Vec2 { x: 1600., y: 800. }),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..default()
            },

            ..default()
        },
        InstructionsText {
            screen_state_to_display: InstructionScreenEnum::ScreenThree,
        },
    ));
}

fn update_instructions(
    button_colors: Res<ButtonColors>,
    mut state: ResMut<NextState<GameState>>,
    mut instruction_screen: ResMut<InstructionScreenState>,
    mut button_text_query: Query<&mut Text, Without<InstructionsText>>,
    mut instructions_text_query: Query<(&mut Visibility, &InstructionsText)>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, children) in &mut interaction_query {
        let mut text = button_text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => match instruction_screen.screen_state {
                InstructionScreenEnum::ScreenOne => {
                    instruction_screen.screen_state = InstructionScreenEnum::ScreenTwo;
                    text.sections[0].value = "continue".to_string();
                }
                InstructionScreenEnum::ScreenTwo => {
                    instruction_screen.screen_state = InstructionScreenEnum::ScreenThree;
                    text.sections[0].value = "start your first puzzle!".to_string();
                }

                InstructionScreenEnum::ScreenThree => {
                    state.set(GameState::PuzzleTurkeyMistletoe);
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

    for (mut instruction_text_visibility, component) in instructions_text_query.iter_mut() {
        if component.screen_state_to_display == instruction_screen.screen_state {
            *instruction_text_visibility = Visibility::Visible;
        } else {
            *instruction_text_visibility = Visibility::Hidden;
        }
    }
}

fn cleanup_title_screen(
    mut commands: Commands,
    button: Query<Entity, With<Button>>,
    text: Query<Entity, With<InstructionsText>>,
    background_image: Query<Entity, With<BackgroundImage>>,
) {
    commands.entity(button.single()).despawn_recursive();

    // not sure why despawn_recursive didn't work here - doing it this way despawns 3
    // text entities as expected
    for text_entity in text.iter() {
        commands.entity(text_entity).despawn();
    }
    commands
        .entity(background_image.single())
        .despawn_recursive();
}
