use std::io;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use crate::utils::read_lines_from_file;

struct Passport {
    fields: HashMap<String, String>,
}

impl Passport {
    fn has_required_keys(&self) -> bool {
        let required_keys: HashSet<&str> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .iter()
            .cloned()
            .collect();
        required_keys.iter().all(|&key| self.fields.contains_key(key))
    }

    fn is_valid(&self) -> bool {
        valid_byr(self.fields.get("byr").unwrap_or(&"".to_string())) &&
        valid_iyr(self.fields.get("iyr").unwrap_or(&"".to_string())) &&
        valid_eyr(self.fields.get("eyr").unwrap_or(&"".to_string())) &&
        valid_hgt(self.fields.get("hgt").unwrap_or(&"".to_string())) &&
        valid_hcl(self.fields.get("hcl").unwrap_or(&"".to_string())) &&
        valid_ecl(self.fields.get("ecl").unwrap_or(&"".to_string())) &&
        valid_pid(self.fields.get("pid").unwrap_or(&"".to_string()))
    }
}

pub fn day_four() -> io::Result<()> {
    let path = "./data/day4.txt";

    match read_lines_from_file(path) {
        Ok(lines) => {
            let passport_data = clean_data(&lines);
            let passports: Vec<Passport> = passport_data
                .into_iter()
                .map(|data| {
                    let fields = data.split_whitespace()
                        .map(|kv| {
                            let mut split = kv.split(':');
                            (split.next().unwrap().to_string(), split.next().unwrap().to_string())
                        })
                        .collect();
                    Passport {fields}
                })
                .collect();

            let valid_passports1: Vec<&Passport> = passports
                .iter()
                .filter(|passport| passport.has_required_keys())
                .collect();
            println!("Part I: Number of valid passports: {}", valid_passports1.len());

            let valid_passports2: Vec<&Passport> = passports
                .iter()
                .filter(|passport| passport.is_valid())
                .collect();

            println!("Part II: Number of valid passports: {}", valid_passports2.len());
            
        }
        Err(e) => {
            eprintln!("Error reading file: {:?}", e);
        }
    }
    
    Ok(())
}

fn clean_data(lines: &Vec<String>) -> Vec<String> {
    let mut blocks: Vec<String> = Vec::new();
    let mut current_block = String::new();

    for line in lines {

        if line.trim().is_empty() {
            // We found a blank line, push the accumulated block to the vector
            if !current_block.is_empty() {
                blocks.push(current_block.trim_end().to_string());
                current_block.clear();
            }
        } else {
            // Append non-blank line to current_block
            current_block.push_str(&line);
            current_block.push(' '); // Add newline that was removed by `lines()`
        }

    }
    // Push the last accumulated block if there's any left
    if !current_block.is_empty() {
        blocks.push(current_block.trim_end().to_string());
    }
    blocks
}



fn valid_byr(byr: &str) -> bool {
    match byr.parse::<i32>() {
        Ok(year) => year >= 1920 && year <= 2002,
        Err(_) => false,
    }
}

fn valid_iyr(iyr: &str) -> bool {
    match iyr.parse::<i32>() {
        Ok(year) => year >= 2010 && year <= 2020,
        Err(_) => false,
    }
}

fn valid_eyr(eyr: &str) -> bool {
    match eyr.parse::<i32>() {
        Ok(year) => year >= 2020 && year <= 2030,
        Err(_) => false,
    }
}

fn valid_hgt(hgt: &str) -> bool {
    if let Some(cm_pos) = hgt.find("cm") {
        if let Ok(height) = hgt[..cm_pos].parse::<i32>() {
            return height >= 150 && height <= 193;
        }
    } else if let Some(in_pos) = hgt.find("in") {
        if let Ok(height) = hgt[..in_pos].parse::<i32>() {
            return height >= 59 && height <= 76;
        }
    }
    false
}

fn valid_hcl(hcl: &str) -> bool {
    let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    re.is_match(hcl)
}

fn valid_ecl(ecl: &str) -> bool {
    matches!(ecl, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")
}

fn valid_pid(pid: &str) -> bool {
    let re = Regex::new(r"^\d{9}$").unwrap();
    re.is_match(pid)
}
