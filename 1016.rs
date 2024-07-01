use std::io;

fn distance() {
    let mut input:String = String::new();
    io::stdin().read_line(&mut input);
    let num:u32 = input.trim().to_string().parse().unwrap();
    println!("{} minutos", num*2);
}