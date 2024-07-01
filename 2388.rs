use std::io;

fn tacografo() {
    let mut input:String = String::new();
    io::stdin().read_line(&mut input);
    let num:u32 = input.trim().to_string().parse().unwrap();
    let mut soma:u32 = 0;
    for _ in 0..num {
        input.clear();
        io::stdin().read_line(&mut input);
        let dados:Vec<&str> = input.trim().split_whitespace().collect();
        soma += &dados[0].trim().to_string().parse::<u32>().unwrap() * &dados[1].trim().to_string().parse::<u32>().unwrap();
    }
    println!("{}", soma);
}