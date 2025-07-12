use ascension_funcs::get_ascension_stat_option;
use character::Character;
use material_funcs::parse_materials;
use parsed_character::ParsedCharacter;
use read_and_write_funcs::{check_and_write_to_file, get_ids_from_user, write_character_list_to_file};
use reqwest::Error;
use skill_funcs::handle_skills;
use hakushin_lists::{MinimalCharacterMap};
use helper_funcs::{compare_color_texts, Parsed};

use crate::{hakushin_lists::{MinimalArtifactMap, MinimalCardMap, MinimalWeaponMap}, helper_funcs::accumulate_materials, parsed_weapon::ParsedWeapon, read_and_write_funcs::{check_and_write, write_list_to_file}, weapon::Weapon};

pub mod character;
pub mod parsed_character;
pub mod ascension_funcs;
pub mod material_funcs;
pub mod skill_funcs;
pub mod read_and_write_funcs;
pub mod hakushin_lists;
pub mod weapon;
mod parsed_weapon;
pub mod helper_funcs;

#[tokio::main]
async fn main() {
    get_minimals().await;
    let inputs: String = get_ids_from_user();
    let ids : Vec<&str> = inputs.split_ascii_whitespace().collect();

    for id in ids {
        if id.len() == 5 {
            let res = weapon_access(id).await;
            match res {
                Ok(weapon) => check_and_write("weapon", Parsed::W(weapon)).await,
                Err(err) => println!("{err:#?}"),
            }
        }
        else {
            let character = character_api_access(id).await;
            check_and_write("character", Parsed::C(character)).await;
            //println!("{:#?}", character);
            //check_and_write_to_file(character).await;
        }
    }
}

async fn get_minimals() {
    println!("CHARACTERS:");
    get_minimal_character_list().await;
    println!("\nWEAPONS:");
    get_minimal_weapons().await;
    println!("\nARTIFACTS:");
    get_minimal_artifacts().await;
    println!("\nCARDS:");
    get_minimal_cards().await;
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

async fn get_minimal_artifacts() {
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
        }
    }
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






