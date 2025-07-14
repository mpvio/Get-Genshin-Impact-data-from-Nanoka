use read_and_write_funcs::get_ids_from_user;
use helper_funcs::Parsed;

use crate::{
    api_funcs::get_item_funcs::{artifact_access, card_access, character_api_access, check_weapon}, 
    get_minimal_lists::get_minimals, 
    read_and_write_funcs::check_and_write
};

pub mod other_helper_funcs;
pub mod parsed_models;
pub mod base_models;
pub mod character_funcs;
pub mod api_funcs;

use base_models::{character, weapon, tcg_cards, hakushin_lists};
use other_helper_funcs::{helper_funcs, read_and_write_funcs};
use api_funcs::{get_minimal_lists};

// TODO: Clean up!
#[tokio::main]
async fn main() {
    let (_, _, _, artifacts) = get_minimals().await;
    let inputs: String = get_ids_from_user();
    let ids : Vec<&str> = inputs.split_ascii_whitespace().collect();
    //let mut ids_len5: Vec<&str> = Vec::new();

    for id in ids {
        if id.len() == 4 || id.len() == 6 {
            match card_access(id).await {
                Ok(card) => {
                    check_and_write("card", Parsed::T(card)).await;
                },
                Err(err) => println!("{err:#?}"),
            }
        } 
        else if id.len() == 5 {
            if let Some(ref sets) = artifacts {
                if sets.contains_key(id) {
                    // artifact
                    let artifact = sets.get(id).unwrap();
                    let new_art = artifact_access(artifact, id).await;
                    check_and_write("artifact", Parsed::A(new_art)).await;
                } else {
                    check_weapon(id).await;
                }
            } else {
                check_weapon(id).await;
            }
            //ids_len5.push(id);
        }
        else {
            let character = character_api_access(id).await;
            check_and_write("character", Parsed::C(character)).await;
            //println!("{:#?}", character);
            //check_and_write_to_file(character).await;
        }
    }
}






