use std::collections::BTreeMap;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::tcg_cards::TalentTCGCost;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ParsedCard {
    Character(ParsedCharacterTCG),
    Talent(ParsedTalentTCG)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ParsedTalent {
    Character(BTreeMap<String, ParsedTCGTalent>),
    Talent(ParsedTCGTalent)
}

impl ParsedTalent {
    pub fn character(&self) -> Option<&BTreeMap<String, ParsedTCGTalent>> {
        match self {
            ParsedTalent::Character(btree_map) => Some(btree_map),
            _ => None,
        }
    }
    pub fn talent(&self) -> Option<&ParsedTCGTalent> {
        match self {
            ParsedTalent::Talent(talent) => Some(talent),
            _ => None
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedCharacterTCG {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Type")]
    pub card_type: String,
    #[serde(rename = "Hp")]
    pub hp: u8,
    #[serde(rename = "Cost")]
    pub cost: u8,
    #[serde(rename = "Tag")]
    pub tag: Vec<String>,
    #[serde(rename = "Talents")]
    pub talents: BTreeMap<String, ParsedTCGTalent>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedTCGTalent {
    #[serde(rename = "Name")]
    pub name: String, 
    #[serde(rename = "Desc")]
    pub desc: String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedTalentTCG {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Type")]
    pub card_type: String,
    #[serde(rename = "Tag")]
    pub tag: Vec<String>,
    #[serde(rename = "Cost")]
    pub cost: Vec<TalentTCGCost>,
    #[serde(rename = "Talents")]
    pub talents: ParsedTCGTalent,
}