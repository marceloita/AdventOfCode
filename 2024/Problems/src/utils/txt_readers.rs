use std::{fs::File, io::BufReader, io::BufRead};
use std::{io:: {Result, Error}, vec};

// PROBLEMA 1 I 2
pub fn read_file_P1P2(path: &str) -> Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut read_file = vec![];
    for line in reader.lines() {
        let line = line?;
        read_file.push(line);
    };
    Ok(read_file)
}

// PROBLEMA 3
pub fn read_file_P3(path: &str) -> Result<String, Error> {
    let mut file_contents = String::new();
    let _ = File::open(path)?.read_to_string(&mut file_contents);
    Ok(file_contents)
}

// PROBLEMA 4
pub fn read_matrix_P4(file_path: &str) -> Result<Vec<Vec<char>>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut matrix: Vec<Vec<char>> = Vec::new();
    let mut expected_columns: Option<usize> = None;

    for (line_num, line_result) in reader.lines().enumerate() {
        let line = line_result?;
        let chars: Vec<char> = line.chars().collect();

        match expected_columns {
            Some(cols) if chars.len() != cols => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Line {} has inconsistent number of columns: expected {}, got {}",
                            line_num + 1, cols, chars.len()),
                ));
            }
            None => {
                expected_columns = Some(chars.len());
            }
            _ => {}
        }

        matrix.push(chars);
    }

    Ok(matrix)
}

// PROBLEMA 5
pub fn extract_rules_P5(file_path: &str) -> Result<Vec<(u32,u32)>> {
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

pub fn extract_orders_P5(file_path: &str) -> Result<Vec<Vec<u32>>> {
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