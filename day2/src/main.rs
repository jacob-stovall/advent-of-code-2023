use std::collections::HashMap;
use std::fs;

// Solution 2 stuff
// =============================================

fn get_power_of_line(line: &str) -> i32 {
    let mut power_map = HashMap::from([("green", 0), ("red", 0), ("blue", 0)]);
    let mut game_rounds = line.split(";").collect::<Vec<&str>>();

    while let Some(round) = game_rounds.pop() {
        let mut colors_drawn = round.split(",").collect::<Vec<&str>>().iter().map(|x| x.trim()).collect::<Vec<&str>>().iter().map(|x| x.split(" ").collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();
        while let Some(color_drawn_pair) = colors_drawn.pop() {
            let color = color_drawn_pair[1];
            let amount = color_drawn_pair[0].parse::<i32>().unwrap();

            if(power_map[color] < amount){
                power_map.insert(color, amount);
            }
        }
    }

    return power_map["red"] * power_map["blue"] * power_map["green"];
}

fn solution_two(content: String) -> i32 {
    let mut total_power: i32 = 0;
    let mut lines = content.split("\n").collect::<Vec<&str>>();
    while let Some(line) = lines.pop() {
        let split_line_by_colon = line.split(":").collect::<Vec<&str>>();
        total_power += get_power_of_line(split_line_by_colon[1]);
        
    }

    return total_power;
}

// ====================================
// End of Solution 2 Stuff


// Solution 1 stuff
// =============================================
fn is_over_max_value(color: &str, num: i32) -> bool {
    match color {
        "red" => return num > 12,
        "green" => return num > 13,
        "blue" => return num > 14,
        _ => {
            println!("What happened?");
            return true;
        }
    }
}

fn is_line_legal(line: &str) -> bool {
    let mut game_rounds = line.split(";").collect::<Vec<&str>>();

    while let Some(round) = game_rounds.pop() {
        let mut colors_drawn = round.split(",").collect::<Vec<&str>>().iter().map(|x| x.trim()).collect::<Vec<&str>>().iter().map(|x| x.split(" ").collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();
        while let Some(color_drawn_pair) = colors_drawn.pop() {
            let color = color_drawn_pair[1];
            let amount = color_drawn_pair[0].parse::<i32>().unwrap();

            if(is_over_max_value(color, amount))
            {
                return false;
            }
        }
    }
    return true;
}

fn solution_one(content: String) -> i32 {
    let mut result: i32 = 0;
    let mut lines = content.split("\n").collect::<Vec<&str>>();
    while let Some(line) = lines.pop() {
        let split_line_by_colon = line.split(":").collect::<Vec<&str>>();
        // id of the game
        let id = split_line_by_colon[0].split(" ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
        if(is_line_legal(split_line_by_colon[1])) {
            result += id;
        }
        
    }

    return result;
}

// ====================================
// End of Solution 1 Stuff

fn main() {
    let text = fs::read_to_string("./src/input.txt").expect("Error when trying to read file at the specified path");
    let result = solution_two(text);

    println!("{}", result);
}
