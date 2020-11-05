use my_ext::kt_ext::KtStd;
use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io::stdin;

fn main() {
    println!("Guess the number!");
    thread_rng().gen_range(1, 101).let_ref(|secret_number| loop {
        println!("Please input your guess.");
        let guess: Option<u32> = String::new().also_mut(|guess| {
                stdin().read_line(guess).expect("Failed to read line");
            }).trim_end().parse().let_owned(|it| match it {
                Ok(num) => Some(num),
                Err(_) => { println!("wrong number"); None }
            });

        if let Some(x) = guess {
            println!("You guessed: {}", x);
            match x.cmp(secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("You win!");
                    break;
                }
            }
        }
    })
}
