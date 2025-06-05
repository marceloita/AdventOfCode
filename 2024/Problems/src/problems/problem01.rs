use crate::utils::txt_readers::{read_file_P1P2};
use crate::problems::AdventProblem;

pub struct Problem01;

impl AdventProblem for Problem01 {
    fn part1(&self, file_path: &str) -> i32 {
        
        let file_content = match read_file_P1P2(&file_path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Error reading file {e}");
                return -1;
            }
        };

        let mut left_column: Vec<i32>= Vec::new();
        let mut right_column: Vec<i32> = Vec::new();

        for i in 0..file_content.len() {

            let parts: Vec<&str> =  file_content[i].split_whitespace().collect();
            
            if parts.len() == 2 {
                left_column.push(parts[0].to_owned().parse().expect("Fail"));
                right_column.push(parts[1].to_owned().parse().expect("Fail"));
            }else{
                println!("shit");
            }
        }

        left_column.sort();
        right_column.sort();

        let mut distance = 0;

        for i in 0.. file_content.len() {
            
            let absolute_distance: i32 = left_column[i] - right_column[i];
            distance += absolute_distance.abs();
        };
        distance
    }

    fn part2(&self, file_path: &str) -> i32 {

        let file_content = match read_file_P1P2(&file_path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Error reading file {e}");
                return -1;
            }
        };

        let mut left_column: Vec<i32>= Vec::new();
        let mut right_column: Vec<i32> = Vec::new();

        for i in 0..file_content.len() {
            //let line: String = line?; // Handle possible I/O error.
            let parts: Vec<&str> =  file_content[i].split_whitespace().collect();

            if parts.len() == 2 {
                left_column.push(parts[0].to_owned().parse().expect("Fail"));
                right_column.push(parts[1].to_owned().parse().expect("Fail"));
            }else{
                println!("cagada");
            }
        }

        left_column.sort();
        right_column.sort();

        let mut similarity = 0;

        for i in 0..left_column.len() {
            let target = &left_column[i];
            let counter = right_column.iter().filter(|&&x| &x == target).count() as i32;
            if counter != 0 {
                similarity += counter * left_column[i];
            }
        };
        similarity
    }
}