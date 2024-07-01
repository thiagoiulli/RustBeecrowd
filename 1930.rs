use std::io;
fn eletrical_outlet() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("input");
    let nums: Vec<i32> = line.trim().split(' ').flat_map(str::parse::<i32>).collect::<Vec<_>>();
    let mut total: i32 = 0;
    for num in nums{
        total += num;
    }
    total -= 3;
    println!("{}", total);
}