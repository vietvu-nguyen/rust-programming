use std::io; //io (input/output) library from std library
use std::cmp::Ordering;
use rand::Rng; //Rng trait defines methods that random number generators
fn main(){
	println!("I am Optimus Prime. Please guess the number");
	let secret_number = rand::thread_rng().gen_range(1, 101);
	loop{
		println!("Input your guess, and I will spare you.");
		
		println!("{}", secret_number);
		// rand::thread_rng generate the random number
		let mut guess = String::new(); 
		// mut: mutable, String tring type provided by the standard library 
		// The :: syntax in the ::new line indicates that new is an associated function of the String type
		// created a mutable variable that is currently bound to a new, empty instance of a String.
		io::stdin().read_line(&mut guess)
			.expect("Failed to read line");
		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num, // data that return from match
	   		Err(_) => continue,
		}; // shadowing
		// The & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times.
		println!("You guessed {}", guess);
		match guess.cmp(&secret_number){
			Ordering::Less => println!("Too small"),
			Ordering::Greater => println!("Too Big"),
			Ordering::Equal =>{
				println!("Bingo");
				break;
			} 
		}
	}
	// match will check value return by cmp pattern
}