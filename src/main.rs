use std::collections::BTreeMap;

use ascension_funcs::get_ascension_stat_option;
use character::Character;
use material_funcs::parse_materials;
use parsed_character::ParsedCharacter;
use read_and_write_funcs::{get_ids_from_user, write_character_list_to_file};
use reqwest::Error;
use skill_funcs::handle_skills;
use hakushin_lists::{MinimalCharacterMap};
use helper_funcs::{compare_color_texts, Parsed};

use crate::{hakushin_lists::{MinimalArtifact, MinimalArtifactMap, MinimalCardMap, MinimalWeaponMap}, helper_funcs::accumulate_materials, parsed_artifact::ParsedArtifact, parsed_tcg::{ParsedCard, ParsedCharacterTCG, ParsedTalentTCG}, parsed_weapon::ParsedWeapon, read_and_write_funcs::{check_and_write, write_list_to_file}, tcg_cards::CharacterTCG, weapon::Weapon};

pub mod other_helper_funcs;
pub mod parsed_models;
pub mod base_models;
pub mod character_funcs;

use parsed_models::{parsed_artifact, parsed_character, parsed_weapon, parsed_tcg};
use base_models::{character, weapon, tcg_cards, hakushin_lists};
use character_funcs::{ascension_funcs, material_funcs, skill_funcs};
use other_helper_funcs::{helper_funcs, read_and_write_funcs};

