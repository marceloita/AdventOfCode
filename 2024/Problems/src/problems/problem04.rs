use std::{char};
use crate::problems::AdventProblem;
use crate::utils::txt_readers::read_matrix_P4;

const  DIRECTIONS_PR1: [(isize,isize);8] = [
    (0,1), // RIGHT
    (0,-1), // LEFT
    (1,0), // DOWN
    (-1,0), // UP
    (-1,1), // UP-RIGHT
    (-1,-1), // UP-LEFT
    (1,1), // DOWN-LEFT
    (1,-1) // DOWN-RIGHT
];

const DIRECTIONS_CROSS: [((isize, isize), (isize, isize)); 2] = [
    ((-1, -1), (1, 1)),  // Diagonal top-left to bottom-right
    ((-1, 1), (1, -1)),  // Diagonal top-right to bottom-left
];

pub struct Problem04;

impl AdventProblem for Problem04 {
    fn part1(&self, file_path: &str) -> i32 {

        let word: &str = "XMAS";
        let xmas_matrix = match read_matrix_P4(file_path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Error reading file {e}");
                return -1;
            }
        };

        let mut xmas_count = 0;

        for i in 0..xmas_matrix.len() {
            for j in 0..xmas_matrix[0].len() {

                for &(dx,dy) in &DIRECTIONS_PR1 {
                    if is_correct(&xmas_matrix, i, j, dx, dy, &word) {
                        xmas_count += 1;
                    }
                }
            }
        }
        xmas_count
    }

    fn part2(&self, file_path: &str) -> i32 {

        let xmas_matrix = match read_matrix_P4(file_path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Error reading file {e}");
                return -1;
            }
        };

        let mut xmas_count = 0;

        for i in 0..xmas_matrix.len() {
            for j in 0..xmas_matrix[0].len() {

                if xmas_matrix[i][j] != 'A' {
                    continue;
                }

                let mut checker = 0;
                for &((dx1, dy1),(dx2, dy2)) in &DIRECTIONS_CROSS {
                    if is_cross_correct(&xmas_matrix, i, j, dx1, dy1, dx2, dy2) {
                        checker += 1;
                    }

                    if checker == 2 {
                        xmas_count += 1;
                    }
                }
            }
        };
        xmas_count
    }
}

pub fn part1_old(xmas_matrix: &Vec<Vec<char>>) -> i32 {
    let mut xmas_app = 0;
    for i in 0..xmas_matrix.len() {
        for j in 0..xmas_matrix[i].len() {

            if xmas_matrix[i][j].eq(&'X') {

                if (xmas_matrix[i].len()-1 - j >= 3) { // Rigth check
                    if idx_to_check(&xmas_matrix,i, j+1, i, j+2, i, j+3)  {
                        xmas_app += 1;
                    }
                }
                
                if (xmas_matrix[i].len() - j <= xmas_matrix[i].len() - 3 ){ // Left check
                    if idx_to_check(&xmas_matrix,i, j-1, i, j-2, i, j-3) {
                        xmas_app += 1;
                    }
                }
                
                if (xmas_matrix.len() - i <= xmas_matrix.len() - 3) { // Up check
                    if idx_to_check(&xmas_matrix,i-1,j, i-2, j, i-3, j) {
                        xmas_app += 1;
                    }
                }
                
                if (xmas_matrix.len()-1 - i >= 3) { // Down check
                    if idx_to_check(&xmas_matrix,i+1,j, i+2, j, i+3, j) {
                        xmas_app += 1;
                    }
                }
                
                if (xmas_matrix.len() - i <= xmas_matrix.len() - 3) // Up-Right diagonal check
                    && (xmas_matrix[i].len()-1 - j >= 3 ) {
                    if idx_to_check(&xmas_matrix,i-1,j+1, i-2, j+2, i-3, j+3) {
                        xmas_app += 1;
                    }
                }
                
                if (xmas_matrix.len() - i <= xmas_matrix.len() - 3) // Up-Left diagonal check
                    && (xmas_matrix[i].len() - j <= xmas_matrix[i].len() - 3 ) {
                    if idx_to_check(&xmas_matrix,i-1,j-1, i-2, j-2, i-3, j-3) {
                        xmas_app += 1;
                    }
                }
                
                if (xmas_matrix.len()-1 - i >= 3) // Down-Right diagonal check
                    &&  (xmas_matrix[i].len()-1 - j >= 3 ) {
                    if idx_to_check(&xmas_matrix,i+1,j+1, i+2, j+2, i+3, j+3) {
                        xmas_app += 1;
                    }
                } 
                
                if (xmas_matrix.len()-1 - i >= 3) // Down-Left diagonal check
                    && (xmas_matrix[i].len() - j <= xmas_matrix[i].len() - 3 ) {
                    if idx_to_check(&xmas_matrix,i+1,j-1, i+2, j-2, i+3, j-3)  {
                        xmas_app += 1;
                    }
                }
            }
        }
    }
    xmas_app
}

fn idx_to_check(xmas_matrix: &Vec<Vec<char>>,
    m_x_idx: usize, m_y_idx: usize,
    a_x_idx: usize, a_y_idx: usize,
    s_x_idx: usize, s_y_idx: usize) -> bool {
    xmas_matrix[m_x_idx][m_y_idx].eq(&'M')
    && xmas_matrix[a_x_idx][a_y_idx].eq(&'A')
    && xmas_matrix[s_x_idx][s_y_idx].eq(&'S')
}

fn is_correct(xmas_matrix: &Vec<Vec<char>>, x: usize, y: usize, dx: isize, dy: isize, word: &str) -> bool{
    let n = xmas_matrix.len();
    let m = xmas_matrix[0].len();
    for (i, char) in word.chars().enumerate() {
        let nx = x as isize + dx * i as isize;
        let ny = y as isize + dy * i as isize;
        if nx < 0 || ny < 0 || nx >= n as isize || ny >= m as isize || xmas_matrix[nx as usize][ny as usize] != char {
            return false
        }
    };
    true
}

fn is_cross_correct(xmas_matrix: &Vec<Vec<char>>, x: usize, y: usize, dx1: isize, dy1: isize, dx2: isize, dy2: isize) -> bool {
    let n = xmas_matrix.len() as isize;
    let m = xmas_matrix[0].len() as isize;

    let (x, y) = (x as isize, y as isize);

    let (nx1, ny1) = (x + dx1, y + dy1);
    let (nx2, ny2) = (x + dx2, y + dy2);

    if nx1 < 0 || ny1 < 0 || nx2 < 0 || ny2 < 0 ||
       nx1 >= n || ny1 >= m || nx2 >= n || ny2 >= m {
        return false;
    }

    let c1 = xmas_matrix[nx1 as usize][ny1 as usize];
    let c2 = xmas_matrix[nx2 as usize][ny2 as usize];

    (c1 == 'M' && c2 == 'S') || (c1 == 'S' && c2 == 'M')

}

