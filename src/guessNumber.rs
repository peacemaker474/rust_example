extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    let secrect_number = rand::thread_rng().gen_range(1..=100);

    println!("정답 번호: {}", secrect_number);

    loop {
        let mut number = String::new();

        io::stdin().read_line(&mut number)
            .expect("Failed to read line");

        let number: u32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match number.cmp(&secrect_number) {
            Ordering::Less => println!("너무 작습니다!"),
            Ordering::Greater => println!("너무 큽니다!"),
            Ordering::Equal => {
                println!("정답입니다!");
                break;
            }
        }
    }
}
