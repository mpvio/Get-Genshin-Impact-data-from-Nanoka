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

pub fn get_custom_name(item: &ItemNames) -> String {
    get_custom_name_from_id(item.key.as_str(), &item.name)
}

pub fn get_custom_name_from_id<'a>(id: &'a str, name: &'a String) -> String {
    let new_name = match id {
        "10000007-2" | "10000005-2" => "Traveler Pyro",
        "10000007-3" | "10000005-3" => "Traveler Hydro",
        "10000007-4" | "10000005-4" => "Traveler Anemo",
        "10000007-6" | "10000005-6" => "Traveler Geo",
        "10000007-7" | "10000005-7" => "Traveler Electro",
        "10000007-8" | "10000005-8" => "Traveler Dendro",
        "1506" => "Wanderer", // tcg card name
        _ => name.as_str(),
    };
    let gender = if id.contains("10000007") {
        " (F)"
    } else if id.contains("10000005") {
        " (M)"
    } else {
        ""
    };
    let mut name_string = new_name.to_owned();
    name_string.push_str(gender);
    name_string
}

pub fn filter_items<'a>(
    items: &'a [ItemNames],
    search_term: &'a str,
) -> impl Iterator<Item = &'a ItemNames> + 'a {
    let search_lower = search_term.to_lowercase();
    
    items.iter().filter(move |item| {
        search_term.is_empty() ||
        get_custom_name(item).to_lowercase().contains(&search_lower) ||
        item.key.contains(search_term)
    })
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