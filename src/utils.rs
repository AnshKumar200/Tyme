use std::process::Command;

pub fn get_active_window() -> Option<String> {
    let output = Command::new("sh")
        .arg("-c")
        .arg("swaymsg -t get_tree | jq '.. | select(.type?) | select(.focused==true).app_id'")
        .output()
        .ok()?;

    let name = String::from_utf8(output.stdout)
        .ok()?
        .trim()
        .trim_matches('"')
        .to_string();

    if name.is_empty() || name == "null" {
        None
    } else {
        Some(name)
    }
}
