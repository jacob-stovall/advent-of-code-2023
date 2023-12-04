
use std::fs;

fn check_bounding_box_has_symbol(grid: &Vec<Vec<char>>, line_y: usize, start_x: usize, end_x: usize) -> bool {
    let mut y = 0;
    if line_y > 0 {
        y = line_y - 1;
    }
    
    while y <= line_y + 1 {
        let mut x = 0;
        if start_x > 0 {
            x = start_x - 1;
        }

        while x <= end_x + 1 {
            if y > 0 && y < grid.len() && x > 0 && x < grid[0].len() && !(grid[y][x].is_numeric()) && !(grid[y][x] == '.'){
                return true;
            }
            x += 1;
        }
        y += 1;
    }
    return false;
}

fn build_string(grid: &Vec<Vec<char>>, y: usize, start_x: usize, end_x: usize) -> String {
    let mut temp_x = start_x;
    let mut str_curr = String::from("");
    while temp_x <= end_x {
        str_curr.push(grid[y][temp_x]);
        temp_x += 1
    }
    return str_curr;
}

fn solution_one(grid: &Vec<Vec<char>>) -> i32 {
    let mut part_numbers: Vec<i32> = Vec::new();
    let mut total: i32 = 0;

    let mut y = 0;
    while y < grid.len() {
        let mut x = 0;
        let mut start_x = usize::MAX;
        let mut end_x = usize::MAX;
        while x < grid[y].len() {
            if grid[y][x].is_numeric() && start_x == usize::MAX {
                start_x = x;
            }
            if !(grid[y][x].is_numeric()) && start_x < usize::MAX {
                end_x = x - 1;
                if check_bounding_box_has_symbol(grid, y, start_x, end_x) {
                    part_numbers.push(build_string(grid, y, start_x, end_x).parse::<i32>().unwrap());
                }
                start_x = usize::MAX;
                end_x = usize::MAX;
            }
            x += 1;
            println!("{} {}", x, y);
        }
        if(start_x < usize::MAX) {
            end_x = x - 1;
            if check_bounding_box_has_symbol(grid, y, start_x, end_x) {
                part_numbers.push(build_string(grid, y, start_x, end_x).parse::<i32>().unwrap());
            }
            start_x = usize::MAX;
            end_x = usize::MAX;
        }
        y += 1;
    }
    while let Some(val) = part_numbers.pop() {
        total += val;
    }
    return total;
}

fn main() {
    let file_contents = fs::read_to_string("./src/input.txt").expect("There was an error reading the file at the specified path.");
    let grid = file_contents.split("\n").map(|x| x.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    println!("{}", solution_one(&grid));
}
