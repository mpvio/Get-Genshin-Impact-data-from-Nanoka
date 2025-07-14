use egui::ViewportBuilder;
use read_and_write_funcs::get_ids_from_user;

use crate::{
    api_funcs::get_item_funcs::{query_api}, get_minimal_lists::get_minimals, gui_funcs::egui_for_lists::HakuGIApp
};

pub mod other_helper_funcs;
pub mod parsed_models;
pub mod base_models;
pub mod character_funcs;
pub mod api_funcs;
pub mod gui_funcs;

use base_models::{character, weapon, tcg_cards, hakushin_lists};
use other_helper_funcs::{helper_funcs, read_and_write_funcs};
use api_funcs::{get_minimal_lists};

// TODO: Clean up!
#[tokio::main]
async fn main() {
    show_ui().await;
}

async fn show_ui() {
    let app = HakuGIApp::new().await;
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([1100.0, 500.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Genshin Data Viewer",
        options,
        Box::new(|_cc| Ok(Box::new(app))),
    ).unwrap();
}

async fn _non_ui_version() {
    let (_, _, _, artifacts) = get_minimals(false).await;
    let inputs: String = get_ids_from_user();

    query_api(&inputs, &artifacts).await;

    // let ids : Vec<&str> = inputs.split_ascii_whitespace().collect();
    // //let mut ids_len5: Vec<&str> = Vec::new();

    // for id in ids {
    //     if id.len() == 4 || id.len() == 6 {
    //         match card_access(id).await {
    //             Ok(card) => {
    //                 check_and_write("card", Parsed::T(card)).await;
    //             },
    //             Err(err) => println!("{err:#?}"),
    //         }
    //     } 
    //     else if id.len() == 5 {
    //         if let Some(ref sets) = artifacts {
    //             if sets.contains_key(id) {
    //                 // artifact
    //                 let artifact = sets.get(id).unwrap();
    //                 let new_art = artifact_access(artifact, id).await;
    //                 check_and_write("artifact", Parsed::A(new_art)).await;
    //             } else {
    //                 check_weapon(id).await;
    //             }
    //         } else {
    //             check_weapon(id).await;
    //         }
    //         //ids_len5.push(id);
    //     }
    //     else {
    //         let character = character_api_access(id).await;
    //         check_and_write("character", Parsed::C(character)).await;
    //         //println!("{:#?}", character);
    //         //check_and_write_to_file(character).await;
    //     }
    // }
}






