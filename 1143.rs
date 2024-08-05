use std::io;

fn square_and_cubic() {
    let mut input:String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n:u32 = input.trim().to_string().parse().unwrap();

    for i in 1..n+1 {
        println!("{} {} {}", i, i*i, i*i*i);
    }
}