use std::{collections::{HashMap, VecDeque}, ffi::FromVecWithNulError, fs::File, io::{BufRead, BufReader, Result}};
use std::hash::Hash;

use crate::{problems::AdventProblem, utils::txt_readers::{extract_orders_P5, extract_rules_P5}};

pub struct Problem05;

impl AdventProblem for Problem05 {
    fn part1(&self,file_path: &str) -> i32 {

        let rules = match extract_rules_P5(file_path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Error reading file {e}");
                return -1;
            }
        };

        let orders = match extract_orders_P5(file_path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Error reading file {e}");
                return -1;
            }
        };

        let mut rules_map: HashMap<i32,Vec<i32>> = HashMap::new();

        for (key, value) in rules {
            rules_map.entry(key).or_insert(Vec::new()).push(value);
        }

        let mut resulting_seq: VecDeque<i32> = VecDeque::new();
        let mut total_sum = 0;

        for order in orders {
            resulting_seq = reorder_sequence(&rules_map, &order);
            if resulting_seq.eq(&order) {
                total_sum += resulting_seq[resulting_seq.len()/2];
            }
        }
        total_sum
    }
    fn part2(&self, file_path: &str) -> i32 {

        let rules = match extract_rules_P5(file_path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Error reading file {e}");
                return -1;
            }
        };

        let orders = match extract_orders_P5(file_path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Error reading file {e}");
                return -1;
            }
        };

        let mut rules_map: HashMap<i32,Vec<i32>> = HashMap::new();

        for (key, value) in rules {
            rules_map.entry(key).or_insert(Vec::new()).push(value);
        }

        let mut resulting_seq: VecDeque<i32> = VecDeque::new();
        let mut total_sum: i32 = 0;

        for order in orders {
            resulting_seq = reorder_sequence(&rules_map, &order);
            if !resulting_seq.eq(&order) {
                total_sum += resulting_seq[resulting_seq.len()/2];
            }
        }
        total_sum
    }
}

fn reorder_sequence(rules: &HashMap<i32,Vec<i32>>, sequence: &Vec<i32>) -> VecDeque<i32> {
    
    let mut in_degree: HashMap<i32, i32> = HashMap::new();

    // COUNT THE IN-DEGREE FOR THE CURRENT SEQUENCE
    for &node in sequence {
        in_degree.entry(node).or_insert(0);
    }

    for (&val_before, vals_after) in rules {
        if sequence.contains(&val_before) {
            for &val in vals_after {
                if sequence.contains(&val){
                    *in_degree.entry(val).or_insert(0) += 1;
                }
            }
        }
    };

    //  ADD THE IN-DEGREE 0 VALUES TO A QUEUE
    let mut queue = VecDeque::new();
    let mut result = VecDeque::new();

    for (&val, &degs) in &in_degree {
        if degs == 0 {
            queue.push_back(val);
        }
    };

    while let Some(node) = queue.pop_front() {
        result.push_back(node);
        if let Some(conc_rule) = rules.get(&node){
            for &c_rule in conc_rule {
                if in_degree.contains_key(&c_rule) {
                    let entry = in_degree.get_mut(&c_rule).unwrap();
                    *entry -= 1;
                    if *entry == 0 {
                        queue.push_back(c_rule);
                    }
                }
            }
        }
    };

    if result.len() == sequence.len() {
        return result;
    } else {
        return VecDeque::new();
    }
}