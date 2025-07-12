use std::collections::BTreeMap;

use crate::{character::{AscensionORTalent, Materials}, parsed_character::ParsedMaterial};

pub fn parse_materials<'b>(materials : &'b Materials) -> (Vec<ParsedMaterial>, Vec<ParsedMaterial>) {
    //ascensions
    let ascension = &materials.ascensions;
    let mut parsed_asc: BTreeMap<&str, i64> = BTreeMap::<&str, i64>::new();
    single_full_asc_or_talent_parsing(&mut parsed_asc, ascension);

    //talents
    let talents = &materials.talents;
    let mut full_talent_costs = Vec::<BTreeMap::<&str, i64>>::new(); //0: basic, 1: skill, 2: burst

    for talent in talents {
        let mut talent_costs = BTreeMap::<&str, i64>::new();

        single_full_asc_or_talent_parsing(&mut talent_costs, talent);
        full_talent_costs.push(talent_costs);
    }

    return convert_maps_to_vecs(parsed_asc, full_talent_costs);
}

fn single_full_asc_or_talent_parsing<'c>(btree_map : &mut BTreeMap<&'c str, i64>, full_vector : &'c Vec<AscensionORTalent>) {
    for asc_or_talent in full_vector {
        single_asc_or_talent_vector_parsing(btree_map, asc_or_talent);
    }
}

fn single_asc_or_talent_vector_parsing<'c>(btree_map : &mut BTreeMap<&'c str, i64>, asc_or_talent : &'c AscensionORTalent){
    match btree_map.get("Cost"){
        Some(cost) => {
            btree_map.insert("Cost", cost + asc_or_talent.cost);
        },
        None => {
            btree_map.insert("Cost", asc_or_talent.cost);
        },
    }
    for mat in &asc_or_talent.mats {
        let name = mat.name.as_str();
        let amt = mat.count;
        match btree_map.get(name) {
            Some(val) => {
                btree_map.insert(name, val + amt);
            },
            None => {
                btree_map.insert(name, amt);
            },
        }
    }
}

fn convert_maps_to_vecs(asc: BTreeMap<&str, i64>, tal: Vec<BTreeMap<&str, i64>>) -> (Vec<ParsedMaterial>, Vec<ParsedMaterial>) {
    let mut ascension_mats : Vec<ParsedMaterial> = Vec::new();
    for (name, amount) in asc {
        let a_temp = ParsedMaterial {
            name: name.to_string(),
            amount
        };
        ascension_mats.push(a_temp);
    }
    ascension_mats.sort_by(|a, b| a.amount.cmp(&b.amount));

    if let Some(talent) = tal.first() {
        let mut talent_mats : Vec<ParsedMaterial> = Vec::new();
        for (name, amount) in talent {
            let t_temp = ParsedMaterial {
                name: name.to_string(),
                amount: *amount
            };
            talent_mats.push(t_temp);
        }
        talent_mats.sort_by(|a, b| a.amount.cmp(&b.amount));

        return (ascension_mats, talent_mats);
    } else {
        panic!("No talent mats!");
    }
}