use regex::{Captures, Regex};
use serde::{Deserialize, Serialize};

use crate::{base_models::character::HasDescriptionRef, character::{Promote, Skill}, parsed_models::ParsedSkill};
use crate::helper_funcs::clean_text;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OneValOrTwo {
    One(f64),
    OneTen((f64, f64))
}

pub fn handle_skills<'a>(skills : &'a Vec<Skill>) -> (Vec<ParsedSkill>, Vec<String>) {
    let mut better_skills : Vec<ParsedSkill> = Vec::new();
    let mut terms: Vec<String> = vec![];
    for skill in skills {
        //let (param_list, param_values) = handle_stats_trim(&skill.promote);
        //let str_list_converted_to_string = param_list.iter().map(|&z| z.to_string()).collect();
        let (desc, mut key_terms) = clean_text(&skill.description());
        let x = ParsedSkill {
            name: skill.name.clone(),
            desc,
            parameters: handle_stats_trim_regex(&skill.promote),
            //param_values
        };
        better_skills.push(x);
        terms.append(&mut key_terms);
    }
    return (better_skills, terms);
}

fn handle_stats_trim_regex<'a>(skill : &'a Promote) -> Vec<String> {
    let re = Regex::new(r"\{param[0-9]+\:[A-Z0-9]+\}").unwrap();
    let n0 = &skill.n0;

    let mut parsed_params = Vec::<String>::new();
    let n0_param = &n0.param;
    let n9_param = match &skill.n9 {
        Some(val) => &val.param,
        None => &Vec::<f64>::new(),
    };
    // let n9_param = &skill.n9.param;
    let n12_param = match &skill.n12 {
        Some(val) => &val.param,
        None => &Vec::<f64>::new(),
    };

    for desc in &n0.desc {
        if desc.eq_ignore_ascii_case("") {
            break;
        }
        let new_desc = handle_stats_regex_separate(&desc.replace("×", "x"), n0_param, n9_param, n12_param, &re);
        parsed_params.push(new_desc);
    }
    parsed_params
}

fn handle_stats_regex_separate(desc : &String, n0: &Vec<f64>, n9: &Vec<f64>, n12: &Vec<f64>, re : &Regex) -> String{
    let new_desc = re.replace_all(desc, |caps: &Captures| {
        let captured_string = caps[0].parse::<String>().unwrap();
        let Some(start) = captured_string.find("m") else {
            return captured_string;
        };
        let Some(end) = captured_string.find(':') else {
            return captured_string;
        };
        let Some(end_brace) = captured_string.find('}') else {
            return captured_string;
        };
        let format_type = captured_string[end+1..end_brace].parse::<String>().unwrap();

        //let seconds = end_brace + 1 < captured_string.len();
        let Ok(index) = captured_string[start+1..end].parse::<usize>() else {
            return captured_string;
        };
        let index = index - 1;
        let Some(n0_value) = n0.get(index) else {
            return captured_string;
        };
        // check if n9 is empty. if so, skip to implementing single param version
        if n9.is_empty() {
            return format_percentage_or_not(format_type, *n0_value);
        }
        // else, parse n9
        let Some(n9_value) = n9.get(index) else {
            return format_percentage_or_not(format_type, *n0_value).to_owned();
        };

        // if n0 = n9, use single param version
        if n0_value.eq(n9_value) {
            return format_percentage_or_not(format_type, *n0_value);
        }

        // otherwise, also parse n12 and calculate with all three params
        let Some(n12_value) = n12.get(index) else {
            return format_percentage_or_not(format_type, *n0_value).to_owned();
        };

        return format_percentage_or_not_two_params(format_type, *n0_value, *n9_value, *n12_value);
    });

    let final_desc = new_desc.to_string();
    return final_desc;
}

fn format_percentage_or_not (param_type : String, param : f64) -> String {
    //println!("{param_type}: {param}");
    if param_type.eq("F1P") {
        let param_100 = param*100.0;
        format!("{param_100:.1}%")
    } else if param_type.eq("F2P") {
        let param_100 = param*100.0;
        format!("{param_100:.2}%")
    } else if param_type.eq("F1") {
        format!("{param:.1}")
    } else {
        format!("{param:.0}")
    }
}

fn format_percentage_or_not_two_params (
    param_type : String, 
    n0: f64, 
    n9: f64,
    n12: f64) -> String {
    //println!("{param_type}: {n0} {n9} {n12}");
    if param_type.eq("F1P") {
        let n0_100 = n0*100.0;
        let n9_100 = n9*100.0;
        let n12_100 = n12*100.0;
        format!("[{n0_100:.1}|{n9_100:.1}|{n12_100:.1}]%")
    } else if param_type.eq("F2P") {
        let n0_100 = n0*100.0;
        let n9_100 = n9*100.0;
        let n12_100 = n12*100.0;
        format!("[{n0_100:.2}|{n9_100:.2}|{n12_100:.2}]%")
    } else if param_type.eq("F1") {
        format!("[{n0:.1}|{n9:.1}|{n12:.1}]")
    } else {
        format!("[{n0:.0}|{n9:.0}|{n12:.0}]")
    }
}