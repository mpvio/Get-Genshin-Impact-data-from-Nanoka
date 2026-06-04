use crate::{
    base_models::hakushin_lists::{MinimalArtifactMap, MinimalNameMap}, 
    other_helper_funcs::read_and_write_funcs::write_list_to_file
};

pub async fn get_minimals(with_ui: bool, version: &String) -> (Option<MinimalNameMap>,  Option<MinimalNameMap>, Option<MinimalNameMap>, Option<MinimalArtifactMap>) {
    if !with_ui {println!("CHARACTERS:");}
    let characters = get_minimal_characters(with_ui, version).await;
    if !with_ui {println!("\nWEAPONS:");}
    let weapons = get_minimal_weapons(with_ui, version).await;
    if !with_ui {println!("\nCARDS:");}
    let cards = get_minimal_cards(with_ui, version).await;
    if !with_ui {println!("\nARTIFACTS:");}
    let artifacts = get_minimal_artifacts(with_ui, version).await;
    
    (characters, weapons, cards, artifacts)
}

async fn get_minimal_characters(with_ui: bool, version: &String) -> Option<MinimalNameMap> {
    let url = format!("https://static.nanoka.cc/gi/{version}/character.json");
    let chars_per_row = 5;

    if let Ok(response) = reqwest::get(url).await {
        // println!("{response:#?}");
        if let Ok(map) = response.json::<MinimalNameMap>().await {
            // println!("{map:#?}");
            if !with_ui {
                let mut count = 0;
                for (key, value) in &map {
                    print!("{:<10}: {:<20} ", key, value.en);
                    count += 1;
                    if count % chars_per_row == 0 {
                        println!(); //new line after every N characters
                    }
                }
                if count % chars_per_row != 0 {
                    println!(); //forcibly switch to new line if total characters isn't a multiple of N
                }
            }
            write_list_to_file("character", &map).await;
            return Some(map);
        }
    }

    None
}

async fn get_minimal_weapons(with_ui: bool, version: &String) -> Option<MinimalNameMap> {
    let url = format!("https://static.nanoka.cc/gi/{version}/weapon.json");
    let chars_per_row = 5;

    if let Ok(response) = reqwest::get(url).await {
        if let Ok(map) = response.json::<MinimalNameMap>().await {
            if !with_ui {
                let mut count = 0;
                for (key, value) in &map {
                    print!("{:<10}: {:<20} ", key, value.en);
                    count += 1;
                    if count % chars_per_row == 0 {
                        println!(); //new line after every N characters
                    }
                }
                if count % chars_per_row != 0 {
                    println!(); //forcibly switch to new line if total characters isn't a multiple of N
                }
            }
            write_list_to_file("weapon", &map).await;
            return Some(map);
        }
    }

    None
}

async fn get_minimal_cards(with_ui: bool, version: &String) -> Option<MinimalNameMap> {
    let url = format!("https://static.nanoka.cc/gi/{version}/gcg.json");
    let chars_per_row = 5;

    if let Ok(response) = reqwest::get(url).await {
        if let Ok(map) = response.json::<MinimalNameMap>().await {
            if !with_ui {
                let mut count = 0;
                for (key, value) in &map {
                    let name = if key == "1506" {
                        "Wanderer" // implement this better
                    } else {
                        &value.en
                    };
                    print!("{:<10}: {:<20} ", key, name);
                    count += 1;
                    if count % chars_per_row == 0 {
                        println!(); //new line after every N characters
                    }
                }
                if count % chars_per_row != 0 {
                    println!(); //forcibly switch to new line if total characters isn't a multiple of N
                }
            }
            write_list_to_file("gcg", &map).await;
            return Some(map);
        }
    }

    None
}

async fn get_minimal_artifacts(with_ui: bool, version: &String) -> Option<MinimalArtifactMap> {
    let url = format!("https://static.nanoka.cc/gi/{version}/artifact.json");
    let chars_per_row = 5;

    if let Ok(response) = reqwest::get(url).await {
        if let Ok(mut map) = response.json::<MinimalArtifactMap>().await {
            if !with_ui {
                let mut count = 0;
                for (key, value) in &mut map {
                    print!("{:<10}: {:<20} ", key, value.set.first_entry().unwrap().get().name.en);
                    count += 1;
                    if count % chars_per_row == 0 {
                        println!(); //new line after every N characters
                    }
                }
                if count % chars_per_row != 0 {
                    println!(); //forcibly switch to new line if total characters isn't a multiple of N
                }
            }
            write_list_to_file("artifact", &map).await;
            return Some(map);
        }
    }

    None
}
