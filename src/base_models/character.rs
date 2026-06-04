use std::cmp::Ordering;

use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
pub struct Character {
    // #[serde(rename = "Name")]
    pub name: String,
    // #[serde(rename = "Desc")]
    pub desc: String,
    // #[serde(rename = "CharaInfo")]
    pub chara_info: CharaInfo,
    // #[serde(rename = "Weapon")]
    pub weapon: String,
    // #[serde(rename = "Rarity")]
    pub rarity: String,
    // #[serde(rename = "StatsModifier")]
    pub stats_modifier: StatsModifier,
    // #[serde(rename = "Skills")]
    pub skills: Vec<Skill>,
    // #[serde(rename = "Passives")]
    pub passives: Vec<Passive>,
    // #[serde(rename = "Constellations")]
    pub constellations: Vec<Constellation>,
    // #[serde(rename = "Materials")]
    pub materials: Materials
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
pub struct CharaInfo {
    // #[serde(rename = "Vision")]
    pub vision: String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
pub struct StatsModifier {
    // #[serde(rename = "Ascension")]
    pub ascension: Vec<serde_json::Value>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
pub struct Skill {
    // #[serde(rename = "Name")]
    pub name: String,
    // #[serde(rename = "Desc")]
    pub desc: String,
    // #[serde(rename = "Promote")]
    pub promote: Promote,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub special: Option<String>,
}

pub trait HasDescriptionRef {
    fn description(&self) -> &String;
}

impl HasDescriptionRef for Skill {
    fn description(&self) -> &String {
        if let Some(desc) = &self.special {
            desc
        } else {
            &self.desc
        }
    } 
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
pub struct Promote {
    #[serde(rename = "0")]
    pub n0: SkillStatBreakdown,
    #[serde(rename = "9")]
    pub n9: Option<SkillStatBreakdown>,
    #[serde(rename = "12")]
    pub n12: Option<SkillStatBreakdown>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
pub struct SkillStatBreakdown {
    // #[serde(rename = "Level")]
    pub level: i64,
    // #[serde(rename = "Desc")]
    pub desc: Vec<String>,
    // #[serde(rename = "Param")]
    pub param: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Unlock {
    Single(i64),
    Multiple(Vec<i64>),
}

impl Default for Unlock {
    fn default() -> Self {
        Unlock::Single(0)
    }
}

impl PartialOrd for Unlock {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Unlock {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            // Compare two Single values
            (Unlock::Single(a), Unlock::Single(b)) => a.cmp(b),
            
            // Single always comes before Multiple
            (Unlock::Single(_), Unlock::Multiple(_)) => Ordering::Less,
            (Unlock::Multiple(_), Unlock::Single(_)) => Ordering::Greater,
            
            // Compare two Multiple values - you can define this logic as needed
            // For example, compare by first element, then length
            (Unlock::Multiple(a), Unlock::Multiple(b)) => {
                match (a.first(), b.first()) {
                    (Some(a_first), Some(b_first)) => {
                        a_first.cmp(b_first).then_with(|| a.len().cmp(&b.len()))
                    }
                    (Some(_), None) => Ordering::Greater,
                    (None, Some(_)) => Ordering::Less,
                    (None, None) => Ordering::Equal,
                }
            }
        }
    }
}

// Implement Eq since we have PartialEq and Ord
impl Eq for Unlock {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
pub struct Passive {
    // #[serde(rename = "Name")]
    pub name: String,
    // #[serde(rename = "Desc")]
    pub desc: String,
    // #[serde(rename = "Unlock")]
    pub unlock: Unlock
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
pub struct Constellation {
    // #[serde(rename = "Name")]
    pub name: String,
    // #[serde(rename = "Desc")]
    pub desc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub special: Option<String>
}

// define "has desc" trait to reuse cleaning function for passive & constellation:
pub trait HasDescription {
    fn description(&mut self) -> &mut String;
}

impl HasDescription for Passive {
    fn description(&mut self) -> &mut String {
        &mut self.desc
    }
}

impl HasDescription for Constellation {
    fn description(&mut self) -> &mut String {
        if let Some(desc) = &mut self.special {
            desc
        } else {
            &mut self.desc
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
pub struct Materials {
    // #[serde(rename = "Ascensions")]
    pub ascensions: Vec<AscensionORTalent>,
    // #[serde(rename = "Talents")]
    pub talents: Vec<Vec<AscensionORTalent>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
pub struct AscensionORTalent {
    // #[serde(rename = "Mats")]
    pub mats: Vec<Mat>,
    // #[serde(rename = "Cost")]
    pub cost: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
pub struct Mat {
    // #[serde(rename = "Name")]
    pub name: String,
    // #[serde(rename = "Id")]
    pub id: i64,
    // #[serde(rename = "Count")]
    pub count: i64,
    // #[serde(rename = "Rank")]
    pub rank: i64,
}