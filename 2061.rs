use std::io;

fn closing_tabs() {
    let mut input:String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut numbers:Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
    for _ in 0..numbers[1] {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();
        if input == "fechou" {
            numbers[0] += 1;
        } else {
            numbers[0] -= 1;
        }
    }
    println!("{}", numbers[0]);
}