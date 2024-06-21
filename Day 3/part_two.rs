use std::fs;

#[derive(Clone, Copy)]
struct Gear {
    num: i32,
    st_i: usize,
    st_j: usize
}

impl Gear {
    fn new(num: i32, st_i: usize, st_j: usize) -> Self {
        Gear {num, st_i, st_j}
    }
}

fn main() {
    let binding = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = binding.lines().collect();
    let mut gvec: Vec<Gear> =  Vec::new();
    let mut sum = 0;
    let line_no = lines.len();
    

    for (i, line) in lines.iter().enumerate() {
        let mut num =  String::new();
        let str_len = line.len();

        for (j, letter) in line.chars().enumerate() {
            let mut star_i = i;
            let mut star_j = 0;
            let is_num = letter.is_numeric();
            if is_num {
                num.push(letter);
            }

            if !is_num || j==(str_len-1) {
                if !num.is_empty() {
                    let num_len = num.len();
                    let mut ind = j-num_len;
                    if j==(str_len-1) && letter.is_numeric() {
                        ind += 1;
                    }
                    let mut check = false;

                    'outer: loop {
                        if ind>0 {
                            let ch = line.chars().nth(ind-1).unwrap();
                            if ch=='*' {
                                check  = true;
                                star_j = ind-1;
                                break;
                            }
                        }

                        if (ind+num_len)<str_len {
                            let ch = line.chars().nth(ind+num_len).unwrap();
                            if ch=='*' {
                                check  = true;
                                star_j = ind+num_len;
                                break;
                            }
                        }

                        let mut st = ind;
                        let mut end = ind+num_len;
        
                        if st==0 {
                            st += 1;
                        }
                        if end==str_len {
                            end -= 1;
                        }
        
                        if i>0 {
                            let prev_line = lines.get(i-1).unwrap();
                            for k in st-1..=end {
                                let ch = prev_line.chars().nth(k).unwrap();
                                if ch=='*' {
                                    check  = true;
                                    star_j = k;
                                    star_i = i-1;
                                    break 'outer;
                                }
                            }
                        }
        
                        if (i+1)<line_no {
                            let next_line = lines.get(i+1).unwrap();
                            for k in st-1..=end {
                                let ch = next_line.chars().nth(k).unwrap();
                                if ch=='*' {
                                    star_j = k;
                                    star_i = i+1;
                                    check  = true;
                                    break 'outer;
                                }
                            }
                        }
                        break;
                    }
    
                    if check {
                        let num = num.parse::<i32>().unwrap();
                        let mut push = true;
                        for (i, g) in gvec.clone().iter().enumerate() {
                            if g.st_j==star_j && g.st_i==star_i{
                                sum += num*g.num;
                                gvec.remove(i);
                                push = false;
                            }
                        }
                        if push {
                            let gear = Gear::new(num, star_i, star_j);
                            gvec.push(gear);
                        }
                    }
                    num.clear();
                }
            }
        }
    }
    println!("{}", sum);
}
