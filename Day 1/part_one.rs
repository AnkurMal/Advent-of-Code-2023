use std::fs::{self};

fn main() {
    let binding = fs::read_to_string("input.txt").unwrap();
    let file = binding.lines();
    let mut vec: Vec<u32> = Vec::new();
    let mut sum: u32 = 0;
    
    for i in file {
        for j in i.chars() {
            if j.is_numeric() {
                vec.push(j.to_digit(10).unwrap());
            }
        }
        let first = vec.first().unwrap();
        let last = vec.last().unwrap();
        sum += first*10+last;
        vec.clear();
    }

    println!("{}", sum);
}