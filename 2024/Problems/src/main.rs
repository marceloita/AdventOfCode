#![allow(unused_variables)]
#![allow(unused_impor)]
mod Problem1;
mod Problem2;
mod Problem3;
mod Problem4;
mod Problem5;

use std::{path, vec};

use Problem1::{part1 as problem1_part1, part2 as problem1_part2};
use Problem2:: {part1 as problem2_part1, part2 as  problem2_part2};
use Problem3::{part1 as problem3_part1};
use Problem4::{part1_refact as problem4_part1, part2 as problem4_part2};
use Problem5::{part1 as problem5_part1};

const FILE_NAME: &str = "problem5.txt";

fn main(){

    let path_name = String::from(FILE_NAME);
    let rules_check = Problem5::extract_rules(&path_name);
    let orders_check = Problem5::extract_orders(&path_name);

    let mut rules: Vec<(u32,u32)> = Vec::new();

    match rules_check {
        Ok(file_content) => rules = file_content,
        Err(e) => eprintln!("Error"),
    }

    let mut orders: Vec<Vec<u32>> = Vec::new();

    match orders_check {
        Ok(file_content) => orders = file_content,
        Err(e) => eprintln!("Error"),
    }

    // let mut file_content = Vec::with_capacity(file_content_check.iter().len());
    // let mut file_content = String::new();

    // match file_content_check {
    //     Ok(file_content_check) => file_content = file_content_check,
    //     Err(e) => eprintln!("Error: {}", e),
    // }

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
    let test = problem5_part1(&rules, &orders);
    // -------------------------------------------------


}
