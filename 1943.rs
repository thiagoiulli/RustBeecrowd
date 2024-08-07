use std::io;

fn top_n() {
    let mut input:String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n:u32 = input.trim().to_string().parse().unwrap();
    match n{
        1=>println!("Top 1"),
        2|3=>println!("Top 3"),
        4|5=>println!("Top 5"),
        5..=10=>println!("Top 10"),
        11..=25=>println!("Top 25"),
        26..=50=>println!("Top 50"),
        51..=100=>println!("Top 100"),
        _=>println!("Erro"),
    }
}