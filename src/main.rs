use std::io;

fn main() {
    let mut a: i128 = 0;

    loop {
        let mut s = String::new();
        let n = io::stdin().read_line(&mut s).unwrap();

        if n == 0 {
            break;
        }

        if s.trim().is_empty() {
            continue;
        }
        let b: i128 = match s.trim().parse() {
            Ok(x) if x == -1 => break,
            Ok(x) if x > 0 => x,
            _ => {
                println!("NaN");
                return;
            }
        };

        a += b;
    }

    println!("{}", a);
}
