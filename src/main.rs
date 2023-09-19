use bevy::{prelude::*, window::PresentMode};

mod audio;
mod cranberries;
mod family_tree;
mod grocery;
mod instructions;
mod introduction;
mod periodic_table;
mod reindeer;
mod score;
mod show_score;
mod snowflakes;
mod stage_common;
mod timer_bar;
mod turkey_mistletoe;
mod twelve_days;
mod waltz;

use crate::audio::AudioPlugin;
use crate::cranberries::CranberriesPlugin;
use crate::family_tree::FamilyTreePlugin;
use crate::grocery::GroceryPlugin;
use crate::instructions::InstructionsPlugin;
use crate::introduction::IntroductionPlugin;
use crate::periodic_table::PeriodicTablePlugin;
use crate::reindeer::ReindeerPlugin;
use crate::score::ScorePlugin;
use crate::show_score::ShowScorePlugin;
use crate::snowflakes::SnowflakesPlugin;
use crate::stage_common::StageCommonPlugin;
use crate::timer_bar::TimerBarPlugin;
use crate::turkey_mistletoe::TurkeyMistletoePlugin;
use crate::twelve_days::TwelveDaysPlugin;
use crate::waltz::WaltzPlugin;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    TitleScreen,
    Instructions,
    PuzzleTurkeyMistletoe,
    PuzzlePeriodicTable,
    PuzzleTwelveDays,
    PuzzleReindeer,
    PuzzleWaltz,
    PuzzleGrocery,
    PuzzleFamilyTree,
    PuzzleCranberries,
    ShowScore,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum PuzzleState {
    #[default]
    GetReady,
    InProgress,
    Complete,
}

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

#[derive(Component)]
struct BackgroundImage;

struct ScoreChange {
    delta_score: f32,
    initial_score: f32,
}

#[derive(Resource, Default)]
struct Score {
    current_score: f32,
    mistakes: u32,
    score_change_vector: Vec<ScoreChange>,
}

#[derive(Component, PartialEq, Debug)]
pub enum SoundsEnum {
    TitleScreen = 0,
    HoHoHo = 1,
    Success = 2,
    Failure = 3,
    TurkeyMistletoeSong = 4,
    PeriodicTableSong = 5,
    TwelveDaysSong = 6,
    ReindeerSong = 7,
    WaltzSong = 8,
    GrocerySong = 9,
    FamilyTreeSong = 10,
    CranberriesSong = 11,
    MerryChristmas = 12,
}

// events
#[derive(Event)]
struct SoundEvent(SoundsEnum);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "the christmas snowglobe 3".into(),
                resolution: (1600., 800.).into(),
                present_mode: PresentMode::AutoVsync,
                // Tells wasm to resize the window according to the available canvas
                fit_canvas_to_parent: true,
                // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .init_resource::<ButtonColors>()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Score {
            current_score: 0.0,
            mistakes: 0,
            score_change_vector: Vec::new(),
        })
        .add_state::<GameState>()
        .add_state::<PuzzleState>()
        .add_plugins((
            IntroductionPlugin,
            SnowflakesPlugin,
            ScorePlugin,
            StageCommonPlugin,
            TurkeyMistletoePlugin,
            TwelveDaysPlugin,
            TimerBarPlugin,
            InstructionsPlugin,
            AudioPlugin,
            PeriodicTablePlugin,
            ReindeerPlugin,
            WaltzPlugin,
            GroceryPlugin,
            FamilyTreePlugin,
            CranberriesPlugin,
        ))
        .add_plugins(ShowScorePlugin) // not sure why this needs to be separate
        .add_systems(Startup, setup)
        .add_event::<SoundEvent>()
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
