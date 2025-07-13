use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParsedArtifact {
    pub name: String,
    #[serde(rename = "2pc")]
    pub two: String,
    #[serde(rename = "4pc")]
    pub four: String
}