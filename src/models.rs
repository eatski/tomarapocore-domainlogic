
#[derive(Clone)]
pub struct Board {
    pub characters: Vec<Character>
}

pub type CharacterId = usize;

#[derive(Clone)]
pub struct Character {
    pub id: CharacterId,
    pub cls: CharacterClass,
    pub inventory: Vec<ItemId>,
}

/**
 * クラス
 */
#[derive(Clone)]
pub enum CharacterClass {
    Ordinary(ordinary::Ordinary) /* 一般人 */,
    Brainwashed /* 洗脳された人 */,
    Insane /* 発狂した人 */
}

pub mod ordinary {
    use super::{*};
    #[derive(Clone)]
    pub struct Ordinary {
        pub directivities: Vec<Directivity>,
        pub helth: Helth
    }

    /**
        指向性 そのキャラの価値観などが反映される
    */
    #[derive(Clone)]
    pub enum Directivity {
        Alive /* 生存 */,
        Possession(ItemId) /* アイテムなどの所持 */,
        Love(CharacterId) /* 対象キャラクターが生存しているかどうか */,
        Hatred(CharacterId) /* 対象キャラクターが死亡しているかどうか */,
        Justice /* 正義感 悪巧みを阻止する */,
        Secret(KnowledgeId) /* 秘密を守る */
    }

    #[derive(Clone)]
    pub enum Helth {
        Alive(HelthAlive),
        Dead
    }

    pub type Sanity = usize;

    #[derive(Clone)]
    pub struct HelthAlive {
        pub sanity: Sanity
    }
}

pub type ItemId = usize;

/**
 * アイテム
 */
pub struct Item {
    id: ItemId,
    cls: ItemClass
}

pub enum ItemClass {
    SkillBook(SkillId), 
    Note(KnowledgeId), //知識が記されてる
    Food, //食べ物 san値回復？
    Other
}

type SkillId = usize;

pub struct Skill {
    id: SkillId
}

pub type KnowledgeId = usize;

pub struct Knowledge {
    id: KnowledgeId,
    cls: KnowledgeClass
}

pub enum KnowledgeClass {
    /**
     * 探偵の残した証拠
     * 誰が洗脳者である確率が高いか
     * （ボードには何枚かの証拠が配置され間違った証拠（ミスリード）もいくつかあるが真を示した証拠ほど数が多い）
     */
    Evidence(CharacterId),
    /**
     * 証拠の調べたアリバイ
     * 誰が洗脳者ではないか
     */
    Alibi(CharacterId),
    /**
     * 隠したい秘密
     */
    Secret {
        id: SecretId,
        keeper: CharacterId
    }
}

pub type SecretId = usize;

pub type RoomId = usize;
pub struct Room {
    pub id: RoomId,
    pub cls: RoomClass
}
pub enum RoomClass {
    /**
     * キャラクターの居住地
     */
    Residence(CharacterId),
    /**
     * なんか適当に
     */
    FixmeRoomA,
    /**
     * なんか適当に
     */
    FixmeRoomB
}
