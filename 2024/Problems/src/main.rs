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
        let input_path = format!("inputs/problem{:02}_input.txt", problem_number);

        println!("Part 1 result: {}", problem.part1(&input_path));
        println!("Part 2 result: {}", problem.part2(&input_path));
    } else {
        eprint!("Problem not found my dear...");
    }




    // ------------------- PROBLEM 1 -------------------
    // let distance = problem1_part1(&file_content);
    // println!("{}", distance);
    // let similarity = problem1_part2(&file_content);
    // println!("{similarity}");
    // -------------------------------------------------

    // ------------------- PROBLEM 2 -------------------
    // let safe_index = problem2_part2(file_content);
    // println!("{safe_index}");
    // -------------------------------------------------

    // ------------------- PROBLEM 3 -------------------
    // let mult_result = problem3_part1(&file_content);
    // println!("{mult_result}");
    // -------------------------------------------------

    // ------------------- PROBLEM 4 -------------------
    // let word: &str = "XMAS";
    // let xmas_result = problem4_part1(&file_content, word);
    // let xmas_result =problem4_part2(&file_content);
    // println!("{xmas_result}");
    // -------------------------------------------------

    // ------------------- PROBLEM 5 -------------------
    // let decoded_sum_p1 = problem5_part1(&rules, &orders);
    // let decoded_sum_p2 = problem5_part2(&rules, &orders);

    // println!("{decoded_sum_p1}");
    // println!("{decoded_sum_p2}");
    // -------------------------------------------------


}
