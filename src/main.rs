//https://stevedonovan.github.io/rust-gentle-intro/1-basics.html

//http://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/guessing-game.html
// thats where the code/tutorial is

// imports packages
extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

/*
best practices:
* have all code below package imports
* don't use i64 unless necessary b/c any size integer can be converted to i64 but i64 cannot be converted to any other size integer

handy notes:
"::" is how you access a static method of a type, there is more to this that you should look in the documentation for
"." is how you access instance methods
"let" is how you define a variable
"let mut" is how you define a mutable variable
"name [: type]" is how you optionally specify a type for the variable
variables can be overwritten e.x. to change their type by re-defining them using another "let" keyword
"loop" is the keyword for an infinite loop, you have to specify your own exit conditions using the "break" statement
"continue" skips to the next iteration of the loop
"while" and "for" loops do exist but it's unclear to me what their limitations are
loops can have labels "'label' [loop syntax]" this allows continues in nested loops to apply to loops that the continue isn't in e.x.:
'loop1' loop {
	'loop2' loop {
		if breakout2 {
			break 'loop2'
		}
		if breakout1 {
			break 'loop1'
		}
	}
}
*/

// a print operation that can accept any number of arguments then prints them
macro_rules! printv {
	// vodoo
	( $( $x:expr ),* ) => {
		// more vodoo
		{
			// even more vodoo
			$(
				// prints the argument
				print!("{} ", $x);
			)*
			// prints a newline
			print!("\n");
		}
	};
}

fn pow(n: i32, power: i32) -> i32 {
	let mut f = 1;
	let mut c = 0;
	loop {
		if c >= power {
			break;
		}
		f *= n;
		c += 1;
	}
	return f;
}

fn function(arg: i32) -> i32{
	let g = pow(arg, 2);
	printv!(g);
	return g + 1;
}

fn guess() {
	println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                break;
            }
        }
    }
}

// lists program ids
fn listpids() {
	// holds ids
	let list = ["0: guessing game (rng)", "1: unused"];
	// length of id list
	let len = list.len();
	// loop variable
	let mut i = 0;
	// loops
	loop {
		// checks that i is less than the length of the id list
		if i >= len {
			break;
		}
		// prints the ith item of the id list
		printv!(list[i]);
		// increments i
		i += 1;
	}
}

fn main() {
	// sets if the console should be cleared ()
	let ndb : bool = true;
	loop {
		// prints prompt
		println!("enter program id (-1 to exit, -2 for id list):");
		// initializes a variable
		let mut id = String::new();
		// gets input
		io::stdin().read_line(&mut id).expect("Falied to read line");
		if !ndb {
			// clears the console
			println!("\x1bc");
		}
		// removes leading and trailing whitespaces
		let id = id.trim();
		// matches given id to processes
		match id {
			"-2" => listpids(),
			"-1" => {break},
			"0" => guess(),
			_ => println!("unrecognized id"),
		}
	}
}
