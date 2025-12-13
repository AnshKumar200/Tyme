use std::{collections::HashMap, process::Command, thread, time::Duration};

fn get_active_window() -> Option<String> {
    let output = Command::new("sh")
        .arg("-c")
        .arg("swaymsg -t get_tree | jq '.. | select(.type?) | select(.focused==true).app_id'")
        .output()
        .ok()?;

    let name = String::from_utf8(output.stdout).ok()?.trim().trim_matches('"').to_string();

    if name.is_empty() || name == "null" {
        None
    } else {
        Some(name)
    }
}

fn main() {
    let mut db: HashMap<String, u8> = HashMap::new();

    loop {
        if let Some(app_name) = get_active_window() {
            let value = db.entry(app_name).or_insert(0);
            *value += 1;
        }

        println!("{:?}", db);

        thread::sleep(Duration::from_secs(1));
    }
}
