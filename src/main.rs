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
use std::mem;
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

// gets integer input for the neural network interface
fn read_net_num(prompt: String) -> u32 {
	println!("\n");
	loop {
		// deletes old lines
		print!("\x1b[2K\x1b[1A\x1b[2K\x1b[1A");
		// prints prompt
		println!("{}", prompt);

		// reads input
		let mut inp = String::new();

		io::stdin().read_line(&mut inp)
			.expect("Failed to read line");

		// checks that the input is valid
		let inp: u32 = match inp.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};
		// returns the input
		return inp;
	}
}

// neural network interface
fn neural_net() {
	println!("\x1b[38;2;0;255;0mneural network interface\x1b[39m");
	// node data
	let mut nodes : Vec<Vec<(u8,usize)>> = Vec::new();
	// connection data
	let mut connections : Vec<(usize,usize,usize,usize,f32)> = Vec::new();
	// watched nodes
	let mut watched : Vec<(usize, usize)> = Vec::new();
	// main loop
	loop {
		// gets command
		println!("enter command");
		let mut com = String::new();
		io::stdin().read_line(&mut com).expect("FAILURE");
		let com = com.trim();
		// exits
		if com == "exit" {
			break;
		// runs the network
		} else if com == "run" {
			let mut net = Net::new();
			for layer in &nodes {
				net.add(layer.to_vec());
			}
			let con = &connections;
			net.connect(con.to_vec());
			net.run();
			net.print();
			for watch in &watched {
				println!("Node: ({}, {}), value: {}", watch.0, watch.1, net.layers[watch.0][watch.1].value);
			}
		// displays help on the network builder
		} else if com == "help" {
			let nbh = ["help about help:\n\nhelp gives help with using the neural network builder\n\nsee help (1) for a list of help pages", "help pages:\n\n\u{2022} run - 2\n\u{2022} nodes - 3\n\u{2022} connections 4\n\u{2022} watch - 5", "help about run:\n\nrun will run the network doing the following:\n\n\u{2022} assemble the network using given nodes and the connections between them\n\u{2022} calls print on the network, printing the layers\n\u{2022} prints the values of all watched nodes (see help 5)", "help about nodes:\n\nopens the dialog to edit network nodes\n\n\u{2022} clear - clears all nodes\n\u{2022} list - lists the nodes\n\u{2022} new - creates a new node\n\u{2022} edit - edits the node type of a node\n\u{2022} remove - removes a node", "help about connections:\n\nopens the dialog to edit network connections\n\n\u{2022} clear - clears all connections\n\u{2022} list - lists the connections\n\u{2022} new - creates a new connection\n\u{2022} edit - edits the weight of a connection\n\u{2022} remove - removes a connection", "help about watch:\n\nopens the dialog to edit watched network nodes\n\n\u{2022} clear - clears the list of all watched nodes\n\u{2022} list - lists the watched nodes\n\u{2022} new - watches a node\n\u{2022} remove - stops watching a node"];
			let n = read_net_num(String::from("enter number from 0 to 5:"));
			println!("\x1b[38;2;255;255;0mNeural Network Builder Help\x1b[39m\n\n{}", nbh[n as usize]);
		// edits the networks nodes
		} else if com == "nodes" {
			loop {
				// gets action
				println!("enter action");
				let mut ac = String::new();
				io::stdin().read_line(&mut ac).expect("FAILURE");
				let ac = ac.trim();
				// exits node editor
				if ac == "exit" {
					break;
				// clears all nodes
				} else if ac == "clear" {
					nodes = Vec::new();
				// lists nodes
				} else if ac == "list" {
					let mut i : usize = 0;
					loop {
						if i >= nodes.len() {
							break;
						}
						let mut i2 : usize = 0;
						loop {
							if i2 >= nodes[i].len() {
								break;
							}
							println!("({}, {}) ", nodes[i][i2].0, nodes[i][i2].1);
							i2 += 1;
						}
						i += 1;
					}
				// creates a new node
				} else if ac == "new" {
					let inp = read_net_num(String::from("enter node type:"));
					let mut nt: u8 = 0u8;
					let layer = read_net_num(String::from("enter node layer:")) as usize;
					unsafe {
						let y = mem::transmute::<u32, [u8; 4]>(inp);
						nt = y[0];
					}
					while nodes.len() <= layer {
						nodes.push(Vec::new());
					}
					nodes[layer].push((nt, layer));
				} else if ac == "edit" {
					// checks that there are nodes to edit
					if nodes.len() == 0usize {
						continue;
					}
					let mut esc = true;
					for l in &nodes {
						if l.len() > 0usize {
							esc = false;
							break;
						}
					}
					if esc {
						continue;
					}
					// prints nodes
					let mut i : usize = 0;
					loop {
						if i >= nodes.len() {
							break;
						}
						println!("{}", i);
						let mut i2 : usize = 0;
						loop {
							if i2 >= nodes[i].len() {
								break;
							}
							println!("({}, {}) ", nodes[i][i2].0, nodes[i][i2].1);
							i2 += 1;
						}
						i += 1;
					}
					let mut layer : usize = 0;
					let mut index : usize = 0;
					let mut nodetype : u8 = 0;
					// gets valid layer
					loop {
						layer = read_net_num(String::from("enter node layer:")) as usize;
						if layer >= nodes.len() {
							print!("\x1b[2K\x1b[1A\x1b[2K\x1b[1A\x1b[2K");
							continue;
						}
						break;
					}
					if nodes[layer].len() == 0usize {
						continue;
					}
					i = 0;
					loop {
						if i >= nodes[layer].len() {
							break;
						}
						println!("({}, {}) {}", nodes[layer][i].0, nodes[layer][i].1, i);
						i += 1;
					}
					// gets valid position
					loop {
						index = read_net_num(String::from("enter node position:")) as usize;
						if index >= nodes[layer].len() {
							print!("\x1b[2K\x1b[1A\x1b[2K\x1b[1A\x1b[2K");
							continue;
						}
						break;
					}
					// gets valid node type
					loop {
						nodetype = u8::try_from(read_net_num(String::from("enter node type:"))).unwrap();
						if nodetype > 2 {
							print!("\x1b[2K\x1b[1A\x1b[2K\x1b[1A\x1b[2K");
							continue;
						}
						break;
					}
					nodes[layer][index] = (nodetype, nodes[layer][index].1);
				// removes a node
				} else if ac == "remove" {
					// checks that there are nodes to remove
					if nodes.len() == 0usize {
						continue;
					}
					let mut esc = true;
					for l in &nodes {
						if l.len() > 0usize {
							esc = false;
							break;
						}
					}
					if esc {
						continue;
					}
					// prints nodes
					let mut i : usize = 0;
					loop {
						if i >= nodes.len() {
							break;
						}
						println!("{}", i);
						let mut i2 : usize = 0;
						loop {
							if i2 >= nodes[i].len() {
								break;
							}
							println!("({}, {}) ", nodes[i][i2].0, nodes[i][i2].1);
							i2 += 1;
						}
						i += 1;
					}
					let mut layer : usize = 0;
					let mut index : usize = 0;
					let mut nn : Vec<(u8, usize)> = Vec::new();
					let mut nc : Vec<(usize, usize, usize, usize, f32)> = Vec::new();
					// gets valid layer
					loop {
						layer = read_net_num(String::from("enter node layer:")) as usize;
						if layer >= nodes.len() {
							print!("\x1b[2K\x1b[1A\x1b[2K\x1b[1A\x1b[2K");
							continue;
						}
						break;
					}
					if nodes[layer].len() == 0usize {
						continue;
					}
					i = 0;
					loop {
						if i >= nodes[layer].len() {
							break;
						}
						println!("({}, {}) {}", nodes[layer][i].0, nodes[layer][i].1, i);
						i += 1;
					}
					// gets valid position
					loop {
						index = read_net_num(String::from("enter node position:")) as usize;
						if index >= nodes[layer].len() {
							print!("\x1b[2K\x1b[1A\x1b[2K\x1b[1A\x1b[2K");
							continue;
						}
						break;
					}
					// removes invalid connections
					i = 0;
					loop {
						if i >= connections.len() {
							break;
						}
						if (connections[i].0 == index && connections[i].1 == layer) || (connections[i].2 == index && connections[i].3 == layer) {
							i += 1;
							continue;
						}
						nc.push(connections[i]);
						i += 1;
					}
					// redoes the nodes
					i = 0;
					loop {
						if i >= nodes[layer].len() {
							break;
						}
						if i != index {
							nn.push(nodes[layer][i]);
						}
						i += 1;
					}
					nodes[layer] = nn;
				}
			}
		// edits the connections between network nodes
		} else if com == "connections" {
			loop {
				println!("enter action");
				let mut ac = String::new();
				io::stdin().read_line(&mut ac).expect("FAILURE");
				let ac = ac.trim();
				if ac == "exit" {
					break;
				} else if ac == "clear" {
					connections = Vec::new();
				} else if ac == "list" {
					let mut i = 0;
					loop {
						if i >= connections.len() {
							break;
						}
						println!("({}, {}, {}, {}, {}) ", connections[i].0, connections[i].1, connections[i].2, connections[i].3, connections[i].4);
						i += 1;
					}
				} else if ac == "new" {
					// checks that a valid connection is possible
					let mut poplayers = 0;
					for layer in &nodes {
						if layer.len() > 0usize {
							poplayers += 1;
						}
					}
					if poplayers < 2 {
						continue;
					}
					let mut x1 : usize = 0;
					let mut y1 : usize = 0;
					let mut x2 : usize = 0;
					let mut y2 : usize = 0;
					loop {
						y1 = read_net_num(String::from("enter node 1 layer:")) as usize;
						if y1 >= nodes.len() {
							print!("\x1b[2K\x1b[1A\x1b[2K\x1b[1A");
							continue;
						}
						break;
					}
					loop {
						x1 = read_net_num(String::from("enter node 1 position:")) as usize;
						if x1 >= nodes[y1].len() {
							print!("\x1b[2K\x1b[1A\x1b[2K\x1b[1A");
							continue;
						}
						break;
					}
					loop {
						y2 = read_net_num(String::from("enter node 2 layer:")) as usize;
						if y2 >= nodes.len() {
							print!("\x1b[2K\x1b[1A\x1b[2K\x1b[1A");
							continue;
						}
						break;
					}
					loop {
						x2 = read_net_num(String::from("enter node 2 position:")) as usize;
						if x2 >= nodes[y2].len() {
							print!("\x1b[2K\x1b[1A\x1b[2K\x1b[1A");
							continue;
						}
						break;
					}
					let mut w : f32 = 0f32;
					println!("\n");
					loop {
						print!("\x1b[2K\x1b[1A\x1b[2K\x1b[1A");
						println!("enter connection weight:");
						let mut inp = String::new();
						io::stdin().read_line(&mut inp).expect("Failed to read line");
						let inp: f32 = match inp.trim().parse() {
							Ok(num) => num,
							Err(_) => continue,
						};
						w = inp;
						break;
					}
					connections.push((x1, y1, x2, y2, w));
				} else if ac == "edit" {
					// checks that there are nodes to edit
					if connections.len() == 0usize {
						continue;
					}
					// prints connections
					let mut i : usize = 0;
					loop {
						if i >= connections.len() {
							break;
						}
						println!("({}, {}, {}, {}, {}) {}", connections[i].0, connections[i].1, connections[i].2, connections[i].3, connections[i].4, i);
						i += 1;
					}
					let mut index : usize = 0;
					let mut w : f32 = 0f32;
					// gets valid position
					loop {
						index = read_net_num(String::from("enter connection index:")) as usize;
						if index >= connections.len() {
							print!("\x1b[2K\x1b[1A\x1b[2K\x1b[1A\x1b[2K");
							continue;
						}
						break;
					}
					// gets valid node weight
					println!("\n");
					loop {
						print!("\x1b[2K\x1b[1A\x1b[2K\x1b[1A");
						println!("enter connection weight:");
						let mut inp = String::new();
						io::stdin().read_line(&mut inp).expect("Failed to read line");
						let inp: f32 = match inp.trim().parse() {
							Ok(num) => num,
							Err(_) => continue,
						};
						w = inp;
						break;
					}
					connections[index] = (connections[index].0, connections[index].1, connections[index].2, connections[index].3, w);
				// removes a node
				} else if ac == "remove" {
					// checks that there are nodes to remove
					if connections.len() == 0usize {
						continue;
					}
					// prints nodes
					let mut i : usize = 0;
					loop {
						if i >= connections.len() {
							break;
						}
						println!("({}, {}, {}, {}, {}) {}", connections[i].0, connections[i].1, connections[i].2, connections[i].3, connections[i].4, i);
						i += 1;
					}
					let mut index : usize = 0;
					let mut nc : Vec<(usize, usize, usize, usize, f32)> = Vec::new();
					// gets valid index
					loop {
						index = read_net_num(String::from("enter connection index:")) as usize;
						if index >= connections.len() {
							print!("\x1b[2K\x1b[1A\x1b[2K\x1b[1A\x1b[2K");
							continue;
						}
						break;
					}
					// redoes the connections
					i = 0;
					loop {
						if i >= connections.len() {
							break;
						}
						if i != index {
							nc.push(connections[i]);
						}
						i += 1;
					}
					connections = nc;
				}
			}
		// edits the list of watched nodes
		} else if com == "watch" {
			loop {
				println!("enter action");
				let mut ac = String::new();
				io::stdin().read_line(&mut ac).expect("FAILURE");
				let ac = ac.trim();
				if ac == "exit" {
					break;
				} else if ac == "clear" {
					watched = Vec::new();
				} else if ac == "list" {
					for w in &watched {
						println!("({}, {})", w.0, w.1);
					}
				} else if ac == "new" {
					let mut esc = true;
					for l in &nodes {	
						if l.len() > 0usize {
							esc = false;
							break;
						}
					}
					if esc {
						continue;
					}
					let mut i : usize = 0;
					loop {
						if i >= nodes.len() {
							break;
						}
						println!("{}", i);
						i += 1;
					}
					let mut y : usize = 0;
					let mut x : usize = 0;
					loop {
						y = read_net_num(String::from("enter node layer")) as usize;
						if y >= nodes.len() {
							print!("\x1b[2K\x1b[1A\x1b[2K\x1b[1A");
							continue;
						}
						if nodes[y].len() == 0usize {
							print!("\x1b[2K\x1b[1A\x1b[2K\x1b[1A");
							continue;
						}
						break;
					}
					i = 0;
					loop {
						if i >= nodes[y].len() {
							break;
						}
						println!("({}, {}), {}", nodes[y][i].0, nodes[y][i].1, i);
					}
					loop {
						x = read_net_num(String::from("enter node position")) as usize;
						if x >= nodes[y].len() {
							print!("\x1b[2K\x1b[1A\x1b[2K\x1b[1A");
							continue;
						}
						break;
					}
					for w in &watched {
						if w.0 == y && w.1 == x {
							esc = true;
						}
					}
					if esc {
						continue;
					}
					watched.push((y, x));
				} else if ac == "remove" {
					// checks that there are nodes to remove
					if watched.len() == 0usize {
						continue;
					}
					// prints nodes
					let mut i : usize = 0;
					loop {
						if i >= watched.len() {
							break;
						}
						println!("({}, {}) {}", watched[i].0, watched[i].1, i);
						i += 1;
					}
					let mut index : usize = 0;
					let mut nw : Vec<(usize, usize)> = Vec::new();
					// gets valid index
					loop {
						index = read_net_num(String::from("enter watch index:")) as usize;
						if index >= watched.len() {
							print!("\x1b[2K\x1b[1A\x1b[2K\x1b[1A\x1b[2K");
							continue;
						}
						break;
					}
					// redoes the watched
					i = 0;
					loop {
						if i >= watched.len() {
							break;
						}
						if i != index {
							nw.push(watched[i]);
						}
						i += 1;
					}
					watched = nw;
				}
			}
		}
	}
}

// lists program ids
fn listpids() {
	println!("\x1b[38;2;0;255;0mlisting program ids\x1b[39m");
	// holds ids
	let list = ["0: guessing game (rng)", "1: guessing game (word)", "2: random number generator", "3: neural network builder"];
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
