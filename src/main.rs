use std::io;
use std::time;

fn is_prime(i: &u128) -> bool {
    let now: time::Instant = time::Instant::now();
    let mut is_prime: bool = true;
    for number in (2..*i).rev() {
        if *i % number  == 0 {
            is_prime = false;
            break
        }
    }
    println!("{}", now.elapsed().as_millis());
    is_prime
}


fn main() {
    println!("Is it prime?");

    let mut number = String::new();

    io::stdin().read_line(&mut number)
        .expect("failed to read line");

    loop {
        let number: u128 = match number.trim().parse() {
            Ok(num) => num,
            Err(_e) => {
                println!("Not a number. Type a number.");
                continue
            }
        };
        match is_prime(&number) {
            false => {
                println!("No");
                break
            },
            true => {
                println!("Yes");
                break
            }
        }
    }
}
