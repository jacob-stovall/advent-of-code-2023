use std::fs;

fn main() {
    let contents = fs::read_to_string("./src/input.txt").expect("File could not be found");
    let numbers = contents.split("\n").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let mut i = 3; // = 1; for answer 1
    let mut count = 0;
    while i < numbers.len() {
        // Condition for answer 1: numbers[i-1] < numbers[i]
        if numbers[i-1] + numbers[i-2] + numbers[i-3] < numbers[i] + numbers[i-1] + numbers[i-2]
        {
            count += 1;
        }
        i += 1;
    }
    println!("{}", count);
}
