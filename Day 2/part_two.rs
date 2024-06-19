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

    fn product(&self) -> u32 {
        self.red*self.green*self.blue
    }
}

fn main() {
    let mut sum = 0;
    let binding = fs::read_to_string("input.txt").unwrap();
    let lines = binding.lines();

    for i in lines {
        let mut cube = Cube::new(0, 0, 0);
        let str = i.split(": ").last().unwrap();
        let token: Vec<&str> = str.split("; ").collect();

        for j in token.iter() {
            let split: Vec<&str> = j.split(" ").collect();

            for i in (1..split.len()).step_by(2) {
                let item = split.get(i).unwrap();
                if item.find("red").is_some() {
                    let red: u32 = split.get(i-1).unwrap().parse().unwrap();
                    if red>cube.red {
                        cube.red = red;
                    }
                }
                else if item.find("green").is_some() {
                    let green: u32 = split.get(i-1).unwrap().parse().unwrap();
                    if green>cube.green {
                        cube.green = green;
                    }
                }
                if item.find("blue").is_some() {
                    let blue: u32 = split.get(i-1).unwrap().parse().unwrap();
                    if blue>cube.blue {
                        cube.blue = blue;
                    }
                }
            }
        }
        sum += cube.product();
        cube.reset();
    }
    println!("{}", sum);
}