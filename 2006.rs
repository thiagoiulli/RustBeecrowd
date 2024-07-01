use std::io;
fn identifying_tea() {
    let mut input:String = String::new();
    let mut correct_tea:String = String::new();
    let mut soma: i32 = 0;
    io::stdin().read_line(&mut correct_tea).expect("input");
    io::stdin().read_line(&mut input).expect("input");
    correct_tea = correct_tea.trim().to_string();
    for num in input.split_whitespace(){
        if num == correct_tea {
            soma += 1;
        }
    }
    println!("{}", soma);
}