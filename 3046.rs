use std::io;

fn domino() {
    let mut input:String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n:u32 = input.trim().to_string().parse().unwrap();
    println!("{}", (n+1)*(n+2)/2);
}