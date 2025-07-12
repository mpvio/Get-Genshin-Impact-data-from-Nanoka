use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::collections::{BTreeMap};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Weapon {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "WeaponType")]
    pub weapon_type: String,
    #[serde(rename = "WeaponProp")]
    pub weapon_prop: Vec<SubStat>,
    #[serde(rename = "Rarity")]
    pub rarity: i8,
    #[serde(rename = "Refinement")]
    pub refinement: Refinement,
    #[serde(rename = "Materials")]
    pub materials: BTreeMap<String, Materials>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeaponProp {
    #[serde(rename = "1")]
    pub sub_stat: SubStat
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubStat {
    #[serde(rename = "propType")]
    pub prop_type: String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Refinement {
    #[serde(rename = "1")]
    pub r1: RefinementLevel,
    #[serde(rename = "5")]
    pub r5: RefinementLevel
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RefinementLevel {
    #[serde(rename = "Desc")]
    pub desc: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Materials {
    #[serde(rename = "Mats")]
    pub mats: Vec<Item>,
    #[serde(rename = "Cost")]
    pub cost: u32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Count")]
    pub count: u32,
    #[serde(rename = "Rank")]
    pub rank: u32
}