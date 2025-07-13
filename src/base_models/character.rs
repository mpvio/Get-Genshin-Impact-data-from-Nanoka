use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Character {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Desc")]
    pub desc: String,
    #[serde(rename = "CharaInfo")]
    pub chara_info: CharaInfo,
    #[serde(rename = "Weapon")]
    pub weapon: String,
    #[serde(rename = "Rarity")]
    pub rarity: String,
    #[serde(rename = "StatsModifier")]
    pub stats_modifier: StatsModifier,
    #[serde(rename = "Skills")]
    pub skills: Vec<Skill>,
    #[serde(rename = "Passives")]
    pub passives: Vec<Passive>,
    #[serde(rename = "Constellations")]
    pub constellations: Vec<Constellation>,
    #[serde(rename = "Materials")]
    pub materials: Materials
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CharaInfo {
    #[serde(rename = "Vision")]
    pub vision: String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatsModifier {
    #[serde(rename = "Ascension")]
    pub ascension: Vec<serde_json::Value>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Skill {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Desc")]
    pub desc: String,
    #[serde(rename = "Promote")]
    pub promote: Promote,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Promote {
    #[serde(rename = "0")]
    pub n0: SkillStatBreakdown,
    #[serde(rename = "9")]
    pub n9: SkillStatBreakdown,
    #[serde(rename = "12")]
    pub n12: SkillStatBreakdown
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillStatBreakdown {
    #[serde(rename = "Level")]
    pub level: i64,
    #[serde(rename = "Desc")]
    pub desc: Vec<String>,
    #[serde(rename = "Param")]
    pub param: Vec<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Passive {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Desc")]
    pub desc: String,
    #[serde(rename = "Unlock")]
    pub unlock: i64
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Constellation {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Desc")]
    pub desc: String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Materials {
    #[serde(rename = "Ascensions")]
    pub ascensions: Vec<AscensionORTalent>,
    #[serde(rename = "Talents")]
    pub talents: Vec<Vec<AscensionORTalent>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AscensionORTalent {
    #[serde(rename = "Mats")]
    pub mats: Vec<Mat>,
    #[serde(rename = "Cost")]
    pub cost: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mat {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "Count")]
    pub count: i64,
    #[serde(rename = "Rank")]
    pub rank: i64,
}