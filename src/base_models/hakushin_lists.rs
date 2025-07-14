use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap};

#[derive(Debug, Serialize, Deserialize)]
pub struct JustTheName {
    #[serde(rename = "EN")]
    pub en: String //valid for characters, weapons, tcg cards
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MinimalArtifact {
    pub set: BTreeMap<String, MinimalArtifactSet>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MinimalArtifactSet {
    pub name: JustTheName,
    pub desc: JustTheName
}


pub type MinimalNameMap = BTreeMap<String, JustTheName>;
pub type MinimalWeaponMap = BTreeMap<String, JustTheName>;
pub type MinimalCardMap = BTreeMap<String, JustTheName>;
pub type MinimalArtifactMap = BTreeMap<String, MinimalArtifact>;