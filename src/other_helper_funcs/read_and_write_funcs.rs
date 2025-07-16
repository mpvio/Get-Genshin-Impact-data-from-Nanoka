use std::fs::create_dir_all;
use std::path::Path;
use std::{fs::File, io::{self, BufReader, Seek, SeekFrom}};

use crate::{hakushin_lists::MinimalNameMap, parsed_models::ParsedCharacter};
use crate::helper_funcs::Parsed;

use serde_json::json;
use serde_json_diff::Difference;
use serde::{Serialize, Deserialize};

fn write_diff_to_file(diffs: &Difference, name: &String, list: bool) -> String {
    let date = chrono::Local::now().format("%y-%m-%d");
    let folders = if !list {"changes"} else {"list_changes"};
    create_dir_all(folders).unwrap();
    let base_title = format!("{folders}/{name}_{date}.json");
    //println!("{}", base_title);
    
    let mut counter = 0;
    let mut title = base_title.clone();
    
    // Find the first available filename
    while Path::new(&title).exists() {
        counter += 1;
        title = format!("changes/{name}_{date} ({counter}).json");
        //println!("{}", title);
    }

    if let Ok(file) = File::options()
        .write(true)
        .truncate(true)
        .create_new(true)  // Using create_new to ensure atomicity
        .open(&title) 
    {
        let write_outcome = serde_json::to_writer_pretty(file, &diffs);
        match write_outcome {
            Ok(_) => {
                println!("{title} created.");
            },
            Err(e) => {
                println!("Error writing to {title}: {}", e);
            },
        }
    } else {
        println!("Failed to create file {}", title);
    }
    title
}

async fn compare_items<T: serde::Serialize>(old: T, new: T, name: &String) -> (bool, Option<String>) {
    match serde_json_diff::values(json!(old), json!(new)) {
        Some(diffs) => {
            //println!("found diff!");
            let result = write_diff_to_file(&diffs, &name, false);
            (true, Some(result))
        },
        None => (false, None),
    }
}

async fn compare_characters(old_char : &ParsedCharacter, new_char : &ParsedCharacter) -> bool {
    let old_char_json = json!(old_char);
    let new_char_json = json!(new_char);

    match serde_json_diff::values(old_char_json, new_char_json) {
        Some(differences) => {
            //println!("{differences:#?}");
            write_diff_to_file(&differences, &old_char.name, false);
            true
        },
        None => {
            false
        },
    }
}

async fn compare_and_write<T: Serialize> (file: &mut File, old: &T, current: &T, name: &String, title: &String) -> Vec<String>{
    let mut outcomes = Vec::<String>::new();
    let (updated, update_result) = compare_items(old, current, name).await;
    if updated {
        let write_result = write_item_to_file(file, current, title, true);
        outcomes.push(write_result);
    }
    if let Some(res) = update_result {
        outcomes.push(res);
    }
    outcomes
}

pub async fn check_and_write(_category: &str, item: Parsed) -> Vec<String> {
    let folders = format!("results/{_category}");
    create_dir_all(&folders).unwrap();
    let mut all_outcomes = Vec::<String>::new();

    let name = item.name();
    let title = format!("{}/{}.json", folders, &item.name());
    if let Ok(mut file) = File::options()
    .read(true)
    .write(true)
    .create(true)
    .open(&title) {
        let reader = BufReader::new(&file);
        let old_content: Result<Parsed, serde_json::Error> = serde_json::from_reader(reader);
        all_outcomes = match old_content {
            Ok(content) => {
                //let name = item.name();
                match (content, item) {
                    (Parsed::C(old), Parsed::C(current)) => {
                        compare_and_write(&mut file, &old, &current, &name, &title).await
                    },
                    (Parsed::W(old), Parsed::W(current)) => {
                        compare_and_write(&mut file, &old, &current, &name, &title).await
                    },
                    (Parsed::A(old), Parsed::A(current)) => {
                        compare_and_write(&mut file, &old, &current, &name, &title).await
                    },
                    (Parsed::T(old), Parsed::T(current)) => {
                        compare_and_write(&mut file, &old, &current, &name, &title).await
                    }
                    _ => {
                        // content & item aren't the same struct
                        Vec::<String>::new()
                    }
                }
            },
            Err(_) => {
                // file didn't exist
                let mut v = Vec::<String>::new();
                v.push(write_item_to_file(&mut file, &item, &title, false));
                v
            },
        };
    }

    all_outcomes
}

