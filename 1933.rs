use std::io;

fn tri_du() {
    let mut input:String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let numbers:Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
    if numbers[0] == numbers[1] {
        println!("{}", numbers[0]);
    }
    else{
        if numbers[0] > numbers[1] {
            println!("{}", numbers[0]);
        }
        else{
            println!("{}", numbers[1]);
        }
    }
}