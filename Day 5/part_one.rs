use std::fs;

fn main() {
    let binding = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = binding.lines().collect();

    let seed: String = lines[0].split(": ").skip(1).collect();
    let mut seed: Vec<i64> = seed.split(' ').map(|x| x.parse().unwrap()).collect();
    let mut ivec = Vec::new();
    let lines: Vec<&str> = lines.into_iter().skip(2).filter(|x| x.is_empty() || x.chars().next().unwrap().is_numeric()).collect();

    for line in lines.iter() {
        if !line.is_empty() {
            let vec: Vec<i64> = line.split(' ').map(|x| x.parse().unwrap()).collect();

            for (i, s) in seed.clone().iter().enumerate() {
                let st = vec[1];
                let end = st+vec[2];
                if *s>=st && *s<end {
                    if !ivec.contains(&i) {
                        seed[i] = s-st+vec[0];
                        ivec.push(i);
                    }
                }
            }
        }
        else {
            ivec.clear();
        }
    }
    println!("{}", seed.iter().min().unwrap());
}