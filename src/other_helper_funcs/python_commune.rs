use std::{collections::HashMap, error::Error, io::Write, process::{Command, Stdio}};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CleanDiffs {
    Str(String),
    Arr(Vec::<String>)
}

pub fn compare_via_python<T: Serialize>(old: &T, new: &T) -> Result<HashMap<String, CleanDiffs>, Box<dyn Error>> {

    let input = serde_json::json!({
        "old": old,
        "new": new
    }).to_string();

    // spawn python process
    let mut cmd = Command::new("python")
        .arg("src/python/comparer.py")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    // write json to stdin
    if let Some(ref mut stdin) = cmd.stdin {
        stdin.write_all(input.as_bytes())?;
    }

    // wait for python and capture output
    let output = cmd.wait_with_output()?;
    if !output.status.success() {
        return Err(format!(
            "Python failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ).into());
    }

    // parse output to struct
    let result: HashMap<String, CleanDiffs> = serde_json::from_slice(&output.stdout)?;

    // return
    Ok(result)
}