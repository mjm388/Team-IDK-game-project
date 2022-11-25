use std::{collections::HashMap, fs::File, io::BufReader};

pub fn read_in() -> std::io::Result<HashMap<String, HashMap<String, f64>>>{
    let f = File::open("agent_small.json")?;
    let file = BufReader::new(f);
    let ai_state = serde_json::from_reader(file)?;
    Ok(ai_state)
}
