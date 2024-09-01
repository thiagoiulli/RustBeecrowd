use std::io;

fn nota_esquecida() {
    let mut input:String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let a:u32 = input.trim().to_string().parse().unwrap();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let m:u32 = input.trim().to_string().parse().unwrap();
    println!("{}", 2*m-a);
}