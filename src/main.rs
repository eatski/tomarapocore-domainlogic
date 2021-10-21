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
        AppState::Blank
    }

    fn resolve(prev: &Self::State, command: &Self::Command) -> Self::Result {
        match (prev,command) {
            (AppState::Blank, AppCommand::Init { characters_num }) => AppResult::Init(init(*characters_num)),
        }
    }

    fn reducer(prev: &Self::State, result: &Self::Result) -> Self::State {
        todo!()
    }
}

enum AppState {
    Blank
}
enum AppCommand {
    Init {
        characters_num: usize
    }
}

enum AppResult {
    Init(Board)
}
