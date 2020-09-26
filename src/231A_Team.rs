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

    let mut answer = 0;

    for _ in 0..i {
        let mut sure = 0;
        for _ in 0..3 {
            sure += scan.next::<usize>();
        }
        if sure > 1 {
            answer += 1;
        }
    }
    println!("{}", answer);
}
