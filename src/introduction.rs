use crate::BackgroundImage;
use crate::GameState;
use crate::SoundEvent;
use crate::SoundsEnum;
use bevy::prelude::*;

#[derive(Component)]
struct TitleText;

#[derive(Resource)]
struct ButtonColors {
    normal: Color,
    hovered: Color,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::rgb(131. / 255., 11. / 255., 20. / 255.),
            hovered: Color::rgb(151. / 255., 31. / 255., 40. / 255.),
        }
    }
}

pub struct IntroductionPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for IntroductionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ButtonColors>()
            .add_systems(OnEnter(GameState::TitleScreen), setup_title_screen)
            .add_systems(
                Update,
                click_play_button.run_if(in_state(GameState::TitleScreen)),
            )
            .add_systems(OnExit(GameState::TitleScreen), cleanup_title_screen);
    }
}

fn setup_title_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    button_colors: Res<ButtonColors>,
    mut sound_event_writer: EventWriter<SoundEvent>,
) {
    commands
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Px(300.0),
                height: Val::Px(50.0),
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                left: Val::Px(00.0),
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
                    "click here to play",
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
            texture: asset_server.load("snowglobe/text/title.png"),
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
        TitleText,
    ));

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

    sound_event_writer.send(SoundEvent(SoundsEnum::TitleScreen));
}

fn click_play_button(
    button_colors: Res<ButtonColors>,
    mut state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                state.set(GameState::Instructions);
            }
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
            Interaction::None => {
                *color = button_colors.normal.into();
            }
        }
    }
}

fn cleanup_title_screen(
    mut commands: Commands,
    button: Query<Entity, With<Button>>,
    text: Query<Entity, With<TitleText>>,
) {
    commands.entity(button.single()).despawn_recursive();
    commands.entity(text.single()).despawn_recursive();
    // don't despawn the background image so it shows up in the instructions page (the next game state)
}
