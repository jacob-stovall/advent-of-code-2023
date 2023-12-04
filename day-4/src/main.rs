use std::{fs, collections::BTreeSet};

fn solution_two(content: String) -> i32 {
    let mut result = 0;
    let lines = content.split("\n").collect::<Vec<&str>>();
    let mut indicies = (0..(lines.iter().count())).collect::<Vec<usize>>();

    while let Some(idx) = indicies.pop() {
        
        result += 1;
        let line = lines[idx];
        let numbers_side = line.split(":").collect::<Vec<&str>>()[1];
        let numbers_parts = numbers_side.split("|").collect::<Vec<&str>>();
        let winnning_numbers_str = numbers_parts[0];
        let winning_numbers = winnning_numbers_str.split(" ").filter(|x| !x.is_empty()).map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let winning_numbers_set: BTreeSet<i32> = BTreeSet::from_iter(winning_numbers);
        
        let elf_numbers_str = numbers_parts[1];
        let elf_numbers = elf_numbers_str.split(" ").filter(|x| !x.is_empty()).map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let match_count = elf_numbers.iter().filter(|x| winning_numbers_set.contains(x)).count();

        let mut j: usize = 1;
        while j <= match_count {
            indicies.push(idx + j);
            j += 1;
        }
    }

    return result;
}

fn solution_one(content: String) -> i32 {
    let mut result = 0;
    let mut lines = content.split("\n").collect::<Vec<&str>>();

    while let Some(line) = lines.pop() {
        let numbers_side = line.split(":").collect::<Vec<&str>>()[1];
        let numbers_parts = numbers_side.split("|").collect::<Vec<&str>>();
        let winnning_numbers_str = numbers_parts[0];
        let winning_numbers = winnning_numbers_str.split(" ").filter(|x| !x.is_empty()).map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let winning_numbers_set: BTreeSet<i32> = BTreeSet::from_iter(winning_numbers);
        
        let elf_numbers_str = numbers_parts[1];
        let elf_numbers = elf_numbers_str.split(" ").filter(|x| !x.is_empty()).map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let match_count = elf_numbers.iter().filter(|x| winning_numbers_set.contains(x)).count();
        if match_count > 0 {
            result += 2_i32.pow((match_count - 1).try_into().unwrap());
        }
    }

    return result;
}

fn main() {
    let file_contents = fs::read_to_string("./src/input.txt").expect("Error when reading contents of input file");
    let result = solution_two(file_contents);

    println!("{}", result);
}
