use crate::{
    base_models::hakushin_lists::{MinimalArtifactMap, MinimalNameMap}, 
    other_helper_funcs::read_and_write_funcs::{write_character_list_to_file, write_list_to_file}
};

pub async fn get_minimals() -> Option<MinimalArtifactMap> {
    println!("CHARACTERS:");
    get_minimal_character_list().await;
    println!("\nWEAPONS:");
    get_minimal_weapons().await;
    println!("\nCARDS:");
    get_minimal_cards().await;
    println!("\nARTIFACTS:");
    return get_minimal_artifacts().await;
}

async fn get_minimal_character_list() {
    let url = "https://api.hakush.in/gi/data/character.json";
    let chars_per_row = 5;

    if let Ok(response) = reqwest::get(url).await {
        if let Ok(map) = response.json::<MinimalNameMap>().await {
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
            write_character_list_to_file(&map);
        }
    }
}

async fn get_minimal_weapons() {
    let url = "https://api.hakush.in/gi/data/weapon.json";
    let chars_per_row = 5;

    if let Ok(response) = reqwest::get(url).await {
        if let Ok(map) = response.json::<MinimalNameMap>().await {
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
            write_list_to_file("weapon", &map);
        }
    }
}

async fn get_minimal_cards() {
    let url = "https://api.hakush.in/gi/data/gcg.json";
    let chars_per_row = 5;

    if let Ok(response) = reqwest::get(url).await {
        if let Ok(map) = response.json::<MinimalNameMap>().await {
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
            write_list_to_file("gcg", &map);
        }
    }
}

async fn get_minimal_artifacts() -> Option<MinimalArtifactMap> {
    let url = "https://api.hakush.in/gi/data/artifact.json";
    let chars_per_row = 5;

    if let Ok(response) = reqwest::get(url).await {
        if let Ok(mut map) = response.json::<MinimalArtifactMap>().await {
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
            write_list_to_file("artifact", &map);
            return Some(map);
        }
    }

    None
}
