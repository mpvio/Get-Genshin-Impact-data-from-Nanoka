use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap};

#[derive(Debug, Serialize, Deserialize)]
pub struct MinimalCharacter {
    #[serde(rename = "EN")]
    pub en: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MinimalWeapon {
    #[serde(rename = "EN")]
    pub en: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MinimalCard {
    #[serde(rename = "EN")]
    pub en: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MinimalArtifact {
    pub set: BTreeMap<String, MinimalArtifactSet>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MinimalArtifactSet {
    pub name: MinimalArtifactName,
    pub desc: MinimalArtifactDesc
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MinimalArtifactName {
    #[serde(rename = "EN")]
    pub en: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MinimalArtifactDesc {
    #[serde(rename = "EN")]
    pub en: String
}


pub type MinimalCharacterMap = BTreeMap<String, MinimalCharacter>;
pub type MinimalWeaponMap = BTreeMap<String, MinimalWeapon>;
pub type MinimalCardMap = BTreeMap<String, MinimalCard>;
pub type MinimalArtifactMap = BTreeMap<String, MinimalArtifact>;