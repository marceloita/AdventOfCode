use std::{collections::{HashMap, VecDeque}, ffi::FromVecWithNulError, fs::File, io::{BufRead, BufReader, Result}};
use std::hash::Hash;


pub fn extract_rules(file_path: &str) -> Result<Vec<(u32,u32)>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut rules: Vec<(u32,u32)> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str>  = line.trim().split('|').collect();
        if parts.len() == 2 {
            let a = parts[0].parse::<u32>().unwrap_or(0);
            let b = parts[1].parse::<u32>().unwrap_or(0);
            rules.push((a,b));
        }
    };
    Ok(rules)

}

pub fn extract_orders(file_path: &str) -> Result<Vec<Vec<u32>>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut orders: Vec<Vec<u32>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let order_line = line.trim().split(',').filter_map(|x| x.parse::<u32>().ok()).collect::<Vec<u32>>();
        if !order_line.is_empty() {
            orders.push(order_line);
        }
    };

    Ok(orders)
}

pub fn part1(rules: &Vec<(u32,u32)>, orders: &Vec<Vec<u32>>) -> u32 {

    let mut rules_map: HashMap<u32,Vec<u32>> = HashMap::new();

    for (key, value) in rules {
        rules_map.entry(*key).or_insert(Vec::new()).push(*value);
    }

    let mut resulting_seq: VecDeque<u32> = VecDeque::new();
    let mut total_sum = 0;

    for order in orders {
        resulting_seq = reorder_sequence(&rules_map, &order);
        if resulting_seq.eq(order) {
            total_sum += resulting_seq[resulting_seq.len()/2];
        }
    }
    total_sum
}

pub fn part2(rules: &Vec<(u32,u32)>, orders: &Vec<Vec<u32>>) -> u32 {
    let mut rules_map: HashMap<u32,Vec<u32>> = HashMap::new();

    for (key, value) in rules {
        rules_map.entry(*key).or_insert(Vec::new()).push(*value);
    }

    let mut resulting_seq: VecDeque<u32> = VecDeque::new();
    let mut total_sum = 0;

    for order in orders {
        resulting_seq = reorder_sequence(&rules_map, &order);
        if !resulting_seq.eq(order) {
            total_sum += resulting_seq[resulting_seq.len()/2];
        }
    }
    total_sum
}

fn reorder_sequence(rules: &HashMap<u32,Vec<u32>>, sequence: &Vec<u32>) -> VecDeque<u32> {
    
    let mut in_degree = HashMap::new();


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

    // if result.len() == sequence.len() && result.eq(sequence){
    //     let idx = result.len()/2;
    //     return result[idx];
    // } else {
    //     return 0;
    // }


}