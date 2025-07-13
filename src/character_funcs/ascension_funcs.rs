use serde_json::Value;

pub fn get_ascension_stat_option(asc : Option<&Value>) -> &str {
    match asc {
        Some(y) => {
            return get_ascension_stat(y);
        },
        None => {
            return "";
        }
    }
}

fn get_ascension_stat(asc : &Value) -> &str {
    let known_stats: [&str; 3] = ["FIGHT_PROP_BASE_ATTACK", "FIGHT_PROP_BASE_DEFENSE", "FIGHT_PROP_BASE_HP"];
    let keys: serde_json::map::Keys<'_> = asc.as_object().unwrap().keys();
    for key in keys {
        if !known_stats.contains(&key.as_str()) {
            return key.as_str();
        }
    }
    ""
}