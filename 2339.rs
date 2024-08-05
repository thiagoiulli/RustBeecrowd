use std::io;

fn avioes_papel() {
    let mut input:String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let numbers:Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
    if numbers[0]*numbers[2] <= numbers[1] {
        println!("S");
    } else {
        println!("N");
    }
}