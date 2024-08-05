use std::io;

fn divisors() {
    let mut input:String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n:u32 = input.trim().to_string().parse().unwrap();
    for i in 1..n+1 {
        if n % i == 0 {
            println!("{}", i);
        }
    }
}