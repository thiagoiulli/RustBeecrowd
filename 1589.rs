use std::io;

fn bob_conduit() {
    let mut input:String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t:u32 = input.trim().to_string().parse().unwrap();
    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let numbers:Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
        println!("{}", numbers[0]+numbers[1]);
    }
}