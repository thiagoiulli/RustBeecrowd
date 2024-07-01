use std::io;

fn sequencia_secreta() {
    let mut input:String = String::new();
    io::stdin().read_line(&mut input);
    let n:u32 = input.trim().to_string().parse().unwrap();
    let mut num:u32;
    let mut num_antigo:u32 = 0;
    input.clear();
    let mut soma:u32 = 0;
    for i in 0..n {
        io::stdin().read_line(&mut input);
        num = input.trim().to_string().parse::<u32>().unwrap();
        input.clear();
        if num != num_antigo {
            soma += 1;
        }
        num_antigo = num;
    }
    println!("{}", soma);
}