use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
	
	let secret_number = rand::thread_rng().gen_range(1,101);

	loop{
		println!("Please input your guess:");
		let mut guess = String::new();
		
		let _foo = 5; // immutable
		let mut _bar = 5; // mutable
		
		io::stdin().read_line(&mut guess)
			.expect("Failed to read line");
		let guess: u32 = guess.trim().parse()
			.expect("Please type a number!");
			
		println!("You guessed : {}" , guess);
		
		match guess.cmp(&secret_number){
			Ordering::Less => println!("Guess is too small!"),
			Ordering::Greater => println!("Guess is too big!"),
			Ordering::Equal => {
				println!("You guessed correctly!");
				break;
			}
		}
	}
}
