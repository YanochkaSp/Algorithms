// use std::iter;

fn main() {
    // dbg!(reverse_words("Let's take LeetCode contest".to_string()));
    dbg!(final_position_of_snake(2, vec!["RIGHT".to_string(),"DOWN".to_string()]));
}

// pub fn reverse_words(s: String) -> String {
//     s.split_whitespace()
//         .flat_map(|e| e.chars().rev().chain(iter::once(' ')))
//         .map(|e| e)
//         .take(s.len())
//         .collect::<String>()
// }

pub fn final_position_of_snake(n: i32, commands: Vec<String>) -> i32 {
    commands.iter().map(|cmd| parse_command(cmd, n)).sum()
    
}

fn parse_command(cmd: &str, n: i32) -> i32 {
    match cmd {
        "RIGHT" => 1,
        "LEFT" => -1,
        "UP" => -n,
        "DOWN" => n,
        _ => unreachable!("cmd must be valid")
    }
}