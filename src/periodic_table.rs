use bevy::prelude::*;
use bevy::text::BreakLineOn;

use crate::timer_bar::TimerResource;
use crate::GameState;
use crate::PuzzleState;
use crate::Score;
use crate::ScoreChange;
use crate::SoundEvent;
use crate::SoundsEnum;

pub struct PeriodicTablePlugin;

const ELEMENT_SIZE: f32 = 55.;
const FONT_SIZE: f32 = 20.;

#[derive(Component)]
struct ElementData {
    x_pos: f32,
    y_pos: f32,
    text: String,
    is_correct_element: bool,
    has_been_selected: bool,
}

#[derive(Resource)]
struct CursorPositionResource {
    cursor_position: Vec2,
}
impl Default for CursorPositionResource {
    fn default() -> Self {
        Self {
            cursor_position: Vec2::MAX, // to prevent cursor position from being zero and selecting an element (ugh)
        }
    }
}

impl Plugin for PeriodicTablePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorPositionResource>()
            .add_systems(OnEnter(GameState::PuzzlePeriodicTable), setup)
            .add_systems(
                OnEnter(PuzzleState::InProgress),
                begin_periodic_table_puzzle,
            )
            .add_systems(
                Update,
                play_periodic_table_puzzle.run_if(in_state(GameState::PuzzlePeriodicTable)),
            )
            .add_systems(
                Update,
                change_colors.run_if(in_state(GameState::PuzzlePeriodicTable)),
            )
            .add_systems(OnExit(GameState::PuzzlePeriodicTable), cleanup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut element_data_vector: Vec<ElementData> = Vec::new();

    const STARTING_X_POS: f32 = -450.0;
    const STARTING_Y_POS: f32 = 150.0;

    // row 1
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (0.0  * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (0.0 * ELEMENT_SIZE), text: "1\nH".to_owned(),   is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (17.0 * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (0.0 * ELEMENT_SIZE), text: "2\nHe".to_owned(),  is_correct_element: false, has_been_selected: false});

    // row 2
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (0.0  * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (1.0 * ELEMENT_SIZE), text: "3\nLi".to_owned(),  is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (1.0  * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (1.0 * ELEMENT_SIZE), text: "4\nBe".to_owned(),  is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (12.0 * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (1.0 * ELEMENT_SIZE), text: "5\nB".to_owned(),   is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (13.0 * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (1.0 * ELEMENT_SIZE), text: "6\nC".to_owned(),   is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (14.0 * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (1.0 * ELEMENT_SIZE), text: "7\nN".to_owned(),   is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (15.0 * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (1.0 * ELEMENT_SIZE), text: "8\nO".to_owned(),   is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (16.0 * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (1.0 * ELEMENT_SIZE), text: "9\nF".to_owned(),   is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (17.0 * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (1.0 * ELEMENT_SIZE), text: "10\nNe".to_owned(), is_correct_element: false, has_been_selected: false});

    // row 3
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (0.0  * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (2.0 * ELEMENT_SIZE), text: "11\nNa".to_owned(),  is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (1.0  * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (2.0 * ELEMENT_SIZE), text: "12\nMg".to_owned(),  is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (12.0 * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (2.0 * ELEMENT_SIZE), text: "13\nAl".to_owned(),  is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (13.0 * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (2.0 * ELEMENT_SIZE), text: "14\nSi".to_owned(),  is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (14.0 * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (2.0 * ELEMENT_SIZE), text: "15\nP".to_owned(),   is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (15.0 * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (2.0 * ELEMENT_SIZE), text: "16\nS".to_owned(),   is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (16.0 * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (2.0 * ELEMENT_SIZE), text: "17\nCl".to_owned(),  is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (17.0 * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (2.0 * ELEMENT_SIZE), text: "18\nAr".to_owned(),  is_correct_element: false, has_been_selected: false});

    // row 3
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (0.0  * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (3.0 * ELEMENT_SIZE), text: "19\nK".to_owned(),   is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (1.0  * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (3.0 * ELEMENT_SIZE), text: "20\nCa".to_owned(),  is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (2.0  * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (3.0 * ELEMENT_SIZE), text: "21\nSc".to_owned(),  is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (3.0  * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (3.0 * ELEMENT_SIZE), text: "22\nTi".to_owned(),  is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (4.0  * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (3.0 * ELEMENT_SIZE), text: "23\nV".to_owned(),   is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (5.0  * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (3.0 * ELEMENT_SIZE), text: "24\nCr".to_owned(),  is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (6.0  * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (3.0 * ELEMENT_SIZE), text: "25\nMn".to_owned(),  is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (7.0  * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (3.0 * ELEMENT_SIZE), text: "26\nFe".to_owned(),  is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (8.0  * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (3.0 * ELEMENT_SIZE), text: "27\nCo".to_owned(),  is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (9.0  * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (3.0 * ELEMENT_SIZE), text: "28\nNi".to_owned(),  is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (10.0 * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (3.0 * ELEMENT_SIZE), text: "29\nCu".to_owned(),  is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (11.0 * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (3.0 * ELEMENT_SIZE), text: "30\nZn".to_owned(),  is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (12.0 * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (3.0 * ELEMENT_SIZE), text: "31\nGa".to_owned(),  is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (13.0 * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (3.0 * ELEMENT_SIZE), text: "32\nGe".to_owned(),  is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (14.0 * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (3.0 * ELEMENT_SIZE), text: "33\nAs".to_owned(),  is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (15.0 * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (3.0 * ELEMENT_SIZE), text: "34\nSe".to_owned(),  is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (16.0 * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (3.0 * ELEMENT_SIZE), text: "35\nBr".to_owned(),  is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (17.0 * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (3.0 * ELEMENT_SIZE), text: "36\nKr".to_owned(),  is_correct_element: false, has_been_selected: false});

    // row 4
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (0.0  * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (4.0 * ELEMENT_SIZE), text: "37\nRb".to_owned(),  is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (1.0  * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (4.0 * ELEMENT_SIZE), text: "38\nSr".to_owned(),  is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (2.0  * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (4.0 * ELEMENT_SIZE), text: "39\nY".to_owned(),   is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (3.0  * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (4.0 * ELEMENT_SIZE), text: "40\nZr".to_owned(),  is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (4.0  * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (4.0 * ELEMENT_SIZE), text: "41\nNb".to_owned(),  is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (5.0  * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (4.0 * ELEMENT_SIZE), text: "42\nMo".to_owned(),  is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (6.0  * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (4.0 * ELEMENT_SIZE), text: "43\nTc".to_owned(),  is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (7.0  * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (4.0 * ELEMENT_SIZE), text: "44\nRu".to_owned(),  is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (8.0  * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (4.0 * ELEMENT_SIZE), text: "45\nRh".to_owned(),  is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (9.0  * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (4.0 * ELEMENT_SIZE), text: "46\nPd".to_owned(),  is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (10.0 * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (4.0 * ELEMENT_SIZE), text: "47\nAg".to_owned(),  is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (11.0 * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (4.0 * ELEMENT_SIZE), text: "48\nCd".to_owned(),  is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (12.0 * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (4.0 * ELEMENT_SIZE), text: "49\nIn".to_owned(),  is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (13.0 * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (4.0 * ELEMENT_SIZE), text: "50\nSn".to_owned(),  is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (14.0 * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (4.0 * ELEMENT_SIZE), text: "51\nSb".to_owned(),  is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (15.0 * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (4.0 * ELEMENT_SIZE), text: "52\nTe".to_owned(),  is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (16.0 * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (4.0 * ELEMENT_SIZE), text: "53\nI".to_owned(),   is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (17.0 * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (4.0 * ELEMENT_SIZE), text: "54\nXe".to_owned(),  is_correct_element: false, has_been_selected: false});

    // row 5
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (0.0  * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (5.0 * ELEMENT_SIZE), text: "55\nCs".to_owned(),  is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (1.0  * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (5.0 * ELEMENT_SIZE), text: "56\nBa".to_owned(),  is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (2.0  * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (5.0 * ELEMENT_SIZE), text: "57\nLa".to_owned(),  is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (3.0  * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (5.0 * ELEMENT_SIZE), text: "72\nHf".to_owned(),  is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (4.0  * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (5.0 * ELEMENT_SIZE), text: "73\nTa".to_owned(),  is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (5.0  * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (5.0 * ELEMENT_SIZE), text: "74\nW".to_owned(),   is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (6.0  * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (5.0 * ELEMENT_SIZE), text: "75\nRe".to_owned(),  is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (7.0  * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (5.0 * ELEMENT_SIZE), text: "76\nOs".to_owned(),  is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (8.0  * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (5.0 * ELEMENT_SIZE), text: "77\nIr".to_owned(),  is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (9.0  * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (5.0 * ELEMENT_SIZE), text: "78\nPt".to_owned(),  is_correct_element: true , has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (10.0 * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (5.0 * ELEMENT_SIZE), text: "79\nAu".to_owned(),  is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (11.0 * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (5.0 * ELEMENT_SIZE), text: "80\nHg".to_owned(),  is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (12.0 * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (5.0 * ELEMENT_SIZE), text: "81\nTi".to_owned(),  is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (13.0 * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (5.0 * ELEMENT_SIZE), text: "82\nPb".to_owned(),  is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (14.0 * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (5.0 * ELEMENT_SIZE), text: "83\nBi".to_owned(),  is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (15.0 * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (5.0 * ELEMENT_SIZE), text: "84\nPo".to_owned(),  is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (16.0 * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (5.0 * ELEMENT_SIZE), text: "85\nAt".to_owned(),  is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (17.0 * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (5.0 * ELEMENT_SIZE), text: "86\nRn".to_owned(),  is_correct_element: false, has_been_selected: false});

    // row 6
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (0.0  * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (6.0 * ELEMENT_SIZE), text: "87\nFr".to_owned(),  is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (1.0  * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (6.0 * ELEMENT_SIZE), text: "88\nRa".to_owned(),  is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (2.0  * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (6.0 * ELEMENT_SIZE), text: "89\nAc".to_owned(),  is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (3.0  * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (6.0 * ELEMENT_SIZE), text: "104\nRf".to_owned(), is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (4.0  * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (6.0 * ELEMENT_SIZE), text: "105\nDb".to_owned(), is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (5.0  * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (6.0 * ELEMENT_SIZE), text: "106\nSg".to_owned(), is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (6.0  * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (6.0 * ELEMENT_SIZE), text: "107\nBh".to_owned(), is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (7.0  * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (6.0 * ELEMENT_SIZE), text: "108\nHs".to_owned(), is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (8.0  * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (6.0 * ELEMENT_SIZE), text: "109\nMt".to_owned(), is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (9.0  * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (6.0 * ELEMENT_SIZE), text: "110\nDs".to_owned(), is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (10.0 * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (6.0 * ELEMENT_SIZE), text: "111\nRg".to_owned(), is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (11.0 * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (6.0 * ELEMENT_SIZE), text: "112\nCn".to_owned(), is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (12.0 * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (6.0 * ELEMENT_SIZE), text: "113\nNh".to_owned(), is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (13.0 * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (6.0 * ELEMENT_SIZE), text: "114\nFl".to_owned(), is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (14.0 * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (6.0 * ELEMENT_SIZE), text: "115\nMc".to_owned(), is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (15.0 * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (6.0 * ELEMENT_SIZE), text: "116\nLv".to_owned(), is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (16.0 * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (6.0 * ELEMENT_SIZE), text: "117\nTs".to_owned(), is_correct_element: false, has_been_selected: false});
    #[rustfmt::skip] element_data_vector.push(ElementData {x_pos: STARTING_X_POS + (17.0 * ELEMENT_SIZE), y_pos: STARTING_Y_POS - (6.0 * ELEMENT_SIZE), text: "118\nOg".to_owned(), is_correct_element: false, has_been_selected: false});

    for element_data in element_data_vector.iter() {
        commands
            .spawn((
                SpriteBundle {
                    texture: asset_server.load("snowglobe/icons/element background.png"),
                    sprite: Sprite {
                        custom_size: Some(Vec2 {
                            x: ELEMENT_SIZE,
                            y: ELEMENT_SIZE,
                        }),
                        ..default()
                    },
                    visibility: Visibility::Hidden,
                    transform: Transform {
                        translation: Vec3::new(element_data.x_pos, element_data.y_pos, 0.0),
                        ..default()
                    },

                    ..default()
                },
                ElementData {
                    x_pos: element_data.x_pos,
                    y_pos: element_data.y_pos,
                    text: element_data.text.clone(),
                    is_correct_element: element_data.is_correct_element,
                    has_been_selected: false,
                },
            ))
            .with_children(|parent| {
                parent.spawn(Text2dBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            element_data.text.clone(),
                            TextStyle {
                                font: asset_server.load("snowglobe/fonts/arialceb.ttf"),
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
}

fn begin_periodic_table_puzzle(
    mut sprite_visibility_query: Query<(&mut Visibility, With<ElementData>)>,
) {
    for mut sprite_visibility in &mut sprite_visibility_query.iter_mut() {
        *sprite_visibility.0 = Visibility::Visible;
    }
}

fn play_periodic_table_puzzle(
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Query<&Window>,
    mut cursor_position_resource: ResMut<CursorPositionResource>,
    mut score: ResMut<Score>,
    mut sound_event_writer: EventWriter<SoundEvent>,
    mut transforms: Query<&mut Transform>,
    mut elements: Query<(Entity, &mut ElementData)>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut next_puzzle_state: ResMut<NextState<PuzzleState>>,
    timer_resource: Res<TimerResource>,
) {
    for ev in cursor_moved_events.iter() {
        let window = windows.single();
        let half_window = Vec2::new(window.resolution.width() / 2.0, window.height() / 2.0);
        cursor_position_resource.cursor_position = ev.position - half_window;
    }

    if mouse_button_input.just_pressed(MouseButton::Left) {
        for (entity, mut element) in elements.iter_mut() {
            let sprite_pos = transforms.get_mut(entity).unwrap().translation;

            let vector_to_sprite = Vec3::new(
                sprite_pos.x - cursor_position_resource.cursor_position.x,
                -sprite_pos.y - cursor_position_resource.cursor_position.y,
                0.0,
            );

            if vector_to_sprite.length() < ELEMENT_SIZE / 2.0 && !element.has_been_selected {
                element.has_been_selected = true;

                if element.is_correct_element {
                    sound_event_writer.send(SoundEvent(SoundsEnum::HoHoHo));
                    next_puzzle_state.set(PuzzleState::Complete);

                    let time_remaining = (timer_resource.time_remaining.duration()
                        - timer_resource.time_remaining.elapsed())
                    .as_millis() as f32;

                    let current_score = score.current_score;
                    score.score_change_vector.push(ScoreChange {
                        initial_score: current_score,
                        delta_score: time_remaining,
                    });
                } else {
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

fn change_colors(mut sprite_query: Query<(&mut Sprite, &ElementData)>) {
    for (mut sprite, element_data) in sprite_query.iter_mut() {
        if element_data.has_been_selected {
            match element_data.is_correct_element {
                true => sprite.color = Color::GREEN,
                false => sprite.color = Color::RED,
            }
        }
    }
}

fn cleanup(mut commands: Commands, element_data_query: Query<(Entity, &ElementData)>) {
    for (entity, _element_data) in element_data_query.iter() {
        commands.entity(entity).despawn_recursive(); // to get rid of child text
    }
    commands.remove_resource::<CursorPositionResource>();
}
