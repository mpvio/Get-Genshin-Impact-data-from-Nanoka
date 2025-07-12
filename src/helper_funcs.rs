use regex::Regex;
use std::collections::{BTreeMap, HashMap};
use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::parsed_character::ParsedCharacter;
use crate::parsed_weapon::ParsedWeapon;
use crate::weapon::Item;
use crate::weapon::Materials;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Parsed {
    C(ParsedCharacter),
    W(ParsedWeapon)
}

impl Parsed {
    pub fn name(&self) -> &String {
        match self {
            Parsed::C(parsed_character) => &parsed_character.name,
            Parsed::W(parsed_weapon) => &parsed_weapon.name,
        }
    }
}

pub fn compare_color_texts(text1: &str, text2: &str) -> String {
    let re = Regex::new(r"(.*?)(<color=#[0-9A-Fa-f]{8}>([^<]*)</color>|$)").unwrap();
    
    let mut captures1 = re.captures_iter(text1);
    let mut captures2 = re.captures_iter(text2);
    let mut result = String::new();
    
    loop {
        let (cap1, cap2) = (captures1.next(), captures2.next());
        
        // Both texts exhausted
        if cap1.is_none() && cap2.is_none() {
            break;
        }
        
        match (cap1, cap2) {
            (Some(c1), Some(c2)) => {
                // Compare non-colored text
                let plain1 = c1.get(1).unwrap().as_str();
                let plain2 = c2.get(1).unwrap().as_str();
                
                if plain1 != plain2 {
                    panic!("Text structure differs between inputs");
                }
                result.push_str(plain1);
                
                // Handle colored content if present
                if let (Some(color1), Some(color2)) = (c1.get(3), c2.get(3)) {
                    let content1 = color1.as_str();
                    let content2 = color2.as_str();
                    
                    if content1 == content2 {
                        result.push_str(content1);
                    } else {
                        result.push_str(&format!("[{}|{}]", content1, content2));
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

    // Convert to Vec<Item> and sort by rank
    let mut mats: Vec<Item> = item_map
        .into_iter()
        .map(|(name, (count, rank))| Item { name, count, rank })
        .collect();
    
    // Sort by rank in ascending order
    mats.sort_by(|a, b| a.rank.cmp(&b.rank));

    Materials {
        mats,
        cost: total_cost,
    }
}