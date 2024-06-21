use std::fs;

fn main() {
    let binding = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = binding.lines().collect();
    let mut sum =  0;
    
    for line in lines {
        let vec: Vec<&str> = line.split(" | ").collect();
        let mut points = 0;

        let lvec: Vec<&str> = vec[0].split(": ").collect();
        let lvec: Vec<&str> = lvec[1].split(' ').filter(|x| !x.is_empty()).collect();
        let rvec: Vec<&str> = vec[1].split(' ').filter(|x| !x.is_empty()).collect();
        
        for lnum in lvec.iter() {
            if rvec.contains(lnum) {
                match points {
                    0 => {points = 1},
                    _ => {points *= 2}
                }
            }
        }
        sum += points;
    }
    println!("{}", sum);
}