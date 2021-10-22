use crate::models::{Board, Character, CharacterClass, CharacterId, ItemId, ordinary::{Directivity, Helth, HelthAlive, Ordinary}};
use rand::{self, Rng};

pub fn init(characters_num: usize) -> Board {
    let who_is_brainwashed : CharacterId = rand::thread_rng().gen_range(0..characters_num);
    Board {
        characters: (0..characters_num)
            .into_iter()
            .map(|id| 
                if who_is_brainwashed == id {
                    (id,CharacterClass::Brainwashed)
                 } else {
                    let ordinary = Ordinary {
                        helth: Helth::Alive(
                            HelthAlive {
                                sanity: 100
                            }
                        ),
                        directivities: init_directivities()
                    };
                    (id,CharacterClass::Ordinary(ordinary))
                 }
            ).map(|(id,cls)| 
                Character {
                    id,cls,inventory: init_inventry()
                }
            )
            .collect()
    }
}

fn init_inventry() -> Vec<ItemId> {
    todo!()
}
fn init_directivities() -> Vec<Directivity> {
    todo!()
}