use std::fs;

fn main() {
    let binding = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = binding.lines().collect();
    let mut nvec = Vec::new();
    let line_no = lines.len();
    

    for (i, line) in lines.iter().enumerate() {
        let mut num =  String::new();
        let str_len = line.len();

        for (j, letter) in line.chars().enumerate() {
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
                            if ch!='.' {
                                check  = true;
                                break;
                            }
                        }

                        if (ind+num_len)<str_len {
                            let ch = line.chars().nth(ind+num_len).unwrap();
                            if ch!='.' {
                                check  = true;
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
                                if ch!='.' && !ch.is_numeric() {
                                    check  = true;
                                    break 'outer;
                                }
                            }
                        }
        
                        if (i+1)<line_no {
                            let next_line = lines.get(i+1).unwrap();
                            for k in st-1..=end {
                                let ch = next_line.chars().nth(k).unwrap();
                                if ch!='.' && !ch.is_numeric() {
                                    check  = true;
                                    break 'outer;
                                }
                            }
                        }
                        break;
                    }
    
                    if check {
                        nvec.push(num.parse().unwrap());
                    }
                    num.clear();
                }
            }
        }
    }
    let sum: i32 = nvec.iter().sum();
    println!("{}", sum);
}