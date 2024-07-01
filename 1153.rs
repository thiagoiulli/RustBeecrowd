use std::io;

fn simple_factorial() {
    let mut input:String = String::new();
    io::stdin().read_line(&mut input);
    let num:u32 = input.trim().to_string().parse().unwrap();
    let mut total:u32 = 1;
    for n in 0..num {
        total = total * (num -n);
    }
    println!("{}", total);
}