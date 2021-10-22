//https://stevedonovan.github.io/rust-gentle-intro/1-basics.html

//http://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/guessing-game.html
// thats where the code/tutorial is

// imports packages
extern crate rand;
extern crate rust_shenanigans as mods;

use std::io;
use std::cmp::Ordering;
use rand::Rng;
use std::convert::TryFrom;
use mods::utility::*;
use mods::nurnet::*;
use std::fs::File;
use std::io::Read;

/*
best practices:
* have all code below package imports
* don't use i64 unless necessary b/c any size integer can be converted to i64 but i64 cannot be converted to any other size integer (without being annoying)
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

fn randrange(n1: i32, n2: i32) -> i32 {
	return rand::thread_rng().gen_range(n1, n2);
}

fn guess_num() {
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

// runs a guessing game where the player tries to guess a word
fn guess_word() {
	let mut file = File::open("src/wordguesswords.txt").expect("FAILURE");
	let mut contents = String::new();
	file.read_to_string(&mut contents).expect("FAILURE");
	let contents: Vec<_> = contents.split("\n").collect();
	// tells the player about the game
	println!("guess the word in as few guesses as possible. each incorrect guess reveals a random letter, but also decreases your score");
	// list of words
	let words = contents;
	// picks the word that the player will be guessing
	let e = randrange(0, i32::try_from(words.len()).unwrap());
	let word = words[e as usize].to_lowercase();
	// gets the characters in the word
	let mut wc = word.chars();
	// puts the characters in a vector so that .len() is valid
	let mut v = Vec::new();
	loop {
		let n = wc.next();
		match n {
			Some(n) => v.push(n),
			None => {break},
		}
	}
	// creates a vector that holds all unrevealed positions
	let mut cl = Vec::new();
	let mut i = 1;
	loop {
		if i >= v.len()-1 {
			break;
		}
		cl.push(i);
		i += 1;
	}
	// prints information about the word
	println!("word length: {}, first letter: {}, last letter: {}", v.len(), v[0], v[v.len()-1]);
	// calculates the starting score
	let mut score = (v.len()-2)*5;
	// counts the number of incorrect guesses made
	let mut i = 0;
	loop {
		if v.len()-2-i <= 0 {
			println!("you lost :(");
			return;
		}
		// prints the number of remaining guesses
		println!("guesses remaining: {} enter your guess:", v.len()-2-i);
		// gets player input
		let mut g = String::new();
		io::stdin().read_line(&mut g).expect("Failed to read line");
		g = g.trim().to_string().to_lowercase();
		// prints the players guess
		println!("you guessed: {}", g);
		// checks if the guess was correct
		if g == word {
			break;
		} else {
			// reveals a random letter
			if score > 5 {
				// gets the random index
				let e = randrange(0, i32::try_from(cl.len()).unwrap());
				// gets the position to be revealed
				let pos = cl[e as usize];
				// format stuff
				let mut p = "th";
				if pos < 4 {
					p = "rd";
				}
				if pos < 3 {
					p = "nd";
				}
				if pos < 2 {
					p = "st";
				}
				// gets the letter
				let l = v[pos];
				println!("the {}{} letter is: {}", pos, p, l);
				// discards the position that was revealed
				let mut nv = Vec::new();
				let mut lv = 0;
				loop {
					if lv >= cl.len() {
						break;
					}
					if lv == pos {
						lv += 1;
						continue;
					}
					nv.push(cl[lv]);
					lv += 1;
				}
				cl = nv;
			}
		}
		// increments the number of incorrect guesses
		i += 1;
		// decreases the score
		score -= 5;
	}
	// calculates the percentage of the maximum score the user got
	let pscore = (score*100)/((v.len()-2)*5);
	// prints the score
	println!("your score is: {}", pscore);
}

// generates random numbers
fn num_gen() {
	// gets lower and upper bounds, as well as the number of times to generate
	println!("enter lower bound (inclusive):");
	let mut lb = String::new();
	io::stdin().read_line(&mut lb).expect("FAILURE");
	println!("enter upper bound (exclusive):");
	let mut ub = String::new();
	io::stdin().read_line(&mut ub).expect("FAILURE");
	println!("enter number of numbers to generate:");
	let mut nc = String::new();
	io::stdin().read_line(&mut nc).expect("FAILURE");
	// converts strings to i32s
	let lb: i32 = match lb.trim().parse() {
		Ok(num) => num,
		Err(_) => {return},
	};
	let ub: i32 = match ub.trim().parse() {
		Ok(num) => num,
		Err(_) => {return},
	};
	let nc: i32 = match nc.trim().parse() {
		Ok(num) => num,
		Err(_) => {return},
	};
	// does readback of info
	println!("printing {} number(s) between {} and {}", nc, lb, ub);
	// generates numbers
	let mut i = 0;
	loop {
		if i >= nc {
			print!("\n");
			break;
		}
		print!("{}", randrange(lb, ub));
		i += 1;
		if i < nc {
			print!(", ");
		}
	}
}

// neural network interface
fn neural_net() {
	println!("\x1b[38;2;0;255;0mneural network interface\x1b[39m");
}

// lists program ids
fn listpids() {
	println!("\x1b[38;2;0;255;0mlisting program ids\x1b[39m");
	// holds ids
	let list = ["0: guessing game (rng)", "1: guessing game (word)", "2: random number generator"];
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
			"0" => guess_num(),
			"1" => guess_word(),
			"2" => num_gen(),
			"3" => neural_net(),
			_ => println!("unrecognized id"),
		}
	}
}
