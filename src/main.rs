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

    let mut groups = Vec::new();

    for _ in 0..i {
        groups.push(scan.next::<usize>());
    }

    let mut sum = 0;

    for a in groups {
        sum += a;
    }

    let mut answer = sum / 4;

    if sum % 4 != 0 {
        answer += 1;
    }

    println!("{}", answer);
}
