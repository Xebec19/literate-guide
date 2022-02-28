use std::io;

fn main(){
    let mut input = String::new();
    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");
    let input: usize = input
    .trim()
    .parse()
    .expect("Input should be a number");
    println!("Result is\n{}",convert_fahrenheit_to_celsius(input));
}

fn convert_fahrenheit_to_celsius (x:usize) -> usize {
    (5/9) * (x - 32)
}