use std::io;

fn main() {
    let mut input = String::new();

    println!("Please input the temperature: ");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");

    let temperature: f32 = input
        .trim()
        .parse()
        .expect("Please type a number.");

    let mut type_temp = String::new();

    if temperature >= 50 as f32 {
        println!("Is your temperature in Fahrenheit (true/false):");

        io::stdin()
            .read_line(&mut type_temp)
            .expect("Failed to read line.");

        let type_temp: bool = type_temp
            .trim()
            .parse()
            .expect("Please respond with true/false.");

        if type_temp {
            let result: f32 = fahr_to_celc(temperature);
            println!("Celcius: {}", result as i32);
        }
        else {
            println!("Quite hot over here!");
            let result: f32 = celc_to_fahr(temperature);
            println!("Fahrenheit: {}", result as i32);
        }
    }
    else {
        println!("Is your temperature in Celcius (true/false):");

        io::stdin()
            .read_line(&mut type_temp)
            .expect("Failed to read line.");

        let type_temp: bool = type_temp
            .trim()
            .parse()
            .expect("Please respond with true/false.");

        if type_temp {
            let result: f32 = celc_to_fahr(temperature);
            println!("Fahrenheit: {}", result as i32);
        }
        else {
            println!("Quite cold over here!");
            let result: f32 = fahr_to_celc(temperature);
            println!("Celcius: {}", result as i32);
        }
    }
}

fn celc_to_fahr(c: f32) -> f32 {
    1.8 * c + 32.0
}

fn fahr_to_celc(f: f32) -> f32 {
    (f - 32.0) / 1.8
}
