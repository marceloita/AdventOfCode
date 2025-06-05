use std::{num::ParseIntError, process::Termination};


pub fn part1(file_content: Vec<String>) -> i32 {
    
    let mut safe = 0;
    for i in 0..file_content.len() {
        let unusual_data_frame_tmp: Vec<&str> = file_content[i].split_whitespace().collect();
        let unusual_data_frame: Vec<i32>  = unusual_data_frame_tmp.iter().filter_map(|d|  d.parse::<i32>().ok()).collect();         

        let mut asc_data = unusual_data_frame.clone();
        let mut desc_data = unusual_data_frame.clone();

        asc_data.sort();
        desc_data.sort_by(|a, b| b.cmp(a));

        asc_data.iter().eq(&unusual_data_frame);

        if (asc_data.iter().eq(&unusual_data_frame) || desc_data.iter().eq(&unusual_data_frame))
            && unusual_data_frame
                .windows(2)
                .all(|pair| (pair[0] - pair[1]).abs() >= 1 && (pair[0] - pair[1]).abs() <= 3) {
            safe += 1;
        }

    };
    safe
}

pub fn part2(file_content: Vec<String>) -> i32 {

    let mut safe = 0;

    for i in 0..file_content.len() {
        let unusual_data_frame_tmp: Vec<&str> = file_content[i].split_whitespace().collect();
        let unusual_data_frame: Vec<i32>  = unusual_data_frame_tmp.iter().filter_map(|d|  d.parse::<i32>().ok()).collect();         

        let distances: Vec<i32> = (&unusual_data_frame).windows(2).map(|w| w[1] - w[0] ).collect();

        if frame_check(&distances) {
            safe+= 1;
        } else if solve_distances(&distances, &unusual_data_frame) {
            safe+=1;
        }

    };
    safe
}

fn solve_distances(distances: &Vec<i32>, data_frames: &Vec<i32>) -> bool {

    let data_frame_copy = data_frames.clone();

    for i in 0..distances.len() {
        if asc_or_desc(&distances) {
            if distances[i] == 0 || distances[i] < 0 || distances[i] > 3 {
                let mut possible1 = data_frame_copy.clone();
                let mut possible2 = data_frame_copy.clone();
                possible1.remove(i+1);
                possible2.remove(i);
                if frame_check(&possible2.windows(2).map(|w| w[1] - w[0]).collect())
                    ||  frame_check(&possible1.windows(2).map(|w| w[1] - w[0]).collect())
                {
                    return true;
                };
            }
        } else {
            if distances[i] == 0 || distances[i] > 0 || distances[i] < -3 {
                let mut possible1 = data_frame_copy.clone();
                let mut possible2 = data_frame_copy.clone();
                possible1.remove(i+1);
                possible2.remove(i);
                if frame_check(&possible2.windows(2).map(|w| w[1] - w[0]).collect())
                    ||  frame_check(&possible1.windows(2).map(|w| w[1] - w[0]).collect())
                {
                    return true;
                };
            }
        }   
    }
    false
}

fn frame_check(frame: &Vec<i32>) ->  bool {

    if (frame.iter().all(|x | *x >= 1 && *x <= 3))
    || (frame.iter().all(|x | *x <= -1 && *x >= -3)) {
        return true;
    };
    false
}

fn asc_or_desc(data: &Vec<i32>) -> bool {
    let mut asc = 0;
    let mut desc = 0;
    for i in 0..data.len() {
        if data[i] > 0 {
            asc += 1;
        } else if data[i] < 0 {
            desc += 1;
        }
    };

    if asc > desc{
        true
    } else{
        false
    } 
}