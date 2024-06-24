use std::fs;

fn main() {
    let binding = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = binding.lines().collect();
    let mut err = 1;

    let time: Vec<i32> = lines[0].split(' ').skip(1).filter(|x| !x.is_empty()).map(|x| x.parse().unwrap()).collect();
    let dist: Vec<i32> = lines[1].split(' ').skip(1).filter(|x| !x.is_empty()).map(|x| x.parse().unwrap()).collect();

    for i in 0..time.len() {
        for j in 1..time[i] {
            if (j*(time[i]-j))>dist[i] {
                err *= time[i]-2*j+1;
                break;
            }
        }
    }
    
    println!("{}", err);
}