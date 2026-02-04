use colored::*;
use crate::constants::result::game_msg;

pub fn check_result(p: &str, s: &str) -> ColoredString {
    // ใช้ .as_str() เพื่อให้ String สามารถเทียบกับ &str ใน match ได้
    match (p, s) {
        (user, bot) if user == bot => game_msg::DRAW.to_string().yellow(),
        ("R", "S") | ("S", "P") | ("P", "R") => game_msg::WIN.to_string().green(),
        _ => game_msg::LOSE.to_string().red(),
    }
}