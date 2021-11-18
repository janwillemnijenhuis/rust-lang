use std::io::stdin;

pub fn main() {
    let mut input = String::new();
    println!("Please input a sentence to be converted to pig-latin: ");
    stdin()
        .read_line(&mut input)
        .expect("Please provide a valid string input.");
    let split = input
        .trim()
        .split_whitespace();
    let vowels: Vec<char> = vec!['a','e', 'i', 'o', 'u'];
    for word in split {
        let mut v: Vec<char> = Vec::new();
        let mut counter: usize = 0;
        let mut first: char = ' ';
        for c in word.chars() {
            if !(vowels.contains(&c)) && counter == 0 {
                first = c;
            }
            else if vowels.contains(&c) && counter == 0 {
                first = 'h';
                v.push(c);
            }
            else {
                v.push(c);
            }
            counter += 1;
        }
        let mut s: String = v.into_iter().collect();
        print!("{}{}{}{} ",s,'-',first,"ay");
    }
}
