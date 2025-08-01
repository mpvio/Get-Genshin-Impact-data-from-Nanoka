use std::collections::BTreeMap;

use reqwest::Error;

use crate::{
    base_models::{character::Character, hakushin_lists::{MinimalArtifact, MinimalArtifactMap}, tcg_cards::CharacterTCG, terms::TermMap, weapon::Weapon}, character_funcs::{ascension_funcs::get_ascension_stat_option, material_funcs::parse_materials, skill_funcs::handle_skills}, gui_funcs::display_lists::get_custom_name_from_id, other_helper_funcs::{character_error::CharacterError, helper_funcs::{accumulate_materials, clean_text_colon, compare_color_texts, Parsed}, read_and_write_funcs::check_and_write}, parsed_models::{ParsedArtifact, ParsedCard, ParsedCharacter, ParsedCharacterTCG, ParsedTalentTCG, ParsedWeapon}
};

pub async fn query_api(inputs: &String, artifacts: &Option<MinimalArtifactMap>) -> Vec<String> {
    let ids : Vec<&str> = inputs.split_ascii_whitespace().collect();
    let mut results = Vec::<String>::new();
    let mut terms: Option<TermMap> = None;
    let mut tried_terms = false;

    for id in ids {
        if id.len() == 4 || id.len() == 6 {
            match card_access(id).await {
                Ok(card) => {
                    results.append(&mut check_and_write("card", Parsed::T(card)).await);
                    //check_and_write("card", Parsed::T(card)).await;
                },
                Err(err) => {
                    results.push(format!("{err:#?}"));
                    println!("{err:#?}");
                },
            }
        } 
        else if id.len() == 5 {
            if let Some(ref sets) = artifacts {
                if sets.contains_key(id) {
                    // artifact
                    let artifact = sets.get(id).unwrap();
                    let new_art = artifact_access(artifact, id).await;
                    results.append(&mut check_and_write("artifact", Parsed::A(new_art)).await);
                    //check_and_write("artifact", Parsed::A(new_art)).await;
                } else {
                    results.append(&mut check_weapon(id).await);
                }
            } else {
                results.append(&mut check_weapon(id).await);
            }
        }
        else {
            if !tried_terms {
                terms = get_all_terms().await;
                tried_terms = true;
            }
            match character_api_access(id, &terms).await {
                Ok(character) => {
                    results.append(&mut check_and_write("character", Parsed::C(character)).await);
                },
                Err(err) => {
                    results.push(format!("{:#?}", err.message));
                },
            };
            
        }
    }

    results
}

async fn check_weapon(id: &str) -> Vec<String> {
    let res = weapon_access(id).await;
    match res {
        Ok(weapon) => check_and_write("weapon", Parsed::W(weapon)).await,
        Err(err) => {
            let mut v = Vec::<String>::new();
            v.push(format!("{err:#?}"));
            println!("{err:#?}");
            v
        },
    } 
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
                        name: get_custom_name_from_id(id, &card.name),
                        card_type: card.card_type,
                        hp: card.hp.unwrap(),
                        cost: card.cost.character().unwrap(), // u8
                        tag: card.tag,
                        talents: talents.character().unwrap().clone(),
                    })
            } else {
                    ParsedCard::Talent(ParsedTalentTCG {
                        name: get_custom_name_from_id(id, &card.name),
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
            //println!("{weapon:#?}");

            let r1 = &weapon.refinement.r1.desc;
            let r5 = &weapon.refinement.r5.desc;

            let eff = compare_color_texts(r1, r5);
            let mats = accumulate_materials(&weapon.materials);

            let parsed_weapon = ParsedWeapon {
                name: weapon.name, // get_custom_name_alt(id, &weapon.name)
                weapon_type: weapon.weapon_type,
                rarity: weapon.rarity,
                substat: weapon.weapon_prop.last().unwrap().prop_type.clone(),
                effect: eff,
                materials: mats
            };
            //println!("{parsed_weapon:#?}");

            return Ok(parsed_weapon);
        }
    }
    panic!("API CALL FAILED");
}

async fn character_api_access(char_id : &str, all_terms: &Option<TermMap>) -> Result<ParsedCharacter, CharacterError> {
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
                    let (skills, terms) = handle_skills(&result.skills);

                    // add terms
                    let term_descs = if let Some(t) = all_terms {
                        let outcome = get_specific_terms(t, &terms);
                        if outcome.is_empty() {
                            None
                        } else {
                            Some(outcome)
                        }
                    } else {
                        None
                    };
                    println!("{term_descs:#?}");

                    //get material list - Ascension [1 vec] AND Talents [1 per skill]
                    let (ascension_mats, talent_mats) = parse_materials(&result.materials);
                    
                    //sort passives by unlock ascension level
                    let mut passives = result.passives;
                    passives.sort_by(|a, b| a.unlock.cmp(&b.unlock));

                    let complete_character = ParsedCharacter {
                        name: get_custom_name_from_id(char_id, &result.name),
                        vision: result.chara_info.vision,
                        weapon: result.weapon,
                        rarity: result.rarity,
                        ascension_stat,
                        skills,
                        passives,
                        constellations: result.constellations,
                        ascension_mats,
                        talent_mats,
                        term_descs
                    };

                    return Ok(complete_character);
                } else {
                    //println!("JSON parsing failed.");
                    return Err(CharacterError { 
                        message: String::from("JSON parsing failed.")
                    });
                }
            } else {
                //println!("Response not OK.");
                return Err(CharacterError { 
                    message: String::from("Response not OK.")
                });
            }
        } else {
            //println!("No response.");
            return Err(CharacterError { 
                message: String::from("No response.")
            });
        }
    } else {
        //println!("URL get failed.");
        return Err(CharacterError { 
            message: String::from("URL get failed.")
        });
    }
}

async fn get_all_terms() -> Option<TermMap> {
    let url = "https://api.hakush.in/gi/5.8.50/en/hyperlink.json";
    let response = reqwest::get(url).await.ok()?;
    let term_map = response.json::<TermMap>().await.ok()?;
    return Some(term_map);
}

fn get_specific_terms(all_terms: &TermMap, terms: &Vec<String>) -> BTreeMap<String, String> {
    let mut relevant_terms = BTreeMap::<String, String>::new();
    for term in terms {
        if let Some(res) = all_terms.get(term) {
            let (clean_desc, more_terms) = clean_text_colon(&res.desc, false);
            // add currently searched term to relevant_terms
            relevant_terms.insert(res.name.clone(), clean_desc);
            // if the term itself uses other terms, look for them recursively and add them to THIS relevant_terms
            if !more_terms.is_empty() {
                let mut nested_terms = get_specific_terms(all_terms, &more_terms);
                relevant_terms.append(&mut nested_terms);
            }
        }
    }
    return relevant_terms;
}