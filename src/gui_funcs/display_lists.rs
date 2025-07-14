use crate::{
    api_funcs::get_minimal_lists::get_minimals, 
    base_models::hakushin_lists::{MinimalArtifactMap, MinimalNameMap}
};

#[derive(Clone)]
pub struct ItemNames {
    pub key: String,
    pub name: String
}
impl Default for ItemNames {
    fn default() -> Self {
        Self { key: Default::default(), name: Default::default() }
    }
}
pub async fn get_names () -> (Vec<ItemNames>, Vec<ItemNames>, Vec<ItemNames>, Vec<ItemNames>, Option<MinimalArtifactMap>) {
    let (
        chars, 
        weaps, 
        _cards, 
        arts
    ) = get_minimals(true).await;
    
    let characters = parse_just_names(&chars);
    let weapons = parse_just_names(&weaps);
    let cards = parse_just_names(&_cards);
    let artifacts = parse_artifacts(&arts);

    (characters, weapons, cards, artifacts, arts)
}

fn parse_just_names(option: &Option<MinimalNameMap>) -> Vec<ItemNames> {
    let mut display: Vec<ItemNames> = Vec::<ItemNames>::new();
    match option {
        Some(map) => {
            for (key, name) in map {
                let item_name = ItemNames {
                    key: key.to_string(),
                    name: name.en.clone()
                };
                display.push(item_name);
            }
        },
        None => {},
    }
    display    
}

fn parse_artifacts(option: &Option<MinimalArtifactMap>) -> Vec<ItemNames> {
    let mut display: Vec<ItemNames> = Vec::<ItemNames>::new();
    match option {
        Some(map) => {
            for (key, value) in map {
                let item_name = ItemNames {
                    key: key.to_string(),
                    name: value.set.clone().first_entry().unwrap().get().name.en.clone()
                };
                display.push(item_name);
            }
        },
        None => {},
    }
    display
}