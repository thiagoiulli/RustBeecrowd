use std::io;

fn busca_na_internet() {
    let mut numero:String = String::new();
    io::stdin().read_line(&mut numero).expect("input");
    let num:i32 = numero.trim().to_string().parse::<i32>().unwrap();
    println!("{}", num*4);
}