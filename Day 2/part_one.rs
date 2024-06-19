use std::fs;

#[derive(Debug)]
struct Cube {
    red: u32,
    green: u32,
    blue: u32
}

impl Cube {
    fn new(red: u32, green: u32, blue: u32) -> Cube {
        Cube {red, green, blue}
    }

    fn reset(&mut self) {
        self.red = 0;
        self.green = 0;
        self.blue = 0;
    }
}

fn main() {
    let mut id_vec = Vec::new();
    let mut count = 0;
    let max = Cube::new(12, 13, 14);
    let binding = fs::read_to_string("input.txt").unwrap();
    let lines = binding.lines();

    for i in lines {
        let mut smaller = true;
        let mut cube = Cube::new(0, 0, 0);
        let str = i.split(": ").last().unwrap();
        let token: Vec<&str> = str.split("; ").collect();
        count += 1;

        for j in token.iter() {
            let split: Vec<&str> = j.split(" ").collect();

            for i in (1..split.len()).step_by(2) {
                let item = split.get(i).unwrap();
                if item.find("red").is_some() {
                    cube.red = split.get(i-1).unwrap().parse().unwrap();
                }
                else if item.find("green").is_some() {
                    cube.green = split.get(i-1).unwrap().parse().unwrap();
                }
                if item.find("blue").is_some() {
                    cube.blue = split.get(i-1).unwrap().parse().unwrap();
                }
            }
            
            if cube.red>max.red || cube.green>max.green || cube.blue>max.blue {
                smaller = false;
                continue;
            }
            cube.reset();
        }
        if smaller {
            id_vec.push(count);
        }
    }
    let sum: i32 = id_vec.iter().sum();
    println!("{}", sum);
}