pub async fn check_and_write_to_file(character : ParsedCharacter){
    let title = format!("{}.json", character.name);
    if let Ok(mut file) = File::options()
    .read(true)
    .write(true)
    .create(true)
    .open(&title) {
        let reader = BufReader::new(&file);
        let saved_content: Result<ParsedCharacter, serde_json::Error> = serde_json::from_reader(reader);
        match saved_content {
            Ok(saved_char) => {
                let updated = compare_characters(&saved_char, &character).await;
                if updated {
                    write_character_to_file(&mut file, &character, &title, true);
                }
            },
            Err(_) => {
                //file didn't exist before
                write_character_to_file(&mut file, &character, &title, false);
            },
        }
    }
}

pub fn write_character_list_to_file(map: &MinimalNameMap){
    let path = "characters.json";
    let mut file = File::create(path).unwrap();
    let _ = file.seek(SeekFrom::Start(0));
    match serde_json::to_writer_pretty(file, &map) {
        Ok(_) => {
            println!("{path} created.");
            }
        ,
        Err(err) => {
            println!("{:#?}", err);
        },
    }
}

async fn write_list_to_file_helper<T: Serialize + for<'a> Deserialize<'a>>(mut file: File, map: &T, path: &str) {
    let _ = file.seek(SeekFrom::Start(0));
    match serde_json::to_writer_pretty(file, &map) {
        Ok(_) => {
            println!("{path} created.");
            }
        ,
        Err(err) => {
            println!("{:#?}", err);
        },
    }
}

pub async fn write_list_to_file<T: Serialize + for<'a> Deserialize<'a>>(name: &'static str, map: &T) {
    let folder = format!("lists");
    create_dir_all(&folder).unwrap();

    let txt = format!("{folder}/{name}.json");
    let path = txt.as_str();

    if let Ok(file) = File::options().read(true).write(true).create(true).open(path) {
        let reader = BufReader::new(&file);
        let old_content: Result<T, serde_json::Error> = serde_json::from_reader(reader);
        match old_content {
            Ok(content) => {
                match serde_json_diff::values(json!(content), json!(map)) {
                    Some(diffs) => {
                        write_diff_to_file(&diffs, &name.to_string(), true);
                        write_list_to_file_helper(file, map, path).await;
                    },
                    None => {
                        // no changes to save
                    },
                }
            },
            Err(_) => {
                // old version didn't exist
                write_list_to_file_helper(file, map, path).await;
            },
        }
    }
}

pub fn write_item_to_file<T: serde::Serialize>(file: &mut File, item: &T, title: &String, update: bool) -> String {
    let _ = file.seek(SeekFrom::Start(0));
    let result = match serde_json::to_writer_pretty(file, &item) {
        Ok(_) => {
            if update {
                format!("{title} updated.")
            } else {
                format!("{title} created.")
            }
        },
        Err(err) => {
            format!("{:#?}", err)
        },
    };
    println!("{}", result);
    result
}

pub fn write_character_to_file(file: &mut File, character: &ParsedCharacter, title: &String, update: bool){
    let _ = file.seek(SeekFrom::Start(0));
    match serde_json::to_writer_pretty(file, &character) {
        Ok(_) => {
            if update {
                println!("{title} updated.");
            } else {
                println!("{title} created.");
            }
        },
        Err(err) => {
            println!("{:#?}", err);
        },
    }
}

pub fn get_ids_from_user() -> String {
    let mut buffer: String = String::new();
    println!("\nEnter IDs: ");
    let stdin: io::Stdin = io::stdin();
    match stdin.read_line(&mut buffer) {
        Ok(_) => {
            buffer.trim().to_string()
        },
        Err(_) => {
            String::new()
        },
    }
}