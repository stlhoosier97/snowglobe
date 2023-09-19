use bevy::prelude::*;

use crate::GameState;
use crate::PuzzleState;
use crate::SoundEvent;
use crate::SoundsEnum;

#[derive(Resource)]
struct SoundResource {
    handle_vector: Vec<Handle<AudioSource>>,
}

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::TitleScreen), load_audio)
            .add_systems(OnEnter(PuzzleState::GetReady), stop_all_sounds)
            .add_systems(Update, play_sound);
    }
}

fn load_audio(mut commands: Commands, asset_server: Res<AssetServer>) {
    // beware!  this must match the order of SoundEnum
    commands.insert_resource(SoundResource {
        handle_vector: vec![
            asset_server.load("snowglobe/sounds/title screen.ogg"),
            asset_server.load("snowglobe/sounds/ho ho ho.ogg"),
            asset_server.load("snowglobe/sounds/success.ogg"),
            asset_server.load("snowglobe/sounds/failure.ogg"),
            asset_server.load("snowglobe/sounds/turkey mistletoe.ogg"),
            asset_server.load("snowglobe/sounds/periodic table.ogg"),
            asset_server.load("snowglobe/sounds/twelve days.ogg"),
            asset_server.load("snowglobe/sounds/reindeer.ogg"),
            asset_server.load("snowglobe/sounds/waltz.ogg"),
            asset_server.load("snowglobe/sounds/grocery.ogg"),
            asset_server.load("snowglobe/sounds/family.ogg"),
            asset_server.load("snowglobe/sounds/cranberries.ogg"),
            asset_server.load("snowglobe/sounds/merry christmas.ogg"),
        ],
    });
}

fn play_sound(
    mut commands: Commands,
    sound_resources: Res<SoundResource>,
    mut sound_events: EventReader<SoundEvent>,
) {
    for sound_event in sound_events.iter() {
        commands.spawn(AudioBundle {
            source: match sound_event.0 {
                SoundsEnum::TitleScreen => sound_resources.handle_vector[0].clone(),
                SoundsEnum::HoHoHo => sound_resources.handle_vector[1].clone(),
                SoundsEnum::Success => sound_resources.handle_vector[2].clone(),
                SoundsEnum::Failure => sound_resources.handle_vector[3].clone(),
                SoundsEnum::TurkeyMistletoeSong => sound_resources.handle_vector[4].clone(),
                SoundsEnum::PeriodicTableSong => sound_resources.handle_vector[5].clone(),
                SoundsEnum::TwelveDaysSong => sound_resources.handle_vector[6].clone(),
                SoundsEnum::ReindeerSong => sound_resources.handle_vector[7].clone(),
                SoundsEnum::WaltzSong => sound_resources.handle_vector[8].clone(),
                SoundsEnum::GrocerySong => sound_resources.handle_vector[9].clone(),
                SoundsEnum::FamilyTreeSong => sound_resources.handle_vector[10].clone(),
                SoundsEnum::CranberriesSong => sound_resources.handle_vector[11].clone(),
                SoundsEnum::MerryChristmas => sound_resources.handle_vector[12].clone(),
            },
            settings: match sound_event.0 {
                SoundsEnum::TitleScreen
                | SoundsEnum::HoHoHo
                | SoundsEnum::Success
                | SoundsEnum::Failure
                | SoundsEnum::TurkeyMistletoeSong
                | SoundsEnum::WaltzSong
                | SoundsEnum::GrocerySong
                | SoundsEnum::CranberriesSong
                | SoundsEnum::MerryChristmas
                | SoundsEnum::PeriodicTableSong => PlaybackSettings::REMOVE,
                SoundsEnum::TwelveDaysSong
                | SoundsEnum::ReindeerSong
                | SoundsEnum::FamilyTreeSong => PlaybackSettings::LOOP,
            },
        });
    }
}

fn stop_all_sounds(mut audio_sink_query: Query<&mut AudioSink>) {
    for audio_sink in audio_sink_query.iter_mut() {
        audio_sink.stop();
    }
}
