use std::io;

fn main() {
    let mut input:String = String::new();
    io::stdin().read_line(&mut input).expect("input");
    let mut target:i32 = input.trim().to_string().parse().unwrap();
    input.clear();
    io::stdin().read_line(&mut input).expect("input");
    let mut current:i32 = input.trim().to_string().parse().unwrap();
    println!("{}", target-current);
}