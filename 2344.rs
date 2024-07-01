use std::io;

fn notas_de_prova() {
    let mut input:String = String::new();
    io::stdin().read_line(&mut input);
    let nota:u32 = input.trim().to_string().parse().unwrap();
    match nota {
        0 => println!("E"),
        1..=35 => println!("D"),
        36..=60 => println!("C"),
        61..=85 => println!("B"),
        86..=100 => println!("A"),
        _ => println!("ERRO"),
    }
}