use std::collections::HashMap;
use std::fs::{self, create_dir_all};
use std::path::Path;
use std::{fs::File, io::{self, BufReader, Seek, SeekFrom}};

// use crate::other_helper_funcs::python_commune::{compare_via_python, CleanDiffs};
use crate::{hakushin_lists::MinimalNameMap, parsed_models::ParsedCharacter};
use crate::helper_funcs::Parsed;
use difflib::sequencematcher::{SequenceMatcher};
use diffx_core::{DiffResult};
use serde_json::{Value, json};
use serde_json_diff::Difference;
use serde::{Serialize, Deserialize};

fn write_diff_to_file(diffs: &Difference, name: &String, list: bool) -> String {
    let date = chrono::Local::now().format("%y-%m-%d");
    let folders = if !list {"changes"} else {"list_changes"};
    create_dir_all(folders).unwrap();
    let nname = match name.strip_suffix(".json") {
        Some(res) => String::from(res),
        None => name.clone(),
    };
    let base_title = format!("{folders}/{nname}_{date}.json");
    //println!("{}", base_title);
    
    let mut counter = 0;
    let mut title = base_title.clone();
    
    // Find the first available filename
    while Path::new(&title).exists() {
        counter += 1;
        title = format!("changes/{nname}_{date} ({counter}).json");
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

async fn _compare_items<T: serde::Serialize>(old: T, new: T, name: &String) -> (bool, Option<String>) {
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

// trying diffx & difflib
fn compare_diffx<T: Serialize>(old: &T, new: &T) -> HashMap<String, String> {
    let old_value = serde_json::to_value(old);
    let new_value = serde_json::to_value(new);
    let mut diff_map = HashMap::<String, String>::new();

    match (old_value, new_value) {
        (Ok(ov), Ok(nv)) => {
            let result = diffx_core::diff(&ov, &nv, None);
            if let Ok(diffs) = result {
                // println!("DIFFX");
                for diff in diffs {
                    let (key, val) = format_for_difflib(&diff);
                    diff_map.insert(key, val);
                }
                // println!("{:#?}", diff_vec);
            }
        },
        _ => {},
    }

    diff_map
}

fn format_for_difflib(diff: &DiffResult) -> (String, String) {
    match diff {
        DiffResult::Added(location, value) => (format!("{location}"), format!("+ {value}")),
        DiffResult::Removed(location, value) => (format!("{location}"), format!("- {value}")),
        DiffResult::Modified(location, old, new) => {
            let diff = difflib_calc(old, new);
            (format!("{location}"), format!("~ {diff}"))
        },
        DiffResult::TypeChanged(location, old, new) => {
            let diff = difflib_calc(old, new);
            (format!("{location}"), format!("# {diff}"))
        },
    }
}

fn difflib_calc(old: &Value, new: &Value) -> String {
    let old_string = serde_json::to_string_pretty(old).unwrap();
    let new_string = serde_json::to_string_pretty(new).unwrap();

    let mut matcher = SequenceMatcher::new(&old_string, &new_string);
    let opcodes = matcher.get_opcodes();
    let mut final_line = "".to_string();

    for op in opcodes {
        let old_slice = &old_string[op.first_start..op.first_end];
        let new_slice = &new_string[op.second_start..op.second_end];

        match op.tag.as_str() {
            "equal" => {
                let to_add = format!("{old_slice}");
                final_line.push_str(&to_add);
            }
            "delete" => {
                let to_add = format!("-<{old_slice}>");
                final_line.push_str(&to_add);
            }
            "insert" => {
                let to_add = format!("+<{new_slice}>");
                final_line.push_str(&to_add);
            }
            "replace" => {
                // Show old lines removed then new lines added
                let to_add = format!("<{old_slice} -> {new_slice}>");
                final_line.push_str(&to_add);
            }
            _ => {},
        }
    }
    final_line
}

// end diffx &difflib test

async fn compare_and_write<T: Serialize> (file: &mut File, old: &T, current: &T, name: &String, _title: &String) -> Vec<String>{
    let mut outcomes = Vec::<String>::new();

    // DIFFX & DIFFLIB
    let differences = compare_diffx(old, current);
    if differences.len() == 0 {
        // no changes
        outcomes.push(format!("{name} unchanged."));
    } else {
        // changes
        let write_result = write_item_to_file(file, current, name, true);
        outcomes.push(write_result);
        // write difference to file
        let raw_name = match name.strip_suffix(".json") {
            Some(name) => name,
            None => &name
        };
        outcomes.push(write_diff_to_file_py(&differences, raw_name, false));
    }

    // OLD IMPLEMENTATION
    // let result = compare_via_python(old, current);
    // let (map, success) = match result {
    //     Ok(res) => {
    //         (res, true)
    //     },
    //     Err(_) => {
    //         (HashMap::<String, CleanDiffs>::new(), false)
    //     },
    // };

    // if success {
    //     // can use python's diff file
    //     println!("PYTHON DIFF");
    //     let raw_name = match name.strip_suffix(".json") {
    //         Some(name) => name,
    //         None => &name
    //     };
        
    //     if map.len() > 0 {
    //         // a change happened
    //         let write_result = write_item_to_file(file, current, name, true);
    //         outcomes.push(write_result);
    //         // write difference to file
    //         outcomes.push(write_diff_to_file_py(&map, raw_name, false));
    //     } else {
    //         // nothing changed, so no need to update anything
    //         outcomes.push(format!("{name} unchanged."));
    //     }
    // } else {
    //     // use rust's diff function
    //     println!("RUST DIFF");
    //     let (updated, update_result) = compare_items(old, current, name).await;
    //     if updated {
    //         let write_result = write_item_to_file(file, current, title, true);
    //         outcomes.push(write_result);
    //     }
    //     if let Some(res) = update_result {
    //         outcomes.push(res);
    //     }
    //     if outcomes.is_empty() {
    //         outcomes.push(format!("{title} unchanged."));
    //     }
    // }
    outcomes
}

fn write_diff_to_file_py<T: Serialize>(diffs: &HashMap<String, T>, name: &str, list: bool) -> String {
    let date = chrono::Local::now().format("%y-%m-%d");
    let folders = if !list {"changes"} else {"list_changes"};
    create_dir_all(folders).unwrap();
    
    let mut file_name = format!("{name} {date}");
    let mut base_title = format!("{folders}/{file_name}.json");
    let mut counter = 0;

    while Path::new(&base_title).exists() {
        counter += 1;
        file_name = format!("{name} {date} ({counter})");
        base_title = format!("{folders}/{file_name}.json");
    }

    if let Ok(file) = File::options()
        .write(true)
        .truncate(true)
        .create_new(true) // create_new -> atomicity of write operation
        .open(&base_title) {
            match serde_json::to_writer_pretty(file, &diffs) {
                Ok(_) => {
                    println!("{base_title} created.");
                },
                Err(e) => {
                    println!("Error writing to {base_title}: {e:#?}");
                }
            }
    } else {
        println!("Failed to create {base_title}.")
    }

    format!("{file_name} created.")
}

pub async fn check_and_write(_category: &str, item: Parsed) -> Vec<String> {
    let folders = format!("results/{_category}");
    create_dir_all(&folders).unwrap();
    let mut all_outcomes = Vec::<String>::new();

    let display_name = &format!("{}.json", item.name());
    let title = format!("{}/{}", folders, display_name);

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
                // compare_diffx(&content, &item);
                match (content, item) {
                    (Parsed::C(old), Parsed::C(current)) => {
                        compare_and_write(&mut file, &old, &current, display_name, &title).await
                    },
                    (Parsed::W(old), Parsed::W(current)) => {
                        compare_and_write(&mut file, &old, &current, display_name, &title).await
                    },
                    (Parsed::A(old), Parsed::A(current)) => {
                        compare_and_write(&mut file, &old, &current, display_name, &title).await
                    },
                    (Parsed::T(old), Parsed::T(current)) => {
                        compare_and_write(&mut file, &old, &current, display_name, &title).await
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
                
                v.push(write_item_to_file(&mut file, &item, display_name, false));
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
    let _ = file.set_len(0); // remove contents of file before writing to it
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

    // remove '/'s from result so just {name}.json is displayed on screen (via simpler_title)
    // let temp: Vec<&str> = result.split("/").collect();
    // let simpler_title = String::from(*temp.last().unwrap());
    // simpler_title
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

pub fn get_shortlist() -> Vec<String> {
    let filename = "shortlist.txt";
    if let Ok(contents) = fs::read_to_string(filename) {
        return contents.split_ascii_whitespace().map(|s| s.to_string()).collect();
    }
    Vec::new()
}

pub fn get_latest_boss() -> i64 {
    let filename = "latestboss.txt";
    let default: i64 = 113080;
    if let Ok(contents) = fs::read_to_string(filename) {
        return contents.trim().parse::<i64>().unwrap_or(default);
    }
    default
}