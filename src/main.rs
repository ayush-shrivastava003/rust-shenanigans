//https://stevedonovan.github.io/rust-gentle-intro/1-basics.html

//http://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/guessing-game.html
// thats where the code/tutorial is

extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

// prob best practice
//so is "::" just a way to signifiy a method or value of an object
// like in python you could do Object.value or Object.method()
//but in rust it would be Object::value or Object::method

macro_rules! printv {
	( $( $x:expr ),* ) => {
		{
			$(
				print!("{} ", $x);
			)*
			print!("\n");
		}
	};
}

fn function(arg: i64) -> i64{
	let mut g = arg*arg;
	printv!(g, 1);
	// println!("{}", g);
	g + 1
}

fn guess() {
	println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    // let value = function(2);
    println!("2^2 is {}", function(2));

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

fn listpids() {
	let list = ["0: guessing game (rng)", "1: unused"];
	let len = list.len();
	let mut i = 0;
	loop {
		if i >= len {
			break;
		}
		printv!(list[i]);
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
