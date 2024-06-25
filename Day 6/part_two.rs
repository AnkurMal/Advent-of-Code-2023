use std::fs;

fn main() {
    let binding = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = binding.lines().collect();

    let time: i64 = lines[0].split(' ').skip(1).collect::<String>().parse().unwrap();
    let dist: i64 = lines[1].split(' ').skip(1).collect::<String>().parse().unwrap();

    for i in 1..time {
        if (i*(time-i))>dist {
            println!("{}", time-2*i+1);
            break;
        }
    }
}
