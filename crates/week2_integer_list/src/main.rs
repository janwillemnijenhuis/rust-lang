use integer_list::return_mean;
use std::collections::HashMap;
use std::io::stdin;

pub fn main() {
    let mut ints: Vec<i32> = Vec::new(); // init empty vector
    let mut input = String::new(); // init empty string
    let mut correct: bool = false; // init bool for correct input
                                   // checks if the input is indeed an integer separated list
    while !correct {
        println!("Please enter a list of integers, seperated by whitespaces: ");
        stdin()
            .read_line(&mut input)
            .expect("Please provide a valid string.");
        let split = input.trim().split_whitespace();
        for s in split {
            let test = s.trim().parse::<i32>();
            let valid: bool = match test {
                Ok(ok) => true,
                Err(e) => false,
            };
            if valid == false {
                correct = false;
                break;
            } else {
                let value: i32 = s.trim().parse().unwrap();
                ints.push(value);
            }
        }
        correct = true;
    }
    let mut ints2 = ints.clone();
    let mut ints3 = ints.clone();
    println!("The mean is: {:.2}", return_mean(ints));
    println!("The median is: {:.2}", return_median(ints2));
    println!("The mode is: {:.2}", return_mode(ints3));
}

pub fn return_median(mut v: Vec<i32>) -> f64 {
    let vec_len: f64 = v.len() as f64;
    for i in 0..(vec_len - 2.0) as usize {
        let mut p1 = v[i];
        let mut p2 = v[i + 1];
        if p1 > p2 {
            v[i] = p2;
            v[i + 1] = p1;
        }
    }
    if vec_len % 2.0 == 0.0 {
        let mut idx1: usize = (vec_len / 2.0 - 1.0) as usize;
        let mut idx2: usize = (vec_len / 2.0) as usize;
        let mut val1: f64 = v[idx1] as f64;
        let mut val2: f64 = v[idx2] as f64;
        return (val1 + val2) / 2.0;
    } else {
        let mut idx: usize = (vec_len / 2.0 - 1.0) as usize;
        let mut val: f64 = v[idx] as f64;
        return val;
    }
}

pub fn return_mode(mut v: Vec<i32>) -> f64 {
    let mut mode_count = HashMap::new();
    for i in v {
        if !mode_count.contains_key(&i) {
            mode_count.insert(i, 1);
        } else {
            let mut value: i32 = mode_count.get(&i).unwrap() + 1;
            mode_count.insert(i, value);
        }
    }
    let mut max_val: i32 = 0;
    let mut max_num: i32 = 0;
    for (key, value) in mode_count {
        if value > max_val {
            max_val = value;
            max_num = key;
        }
    }
    return max_num as f64;
}
