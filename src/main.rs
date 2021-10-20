use exprocess::core::{ExprocessCore};

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
        todo!()
    }

    fn reducer(prev: &Self::State, result: &Self::Result) -> Self::State {
        todo!()
    }
}


enum AppState {
    Blank
}
enum AppCommand {

}

enum AppResult {

}
