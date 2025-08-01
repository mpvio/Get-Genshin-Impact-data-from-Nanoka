use std::collections::BTreeMap;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::character::Constellation;
use crate::character::Passive;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParsedCharacter {
    pub name: String,
    pub vision: String,
    pub weapon: String,
    pub rarity: String,
    pub ascension_stat: String,
    pub skills: Vec<ParsedSkill>,
    pub passives: Vec<Passive>,
    pub constellations: Vec<Constellation>,
    pub ascension_mats: Vec<ParsedMaterial>,
    pub talent_mats: Vec<ParsedMaterial>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term_descs: Option<BTreeMap<String, String>>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParsedSkill {
    pub name: String,
    pub desc: String,
    pub parameters: Vec<String>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParsedMaterial {
    pub name: String,
    pub amount: i64
}