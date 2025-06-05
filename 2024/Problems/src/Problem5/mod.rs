use std::{collections::HashMap, fs::File, io::{BufRead, BufReader, Result}};
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

pub fn part1(rules: &Vec<(u32,u32)>, orders: &Vec<Vec<u32>>) {

    let mut rules_map: HashMap<u32,Vec<u32>> = HashMap::new();

    for (key, value) in rules {
        rules_map.entry(*key).or_insert(Vec::new()).push(*value);
    }

    println!("{rules_map:#?}");
    // for i in 0..orders.len() {
    //     for j in 0..orders[i].len() {

    //     }
    // }
}