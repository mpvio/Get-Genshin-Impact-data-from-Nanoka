use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::weapon::Materials;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParsedWeapon {
    pub name: String,
    #[serde(rename = "type")]
    pub weapon_type: String,
    pub rarity: i8,
    pub substat: String,
    pub effect: String,
    pub materials: Materials
}