use std::{fs::File, io::{self, BufReader, Seek, SeekFrom}};

use crate::{hakushin_lists::MinimalCharacterMap, parsed_character::ParsedCharacter};
use crate::helper_funcs::Parsed;

use serde_json::json;
use serde_json_diff::Difference;

fn write_diff_to_file(character : &Difference, name: &String){
    let date = chrono::Local::now().format("%d-%m");
    let title = format!("{name}_{date}.json");

    println!("{character:#?}");

    if let Ok(file) = File::options()
    //.read(true)
    .write(true)
    .truncate(true)
    .create(true)
    .open(&title) {
        //let reader = BufReader::new(&file);
        let write_outcome = serde_json::to_writer_pretty(file, &character);
        match write_outcome {
            Ok(_) => {
                println!("{title} created.");
            },
            Err(_) => {
                println!("Error with {title}.");
            },
        }
    }
}

async fn compare_items<T: serde::Serialize>(old: T, new: T, name: &String) -> bool {
    match serde_json_diff::values(json!(old), json!(new)) {
        Some(diffs) => {
            println!("found diff!");
            write_diff_to_file(&diffs, &name);
            true
        },
        None => false,
    }
}

async fn compare_characters(old_char : &ParsedCharacter, new_char : &ParsedCharacter) -> bool {
    let old_char_json = json!(old_char);
    let new_char_json = json!(new_char);

    match serde_json_diff::values(old_char_json, new_char_json) {
        Some(differences) => {
            //println!("{differences:#?}");
            write_diff_to_file(&differences, &old_char.name);
            true
        },
        None => {
            false
        },
    }
}

pub async fn check_and_write(_category: &str, item: Parsed) {
    let title = format!("{}.json", item.name());
    if let Ok(mut file) = File::options()
    .read(true)
    .write(true)
    .create(true)
    .open(&title) {
        let reader = BufReader::new(&file);
        let old_content: Result<Parsed, serde_json::Error> = serde_json::from_reader(reader);
        match old_content {
            Ok(content) => {
                match (content, item) {
                    (Parsed::C(old), Parsed::C(current)) => {
                        let updated = compare_characters(&old, &current).await;
                        if updated {
                            write_item_to_file(&mut file, &current, &title, true);
                        }
                    },
                    (Parsed::W(old), Parsed::W(current)) => {
                        let updated = compare_items(&old, &current, &current.name).await;
                        if updated {
                            write_item_to_file(&mut file, &current, &title, true);
                        }
                    },
                    _ => {}
                }
            },
            Err(_) => {
                // file didn't exist
                write_item_to_file(&mut file, &item, &title, false);
            },
        }
    }
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

pub fn write_character_list_to_file(map: &MinimalCharacterMap){
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

pub fn write_list_to_file<T: serde::Serialize>(name: &'static str, map: &T) {
    let txt = format!("{name}.json");
    let path = txt.as_str();
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

pub fn write_item_to_file<T: serde::Serialize>(file: &mut File, item: &T, title: &String, update: bool) {
    let _ = file.seek(SeekFrom::Start(0));
    match serde_json::to_writer_pretty(file, &item) {
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