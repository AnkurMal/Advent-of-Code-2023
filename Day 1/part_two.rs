use std::fs;

fn get_num(num_str: &str) -> i32 {
    match num_str {
        "one"   | "1" => 1,
        "two"   | "2" => 2,
        "three" | "3" => 3,
        "four"  | "4" => 4,
        "five"  | "5" => 5,
        "six"   | "6" => 6,
        "seven" | "7" => 7,
        "eight" | "8" => 8,
        "nine"  | "9" => 9,
        _             => 0
    }
}

fn main() {
    let binding = fs::read_to_string("input.txt").unwrap();
    let file = binding.lines();
    let mut sum: i32 = 0;

    let list = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", 
                            "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    
    for i in file {
        let mut first = i.len();
        let mut fnum = "";

        let mut last = 0;
        let mut lnum = "";

        for j in list {
            let fi = i.find(j);
            if let Some(index) = fi {
                if index<=first {
                    first = index;
                    fnum = j;
                }
            }

            let li = i.rfind(j);
            if let Some(index) = li {
                if index>=last {
                    last = index;
                    lnum = j;
                }
            }
            
        }

        sum += get_num(fnum)*10+get_num(lnum);
    }
    println!("{}", sum);
}