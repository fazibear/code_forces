#[allow(unused_imports)]
use std::io::{BufWriter, stdin, stdout, Write};

#[derive(Default)]
struct Scanner {
    buffer: Vec<String>
}
impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}

fn main() {
    let mut scan = Scanner::default();
    let i = scan.next::<usize>();

    for _ in 0..i {
        let word = scan.next::<String>();
        let len = word.len();
        let chars: Vec<char> = word.chars().collect();
        if len > 10 {
            println!("{}{}{}", chars[0], len - 2, chars[len-1]);
        } else {
            println!("{}", word);
        }
    }
}
