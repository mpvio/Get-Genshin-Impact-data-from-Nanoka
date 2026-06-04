use std::collections::BTreeMap;
use regex::Regex;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::helper_funcs::clean_text_colon;
use crate::parsed_models::{ParsedTCGTalent, ParsedTalent};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TCGCardTypes {
    Character(CharacterTCG),
    Talent(TalentTCG)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Cost {
    Character(u8),
    Talent(Vec<TalentTCGCost>)
}

impl Cost {
    pub fn character(&self) -> Option<u8> {
        match self {
            Cost::Character(val) => Some(*val),
            _ => None,
        }
    }
    pub fn talent(&self) -> Option<&Vec<TalentTCGCost>> {
        match self {
            Cost::Talent(val) => Some(val),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Talent {
    Character(BTreeMap<String, TCGTalentEffect>),
    Talent(TCGTalentEffect)
}

impl Talent {
    pub fn character(&self) -> Option<BTreeMap<String, TCGTalentEffect>> {
        match self {
            Talent::Character(val) => Some(val.clone()),
            _ => None,
        }
    }
    pub fn talent(&self) -> Option<TCGTalentEffect> {
        match self {
            Talent::Talent(val) => Some(val.clone()),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
pub struct CharacterTCG {
    // #[serde(rename = "Name")]
    pub name: String,
    // #[serde(rename = "Type")]
    pub card_type: String,
    // #[serde(rename = "Hp")]
    pub hp: Option<u8>,
    // #[serde(rename = "Cost")]
    pub cost: Cost,
    // #[serde(rename = "Tag")]
    pub tag: Vec<String>,
    // #[serde(rename = "Talent")]
    pub talent: Talent,
}

impl CharacterTCG {
    pub fn get_tree(&self) -> BTreeMap<String, String> {
        let mut tree = BTreeMap::<String, String>::new();
        match &self.talent {
            Talent::Character(t) => {
                for (_, val) in t {
                    let mut partial_tree = val.get_tree();
                    tree.append(&mut partial_tree);
                }

            },
            Talent::Talent(val) => {
                let mut partial_tree = val.get_tree();
                tree.append(&mut partial_tree);   
            },
        }
        return tree;
    }
    pub fn convert(&self, tree: &BTreeMap<String, String>) -> ParsedTalent {
        match &self.talent {
            Talent::Character(map) => {
                let mut talents: BTreeMap<String, ParsedTCGTalent> = BTreeMap::<String, ParsedTCGTalent>::new();
                for (key, eff) in map {
                    let p = eff.convert(tree);
                    talents.insert(key.to_string(), p);
                }
                return ParsedTalent::Character(talents);
            },
            Talent::Talent(eff) => {
                return ParsedTalent::Talent(eff.convert(tree));
                
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TCGChildTypes {
    Number(u8),
    String(String),
    Child(Box<TCGTalentEffect>), // Use Box to avoid recursive type issues
}

impl TCGChildTypes {
    pub fn get(&self) -> String {
        match self {
            TCGChildTypes::Number(val) => val.to_string(),
            TCGChildTypes::String(val) => val.clone(),
            TCGChildTypes::Child(val) => val.name.clone(),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
pub struct TCGTalentEffect {
    // #[serde(rename = "Name")]
    pub name: String, 
    // #[serde(rename = "Desc")]
    pub desc: String, 
    // #[serde(rename = "Child")]
    pub child: Option<BTreeMap<String, TCGChildTypes>>,
}

impl TCGTalentEffect {
    pub fn get_tree(&self) -> BTreeMap<String, String> {
        let mut tree = BTreeMap::<String, String>::new();
        match &self.child {
            Some(data) => {
                for (key, value) in data {
                    tree.insert(key.to_string(), value.get());
                }
            },
            None => {},
        }
        return tree;
    }
    pub fn convert(&self, tree: &BTreeMap<String, String>) -> ParsedTCGTalent {
        // remove tags
        let unwanted_tags = Regex::new(r"<color=[^>]*>|</color>|\{SPRITE_PRESET#[^>]*\}").unwrap();        
        let without_tags = unwanted_tags.replace_all(&self.desc, "");

        // isolate $[keys]
        let val_regex = Regex::new(r"\$\[([^\]]+)\]").unwrap();
        let mut result = String::new();
        let mut last_end = 0;

        // process each capture
        for cap in val_regex.captures_iter(&without_tags) {
            let preceding_text = cap.get(0).unwrap();
            let key = cap.get(1).unwrap().as_str();

            // add text before the key to result string
            result.push_str(&without_tags[last_end..preceding_text.start()]);

            // get key value
            if let Some(value) = tree.get(&key.to_string()) {
                result.push_str(value);
            } else {
                result.push_str(&format!("[{key}]"));
            }

            // update last_end to end position of this key
            last_end = preceding_text.end();
        }

        // add remaining text to result
        result.push_str(&without_tags[last_end..]);

        // Handle newlines by replacing them with spaces
        let newline_re = Regex::new(r"\\n|\n").unwrap();
        let with_spaces = newline_re.replace_all(&result, " ");

        // Clean up multiple spaces and trim
        let space_re = Regex::new(r"\s+").unwrap();
        let final_desc = space_re.replace_all(&with_spaces, " ").trim().to_string();

        let (clean_text, _terms) = clean_text_colon(&final_desc, false);

        ParsedTCGTalent {
            name: self.name.clone(),
            desc: clean_text // not sure why technique descs aren't already clean?
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
pub struct CharacterTCGTalentChild {
    // #[serde(rename = "Name")]
    pub name: String, 
    // #[serde(rename = "Desc")]
    pub desc: String, 
    // #[serde(rename = "Child")]
    pub child: BTreeMap<String, TCGChildTypes>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
pub struct TalentTCG {
    // #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "type")]
    pub card_type: String,
    // #[serde(rename = "Tag")]
    pub tag: Vec<String>,
    // #[serde(rename = "Cost")]
    pub cost: Vec<TalentTCGCost>,
    // #[serde(rename = "Talent")]
    pub talent: BTreeMap<String, TCGTalentEffect>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
pub struct TalentTCGCost {
    // #[serde(rename = "costType")]
    pub cost_type: String,
    pub count: Option<u8>
}