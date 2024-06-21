use std::fs;

fn main() {
    let binding = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = binding.lines().collect();
    
    let mut nvec: Vec<usize> = vec![1; lines.len()];
    
    for (i, line) in lines.iter().enumerate() {
        let vec: Vec<&str> = line.split(" | ").collect();
        let mut points = 0;

        let lvec: Vec<&str> = vec[0].split(": ").collect();
        let lvec: Vec<&str> = lvec[1].split(' ').filter(|x| !x.is_empty()).collect();
        let rvec: Vec<&str> = vec[1].split(' ').filter(|x| !x.is_empty()).collect();
        
        for lnum in lvec.iter() {
            if rvec.contains(lnum) {
                points += 1;
            }
        }

        for _ in 0..nvec[i] {
            for k in i+1..=i+points {
                nvec[k] += 1;
            }
        }
    }
    let sum: usize = nvec.iter().sum();
    println!("{}", sum);
}
