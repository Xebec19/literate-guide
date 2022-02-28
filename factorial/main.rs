use std::io;

fn main(){
    let mut input = String::new();
    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");
    let input: usize = input.trim().parse()
    .expect("Input should be a number");
    println!("Result is\n{}",calc_fact(input));
}
fn calc_fact(x:usize) -> usize{
    if(x < 0){
        0
    } else if (x == 1){
        1
    } else { x * calc_fact(x-1) }
}