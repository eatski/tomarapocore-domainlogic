use exprocess::core::{ExprocessCore};
use functions::init;
use models::{Board};
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
            _ => todo!()
        }
    }

    fn reducer(mut prev: &mut Self::State, result: &Self::Result) {
        match (&prev.content,result) {
            (AppStateContent::Blank, AppResult::Init(board)) => {
                prev.content = AppStateContent::Playing {
                    board:board.clone()
                }
            },
            (AppStateContent::Playing { board }, AppResult::Init(_)) => {
                todo!()
            },
        }
    }
}

struct AppState {
    pub content: AppStateContent
}

enum AppStateContent {
    Blank,
    Playing {
        board: Board
    }
}
enum AppCommand {
    Init {
        characters_num: usize
    }
}

enum AppResult {
    Init(Board)
}
