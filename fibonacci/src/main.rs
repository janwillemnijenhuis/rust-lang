use std::io;
use std::process;

fn main() {
    let mut n = String::new();

    println!("Please input a positive integer:");

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line.");

    let mut n: u32 = n
        .trim()
        .parse()
        .expect("Please input a positive integer.");

    let mut n_2: u32 = 0;
    let mut n_1: u32 = 1;

    if n == 0 {
        println!("0");
        process::exit(1);
    }
    else if n == 1 {
        println!("1");
        process::exit(1);
    }

    n -= 2;
    while n != 0 {
        let n_0: u32 = fibonacci(n_1, n_2);
        n_2 = n_1;
        n_1 = n_0;
        n -= 1;
    }

    let result: u32 = fibonacci(n_1, n_2);

    println!("{}", result);
}

fn fibonacci(n_1: u32, n_2: u32) -> u32 {
    n_1 + n_2
}
