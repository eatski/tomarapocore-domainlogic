use core::panic;

use exprocess::core::{ExprocessCore};
use functions::init;
use models::{Board, CharacterId, RoomId};
mod models;
mod functions;

fn main() {
    println!("Hello, world!");
}

struct AppCore;
impl ExprocessCore for AppCore {
    type State = AppState;

    type Command = AppCommand;

    type Result = AppResult;

    fn init() -> Self::State {
        AppState {
            content: AppStateContent::Blank
        }
    }

    fn resolve(prev: &Self::State, command: &Self::Command) -> Self::Result {
        match (&prev.content,command) {
            (AppStateContent::Blank, AppCommand::Init { characters_num }) => AppResult::Init(init(*characters_num)),
            (AppStateContent::Playing { board:_, phase: PlayingPhase::Daytime }, AppCommand::SelectRoom (mov)) => AppResult::SelectRoom(mov.to_vec()),
            _ => panic!(),
        }
    }

    fn reducer(mut prev: &mut Self::State, result: &Self::Result) {
        match (&mut prev.content,result) {
            (
                AppStateContent::Blank,
                AppResult::Init(board)
            ) => {
                prev.content = AppStateContent::Playing {
                    board:board.clone(),phase: PlayingPhase::Daytime
                }
            }
            (
                AppStateContent::Playing { board, phase:_ },
                AppResult::SelectRoom(mov)
            ) => {
                let phase = PlayingPhase::Night { character_locations: mov.to_vec() };
                prev.content = AppStateContent::Playing { board:board.clone(),phase}
            },
            _ => panic!()
        }
    }
}

struct AppState {
    pub content: AppStateContent
}

enum AppStateContent {
    Blank,
    Playing {
        board: Board,
        phase: PlayingPhase
    }
}

enum PlayingPhase {
    Daytime,
    Night {
        character_locations: Vec<SelectRoom>
    },
    Midnight {
        character_locations: Vec<SelectRoom>
    }
}
enum AppCommand {
    Init {
        characters_num: usize
    },
    SelectRoom(Vec<SelectRoom>)
}

#[derive(Clone)]
struct SelectRoom {
    pub character: CharacterId,
    pub room: RoomId
}

enum AppResult {
    Init(Board),
    SelectRoom(Vec<SelectRoom>)
}
