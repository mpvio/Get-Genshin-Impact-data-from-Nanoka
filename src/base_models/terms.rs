use std::collections::BTreeMap;

use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
pub struct Terms {
    // https://api.hakush.in/gi/5.8.50/en/hyperlink.json
    // #[serde(rename = "Name")]
    pub name: String,
    // #[serde(rename = "Desc")]
    pub desc: String,
}

pub type TermMap = BTreeMap<String, Terms>;