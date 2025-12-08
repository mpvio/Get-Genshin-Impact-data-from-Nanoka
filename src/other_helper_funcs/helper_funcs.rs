use regex::Regex;
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};
use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::base_models::character::{Constellation, HasDescription};
use crate::parsed_models::{ParsedArtifact, ParsedConstellation};
use crate::parsed_models::ParsedCharacter;
use crate::parsed_models::ParsedCard;
use crate::parsed_models::ParsedWeapon;
use crate::weapon::Item;
use crate::weapon::Materials;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Parsed {
    C(ParsedCharacter),
    W(ParsedWeapon),
    A(ParsedArtifact),
    T(ParsedCard)
}

impl Parsed {
    pub fn name(&self) -> String {
        match self {
            Parsed::C(parsed_character) => parsed_character.name.clone(),
            Parsed::W(parsed_weapon) => parsed_weapon.name.clone(),
            Parsed::A(parsed_artifact) => parsed_artifact.name.clone(),
            Parsed::T(parsed_card) => {
                let name = parsed_card.name();
                return format!("{name} TCG");
            },
        }
    }
}

pub fn clean_text(input: &str) -> (String, Vec<String>) {
    return clean_text_colon(input, false);
}

pub fn clean_text_colon(input: &str, keep_colon: bool) -> (String, Vec<String>) {
    // Regex to capture LINK numbers and contents
    let link_re = Regex::new(r"\{LINK#([A-Z]\d+)\}(.*?)\{/LINK\}").unwrap();
    let mut link_numbers = Vec::new();
    
    // Process the text while capturing LINK numbers
    let processed_text = link_re.replace_all(input, |caps: &regex::Captures| {
        let link_num = caps.get(1).unwrap().as_str().trim_start_matches(char::is_alphabetic); // trim starting character (N or S)
        let content = caps.get(2).unwrap().as_str();
        link_numbers.push(link_num.to_string());
        String::from(content) // Keep the content in the text (only remove the LINK tags)
    }).to_string();

    // Now clean the text (including the preserved LINK contents)
    let close_color_re = Regex::new(r"</color>").unwrap();
    let replacement = if keep_colon { ":" } else { "" };
    let with_colons = close_color_re.replace_all(&processed_text, replacement);

    let open_color_re = Regex::new(r"<color=[^>]*>").unwrap();
    let without_colors = open_color_re.replace_all(&with_colons, "");

    let italic_tag_re = Regex::new(r"<i>.*?</i>").unwrap();
    let without_italics = italic_tag_re.replace_all(&without_colors, "");

    let newline_re = Regex::new(r"\\n|\n").unwrap();
    let with_spaces = newline_re.replace_all(&without_italics, " ");

    let space_re = Regex::new(r"\s+").unwrap();
    let cleaned = space_re.replace_all(&with_spaces, " ");

    let final_text = if keep_colon {
        cleaned.trim().to_string()
    } else {
        let sprite_re = Regex::new(r"\{SPRITE_PRESET#[^>]*\}").unwrap();
        sprite_re.replace_all(&cleaned, "").trim().to_string()
    };

    (final_text, link_numbers)
}

pub fn compare_color_texts(text1: &str, text2: &str) -> String {
    // regex to capture numbers and trailing non-numeric characters SEPARATELY
    let re = Regex::new(r"(.*?)(<color=#[0-9A-Fa-f]{8}>((?:\d+%?/)*\d+%?)([^<]*)</color>|$)").unwrap();
    let mut captures1 = re.captures_iter(text1);
    let mut captures2 = re.captures_iter(text2);
    let mut result = String::new();
    
    loop {
        let (cap1, cap2) = (captures1.next(), captures2.next());
        
        if cap1.is_none() && cap2.is_none() {
            break;
        }
        
        match (cap1, cap2) {
            (Some(c1), Some(c2)) => {
                // Handle non-colored text
                let plain1 = c1.get(1).unwrap().as_str();
                let plain2 = c2.get(1).unwrap().as_str();
                
                if plain1 != plain2 {
                    println!("Text structure differs between inputs");
                    println!("{plain1:#?}");
                    println!("{plain2:#?}");
                    result.push_str(plain1);
                    // result.push_str(&compare_color_texts(plain1, plain2));
                } else {
                    result.push_str(plain1);
                }
                
                // Handle colored content
                if let (Some(color1), Some(color2)) = (c1.get(3), c2.get(3)) {
                    let num1 = color1.as_str();
                    let num2 = color2.as_str();
                    let suffix1 = c1.get(4).map(|m| m.as_str()).unwrap_or("");
                    let suffix2 = c2.get(4).map(|m| m.as_str()).unwrap_or("");
                    
                    if num1 == num2 && suffix1 == suffix2 {
                        result.push_str(num1);
                        result.push_str(suffix1);
                    } else {
                        // Only include suffix if both have the same one
                        let common_suffix = if suffix1 == suffix2 { suffix1 } else { "" };
                        result.push_str(&format!("[{}|{}]{}", num1, num2, common_suffix));
                    }
                }
            },
            _ => panic!("Text structure differs between inputs"),
        }
    }
    
    result
}

pub fn accumulate_materials(materials_map: &BTreeMap<String, Materials>) -> Materials {
    let mut total_cost = 0;
    let mut item_map: HashMap<String, (u32, u32)> = HashMap::new(); // (count, rank)

    // Iterate through all materials and accumulate costs and item counts
    for materials in materials_map.values() {
        total_cost += materials.cost;
        
        for item in &materials.mats {
            let entry = item_map.entry(item.name.clone()).or_insert((0, item.rank));
            entry.0 += item.count;
            // Ensure we keep the highest rank if there are conflicting ranks for the same item
            if item.rank > entry.1 {
                entry.1 = item.rank;
            }
        }
    }

    // Convert to Vec<Item> and sort by rank, then name
    let mut mats: Vec<Item> = item_map
        .into_iter()
        .map(|(name, (count, rank))| Item { name, count, rank })
        .collect();
    
    mats.sort_by(|a, b| {
        let rank_cmp = a.rank.cmp(&b.rank);
        // if ranking is equal, sort by name as well
        // stops items of same rank being saved in random orders in final file
        if rank_cmp == Ordering::Equal {
            a.name.cmp(&b.name)
        } else {
            rank_cmp
        }
    });

    Materials {
        mats,
        cost: total_cost,
    }
}

pub fn convert_constellations(constellations: &mut Vec<Constellation>) -> Vec<ParsedConstellation> {
    let mut cons = Vec::<ParsedConstellation>::new();
    for con in constellations {
        let temp = ParsedConstellation {
            name: con.name.clone(),
            desc: con.description().to_string()
        };
        cons.push(temp);
    }
    cons
}