// TODO: Clean up!
#[tokio::main]
async fn main() {
    let artifacts = get_minimals().await;
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

async fn check_weapon(id: &str) {
    let res = weapon_access(id).await;
    match res {
        Ok(weapon) => check_and_write("weapon", Parsed::W(weapon)).await,
        Err(err) => println!("{err:#?}"),
    } 
}

async fn get_minimals() -> Option<MinimalArtifactMap> {
    println!("CHARACTERS:");
    get_minimal_character_list().await;
    println!("\nWEAPONS:");
    get_minimal_weapons().await;
    println!("\nCARDS:");
    get_minimal_cards().await;
    println!("\nARTIFACTS:");
    return get_minimal_artifacts().await;
}

async fn get_minimal_character_list() {
    let url = "https://api.hakush.in/gi/data/character.json";
    let chars_per_row = 5;

    if let Ok(response) = reqwest::get(url).await {
        if let Ok(map) = response.json::<MinimalCharacterMap>().await {
            let mut count = 0;
            for (key, value) in &map {
                print!("{:<10}: {:<20} ", key, value.en);
                count += 1;
                if count % chars_per_row == 0 {
                    println!(); //new line after every N characters
                }
            }
            if count % chars_per_row != 0 {
                println!(); //forcibly switch to new line if total characters isn't a multiple of N
            }
            write_character_list_to_file(&map);
        }
    }
}

async fn get_minimal_weapons() {
    let url = "https://api.hakush.in/gi/data/weapon.json";
    let chars_per_row = 5;

    if let Ok(response) = reqwest::get(url).await {
        if let Ok(map) = response.json::<MinimalWeaponMap>().await {
            let mut count = 0;
            for (key, value) in &map {
                print!("{:<10}: {:<20} ", key, value.en);
                count += 1;
                if count % chars_per_row == 0 {
                    println!(); //new line after every N characters
                }
            }
            if count % chars_per_row != 0 {
                println!(); //forcibly switch to new line if total characters isn't a multiple of N
            }
            write_list_to_file("weapon", &map);
        }
    }
}

async fn get_minimal_cards() {
    let url = "https://api.hakush.in/gi/data/gcg.json";
    let chars_per_row = 5;

    if let Ok(response) = reqwest::get(url).await {
        if let Ok(map) = response.json::<MinimalCardMap>().await {
            let mut count = 0;
            for (key, value) in &map {
                let name = if key == "1506" {
                    "Wanderer" // implement this better
                } else {
                    &value.en
                };
                print!("{:<10}: {:<20} ", key, name);
                count += 1;
                if count % chars_per_row == 0 {
                    println!(); //new line after every N characters
                }
            }
            if count % chars_per_row != 0 {
                println!(); //forcibly switch to new line if total characters isn't a multiple of N
            }
            write_list_to_file("gcg", &map);
        }
    }
}

async fn get_minimal_artifacts() -> Option<MinimalArtifactMap> {
    let url = "https://api.hakush.in/gi/data/artifact.json";
    let chars_per_row = 5;

    if let Ok(response) = reqwest::get(url).await {
        if let Ok(mut map) = response.json::<MinimalArtifactMap>().await {
            let mut count = 0;
            for (key, value) in &mut map {
                print!("{:<10}: {:<20} ", key, value.set.first_entry().unwrap().get().name.en);
                count += 1;
                if count % chars_per_row == 0 {
                    println!(); //new line after every N characters
                }
            }
            if count % chars_per_row != 0 {
                println!(); //forcibly switch to new line if total characters isn't a multiple of N
            }
            write_list_to_file("artifact", &map);
            return Some(map);
        }
    }

    None
}

async fn artifact_access(artifact: &MinimalArtifact, key: &str) -> ParsedArtifact {
    let s = &artifact.set;
    // get 2pc and 4pc effects
    let new_key = format!("2{key}");
    let fst = s.get(&format!("{new_key}0")).unwrap();
    let lst = s.get(&format!("{new_key}1")).unwrap();
    // add to new artifact struct
    let new_artifact = ParsedArtifact {
        name: fst.name.en.clone(),
        two: fst.desc.en.clone(),
        four: lst.desc.en.clone()
    };
    return new_artifact;

}

async fn card_access(id: &str) -> Result<ParsedCard, Error> {
    let base_url = format!("https://api.hakush.in/gi/data/en/gcg/{id}.json");

    if let Ok(url) = reqwest::Url::parse(&base_url) {
        //println!("1");
        let response = reqwest::get(url).await?;
        //println!("2");
        if response.status() == reqwest::StatusCode::OK {
            //println!("3");
            let card = response.json::<CharacterTCG>().await?;
            //println!("{card:#?}\n");
            let all_terms: BTreeMap<String, String> = card.get_tree();

            // for (key, value) in card.get_tree() {
            //     talents.insert(key, value.convert(&all_terms));
            // }
            let talents = card.convert(&all_terms);
            let card_type = &card.card_type;

            let parsed_card = if card_type == "Character" {
                    ParsedCard::Character(ParsedCharacterTCG {
                        name: card.name,
                        card_type: card.card_type,
                        hp: card.hp.unwrap(),
                        cost: card.cost.character().unwrap(), // u8
                        tag: card.tag,
                        talents: talents.character().unwrap().clone(),
                    })
            } else {
                    ParsedCard::Talent(ParsedTalentTCG {
                        name: card.name,
                        card_type: card.card_type,
                        cost: card.cost.talent().unwrap().to_vec(), // Vec<TalentTCGCost>
                        tag: card.tag,
                        talents: talents.talent().unwrap().clone(),
                    })
            };

            return Ok(parsed_card)
            //println!("{parsed_card:#?}");
        }
    }
    panic!("API FAIL");
}

async fn weapon_access(id: &str) -> Result<ParsedWeapon, Error>{
    let base_url = format!("https://api.hakush.in/gi/data/en/weapon/{}.json", id);
    //println!("WEAPON");

    if let Ok(url) = reqwest::Url::parse(&base_url) {
        //println!("1");
        let response = reqwest::get(url).await?;
        //println!("2");
        if response.status() == reqwest::StatusCode::OK {
            //println!("3");
            let weapon = response.json::<Weapon>().await?;

            let r1 = &weapon.refinement.r1.desc;
            let r5 = &weapon.refinement.r5.desc;

            let eff = compare_color_texts(r1, r5);
            let mats = accumulate_materials(&weapon.materials);

            let parsed_weapon = ParsedWeapon {
                name: weapon.name,
                weapon_type: weapon.weapon_type,
                rarity: weapon.rarity,
                substat: weapon.weapon_prop.last().unwrap().prop_type.clone(),
                effect: eff,
                materials: mats
            };

            return Ok(parsed_weapon);
        }
    }
    panic!("API CALL FAILED");
}

async fn character_api_access(char_id : &str) -> ParsedCharacter {
    let base_url = format!("https://api.hakush.in/gi/data/en/character/{}.json",char_id);
    //println!("CHARACTER");

    if let Ok(get_url) = reqwest::Url::parse(&base_url) {
        let response = reqwest::get(get_url).await;
        if let Ok(resp) = response {
            if resp.status() == reqwest::StatusCode::OK {
                let parsed_result = resp.json::<Character>().await;
                if let Ok(result) = parsed_result {
                    //get ascension stat
                    let x = result.stats_modifier.ascension.first();
                    let ascension_stat = get_ascension_stat_option(x).to_string();
                    //println!("{ascension_stat}");

                    //parse skills for point breakdowns [or just remove "" and 0.0s?]
                    let skills = handle_skills(&result.skills);

                    //get material list - Ascension [1 vec] AND Talents [1 per skill]
                    let (ascension_mats, talent_mats) = parse_materials(&result.materials);
                    
                    //sort passives by unlock ascension level
                    let mut passives = result.passives;
                    passives.sort_by(|a, b| a.unlock.cmp(&b.unlock));

                    let complete_character = ParsedCharacter {
                        name: result.name,
                        vision: result.chara_info.vision,
                        weapon: result.weapon,
                        rarity: result.rarity,
                        ascension_stat,
                        skills,
                        passives,
                        constellations: result.constellations,
                        ascension_mats,
                        talent_mats
                    };

                    return complete_character;
                } else {
                    //println!("JSON parsing failed.");
                    panic!("JSON parsing failed.");
                }
            } else {
                //println!("Response not OK.");
                panic!("Response not OK.");
            }
        } else {
            //println!("No response.");
            panic!("No response.");
        }
    } else {
        //println!("URL get failed.");
        panic!("URL get failed.");
    }
}






