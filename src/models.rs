
pub struct Board {
    pub characters: Vec<Character>
}

pub type CharacterId = u32;

pub struct Character {
    id:CharacterId,
    cls: CharacterClass,
    inventory: Vec<ItemId>,
}

/**
 * クラス
 */
pub enum CharacterClass {
    Ordinary(ordinary::Ordinary) /* 一般人 */,
    Brainwashed /* 洗脳された人 */,
    Insane /* 発狂した人 */
}

pub mod ordinary {
    use super::{*};
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

    pub enum Helth {
        Alive(HelthAlive),
        Dead
    }

    pub type Sanity = u32;
    
    pub struct HelthAlive {
        sanity: Sanity
    }
}

pub type ItemId = u32;

/**
 * アイテム
 */
pub struct Item {
    id: ItemId,
    cls: ItemClass
}

pub enum ItemClass {
    SkillBook,
    Note,
    Food,
    Other
}

pub type KnowledgeId = u32;
