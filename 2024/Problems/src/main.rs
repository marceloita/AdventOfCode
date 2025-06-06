mod problems;
mod utils;
use problems::*;
use std::io::{stdin};


const MIN_PROBLEM_NUMBER: u32 = 1;
const MAX_PROBLEM_NUMBER: u32 = 24;

fn main(){

    println!("Enter a problem number:");
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let mut problem_number = 0;

    match input.trim().parse::<u32>() {
        Ok(p_number) if p_number >= MIN_PROBLEM_NUMBER || p_number <= MAX_PROBLEM_NUMBER => { 
            problem_number = p_number;
        }
        Ok(_) => {
            println!("The number must be  between 1 and 24");
        }
        Err(e) => {
            eprintln!("Failed to parse  the  input: {}", e);
        }
    }

    if let Some(problem) = get_problem(problem_number) {
        println!("inputs/problem{}_input.txt", problem_number);
        let input_path = format!("src/inputs/problem{}_input.txt", problem_number);

        println!("Part 1 result: {}", problem.part1(&input_path));
        println!("Part 2 result: {}", problem.part2(&input_path));
    } else {
        eprint!("Problem not found my dear...");
    }
}
