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
        match (prev,command) {
            (AppState::Blank, AppCommand::Init { characters_num }) => init(*characters_num),
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
    Init {
        characters: Vec<Character>
    }
}

type CharacterId = u32;

struct Character {
    id:CharacterId,
    cls: CharacterClass,
    inventory: Vec<ItemId>,
    // knowledges: Vec<KnowledgeId>
}

/**
 * クラス
 */
enum CharacterClass {
    Ordinary(ordinary::Ordinary) /* 一般人 */,
    Brainwashed /* 洗脳された人 */,
    Insane /* 発狂した人 */
}

pub mod ordinary {
    use crate::{*};
    pub struct Ordinary {
        directivities: Vec<Directivity>,
        helth: Helth
    }

    /**
        指向性 そのキャラの価値観などが反映される
    */
    pub enum Directivity {
        Alive /* 生存 */,
        Possession(ItemId) /* アイテムなどの所持 */,
        Love(CharacterId) /* 対象キャラクターが生存しているかどうか */,
        Hatred(CharacterId) /* 対象キャラクターが死亡しているかどうか */,
        Justice /* 悪巧みを阻止する */,
        Secret(KnowledgeId) /* 秘密を守る */
    }

    enum Helth {
        Alive(HelthAlive),
        Dead
    }

    type Sanity = u32;
    
    struct HelthAlive {
        sanity: Sanity
    }
}

type ItemId = u32;

/**
 * アイテム
 */
struct Item {
    id: ItemId,
    cls: ItemClass
}

enum ItemClass {
    SkillBook,
    Note,
    Food,
    Other
}

type KnowledgeId = u32;

fn init(characters_num: usize) -> AppResult {
    AppResult::Init {
        characters: todo!()
    }
}
