use std::fs;

fn main() {
    let binding = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = binding.lines().collect();
    let mut ivec = Vec::new();

    let seed: String = lines[0].split(": ").skip(1).collect();
    let seed: Vec<i64> = seed.split(' ').map(|x| x.parse().unwrap()).collect();
    
    let lines: Vec<&str> = lines.into_iter().skip(2).filter(|x| x.is_empty() || x.chars().next().unwrap().is_numeric()).collect();
    
    let mut seed1 = Vec::new();
    let mut seed2 = Vec::new();
    for i in (0..seed.len()).step_by(2) {
        seed1.push(seed[i]+seed[i+1]-1);
        seed2.push(seed[i]);
    }
    let mut size: usize = seed1.len();

    for line in lines.iter() {
        if !line.is_empty() {
            let mut i = 0;
            let vec: Vec<i64> = line.split(' ').map(|x| x.parse().unwrap()).collect();

            while i<size {
                let st = vec[1];
                let end = st+vec[2];
                if seed2[i]>=st && seed1[i]<end {
                    if !ivec.contains(&seed1[i]) {
                        seed1[i] = seed1[i]-st+vec[0];
                        seed2[i] = seed2[i]-st+vec[0];
                        ivec.push(seed1[i]);
                    }
                }
                else if seed2[i]>=st && seed2[i]<end && seed1[i]>=end {
                    if !ivec.contains(&seed1[i]) {
                        seed2[i] = seed2[i]-st+vec[0];
                        seed2.insert(i+1, end);
                        seed1.insert(i, end-1-st+vec[0]);
                        ivec.push(seed1[i]);
                        size += 1;
                    }
                }
                else if seed1[i]>=st && seed1[i]<end && seed2[i]<st {
                    if !ivec.contains(&seed1[i]) {
                        seed1[i] = seed1[i]-st+vec[0];
                        seed1.insert(i+1, st-1);
                        seed2.insert(i, vec[0]);
                        ivec.push(seed1[i]);
                        size += 1;
                    }
                }
                else if seed2[i]<st && seed1[i]>=end {
                    if !ivec.contains(&seed1[i]) {
                        seed2.insert(i, vec[0]);
                        seed1.insert(i, end-1-st+vec[0]);
                        seed1.insert(i+1, st-1);
                        seed2.insert(i+2, end);
                        ivec.push(seed1[i]);
                        size += 2;
                    }
                }
                i += 1;
            }
        }
        else {
            ivec.clear();
        }
    }
    println!("{}", seed2.iter().min().unwrap());
}