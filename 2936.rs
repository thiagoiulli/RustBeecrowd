use std::io;

fn how_much_cassava() {
    let mut total:u32 = 225;
    let mut n:u32 = get_input_to_int();
    total += n * 300;
    let mut n:u32 = get_input_to_int();
    total += n * 1500;
    let mut n:u32 = get_input_to_int();
    total += n * 600;
    let mut n:u32 = get_input_to_int();
    total += n * 1000;
    let mut n:u32 = get_input_to_int();
    total += n * 150;
    println!("{}", total);
}

fn get_input_to_int() -> u32 {
    let mut input:String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n:u32 = input.trim().to_string().parse().unwrap();
    return n;
}