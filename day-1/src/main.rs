use std::fs;

fn solution(curr_line: &str) -> String {
    //Answer 1 change: let matchers = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let matchers = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "zero", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    
    let results = matchers.iter().map(|x| curr_line.match_indices(x).collect::<Vec<(usize, &str)>>()).collect::<Vec<Vec<(usize, &str)>>>();
    // Additionally answer two added the replacements (however, we don't need another solution for this since not matching will just give us the numerics for answer 1)
    let parsed_results = results.iter().map(|x| x.iter().map(|y| (y.0, y.1.replace("zero", "0").replace("one", "1").replace("two", "2").replace("three", "3")
    .replace("four", "4").replace("five", "5").replace("six", "6").replace("seven", "7")
    .replace("eight", "8").replace("nine", "9"))).collect::<Vec<(usize, String)>>()).collect::<Vec<Vec<(usize, String)>>>();
    
    let mut flattened_results = parsed_results.into_iter().flatten().collect::<Vec<(usize, String)>>();
    flattened_results.sort_by(|a, b| a.0.cmp(&b.0));
    return "".to_owned() + &flattened_results[0].1 + &flattened_results[flattened_results.len() - 1].1;
}

fn main() {
    let content = fs::read_to_string("./src/input.txt").expect("Couldn't read file from given path");
    let lines = content.split("\n");
    
    let numbers = lines.map(solution);
    let answer = numbers.map(|x| x.parse::<i64>().unwrap()).reduce(|x,y| x + y).unwrap();
    println!("{}", answer);
}
