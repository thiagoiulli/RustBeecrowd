use std::io;

fn bean() {
    let mut input:String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let numbers = input.split_whitespace().map(|x| x.parse().unwrap()).collect::<Vec<i32>>();
    for i in 0..numbers.len() {
        if numbers[i] == 1 {
            println!("{}", i+1);
            break;
        }
    }
}