use std::collections::HashMap;
use std::io::Read;
use std::{fs::File};
use std::{io};

use crate::problems::AdventProblem;

pub struct Problem03;

impl AdventProblem for Problem03 {
    fn part1(file_content: &str) -> i32 {

        let enable_idx = get_enable_indexes(&file_content);
        let disable_idx = get_disable_indexes(&file_content);
        let mut multiplier_idx = get_multipliers(&file_content);

        let mut enable_limit = 0;
        
        for i in 0..disable_idx.len() {
            for j in 0..enable_idx.len() {
                
                if enable_idx[j] > disable_idx[i]{
                    enable_limit = enable_idx[j];
                    break;
                }
            }

            multiplier_idx.retain(|&k,_| k < disable_idx[i] || k > enable_limit);
        }

        let result: i32 = multiplier_idx.values().map(|(a,b)| a*b).sum();
        result
    }

}

fn get_multipliers(file_content: &str) -> HashMap<usize,(i32,i32)> {

    let target = "mul(";
    let mut multipliers: HashMap<usize, (i32, i32)> = HashMap::new();
    let mut start = 0;

    while let Some(pos) = file_content[start..].find(target) {
        let begin = start + pos + target.len();
        
        if let Some(end) = &file_content[begin..].find(')') {
            let possible_num = &file_content[begin..begin + end];
            let parts: Vec<&str> = possible_num.split(',').map(|s| s.trim()).collect();

            if parts.len() == 2 {
                if let (Ok(a), Ok(b)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                    multipliers.insert(begin-target.len(), (a,b));
                }
            }
            start = begin;
        } else {
            break;
        }
    }
    multipliers
}

fn get_enable_indexes(file_content: &str) -> Vec<usize> {

    let mut start = 0;
    let enable = "do()";
    let mut enablers: Vec<usize> = Vec::new();

    while let Some(pos) = file_content[start..].find(enable) {
        enablers.push(start + pos);
        start = start + pos + enable.len();
    }
    
    enablers
}

fn get_disable_indexes(file_content: &str) -> Vec<usize> {

    let mut start = 0;
    let disable = "don't()";
    let mut disablers: Vec<usize> = Vec::new();

    while let Some(pos) = file_content[start..].find(disable) {
        disablers.push(start + pos);
        start = start + pos + disable.len();
    }
    disablers
}


    
