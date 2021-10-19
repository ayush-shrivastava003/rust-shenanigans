//https://stevedonovan.github.io/rust-gentle-intro/1-basics.html

//http://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/guessing-game.html
// thats where the code/tutorial is

//how do i use rustc with cargo packages
// dont
// just ctrl+enter to run, check .replit for exact bash

//if you want to build without running, 'cargo build' will put the binary into target/debug/rust-shenanigans

// can u invite Alfaheimr
//ok
//also you have access to push to githu
// nice, check out what i've done

extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

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

fn main() {
	// prints prompt
    println!("enter program id:");
	// initializes a variable
	let mut id = String::new();
	// gets input
	io::stdin().read_line(&mut id).expect("Falied to read line");
	// sets if the console should be cleared ()
	let ndb : bool = true;
	if !ndb {
		// clears the console
		println!("\x1bc");
	}
	// removes leading and trailing whitespaces
	let id = id.trim();
	// matches given id to processes
	match id {
		"0" => guess(),
		_ => println!("unrecognized id"),
	}
}