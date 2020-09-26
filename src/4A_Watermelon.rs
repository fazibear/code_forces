fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let x = buffer.trim().parse::<i64>().unwrap();
    if x % 2 == 0 && x != 2{
        println!("YES")
    } else {
        println!("NO")
    }
}
