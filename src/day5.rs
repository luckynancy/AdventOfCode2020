use std::io;
use crate::utils::read_lines_from_file;

pub fn day_five() -> io::Result<()> {
    let path = "./data/day5.txt";

    match read_lines_from_file(path) {
        Ok(lines) => {
            // Calculate seat IDs
            let seat_ids: Vec<i32> = lines.iter().map(|line| calculate_seat_id(&line)).collect();

            // Sort seat IDs
            let mut sorted_seat_ids = seat_ids.clone();
            sorted_seat_ids.sort();

            println!("Part I : Maximum Seat ID: {:?}", sorted_seat_ids.last());

            // Find seat IDs with a difference of 2
            let mut pairs: Vec<(i32, i32)> = Vec::new();
            for i in 0..sorted_seat_ids.len() - 1 {
                if sorted_seat_ids[i + 1] - sorted_seat_ids[i] == 2 {
                    pairs.push((sorted_seat_ids[i], sorted_seat_ids[i + 1]));
                }
            }

            // Output pairs with difference of 2
            for (id1, _id2) in pairs {
                println!("Part II; Seat IDs is: {} ", id1+1, );
            }
            
        }
        Err(e) => {
            eprintln!("Error reading file: {:?}", e);
        }
    }
    Ok(())
}

fn decode_seat(boarding_pass: &str) -> (i32, i32) {
    let row = decode_row(&boarding_pass[..7]);
    let col = decode_col(&boarding_pass[7..]);

    (row, col)
}

fn decode_row(row_part: &str) -> i32 {
    let mut lower = 0;
    let mut upper = 127;

    for ch in row_part.chars() {
        let mid = (lower + upper) / 2;
        match ch {
            'F' => upper = mid,
            'B' => lower = mid + 1,
            _ => panic!("Invalid character in row part"),
        }
    }

    lower
}

fn decode_col(col_part: &str) -> i32 {
    let mut lower = 0;
    let mut upper = 7;

    for ch in col_part.chars() {
        let mid = (lower + upper) / 2;
        match ch {
            'L' => upper = mid,
            'R' => lower = mid + 1,
            _ => panic!("Invalid character in column part"),
        }
    }

    lower
}

fn calculate_seat_id(line: &str) -> i32 {
    let (row, col) = decode_seat(line);
    row * 8 + col
}