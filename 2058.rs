use std::io;

fn triangles_and_polygons() {
    let mut input:String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n:u32 = input.trim().to_string().parse().unwrap();
    println!("{}", n-2);
}