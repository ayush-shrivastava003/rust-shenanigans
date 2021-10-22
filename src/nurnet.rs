pub fn nodefnf () -> bool {
	return false;
}

pub fn nodefnt () -> bool {
	return true;
}

pub struct Net {
	pub layers : Vec<Vec<Node>>,
	layer_width : usize,
}

impl Net {
	pub fn new() -> Net {
		return Net{layers:Vec::new(), layer_width:0usize};
	}
	pub fn addnode(&mut self, node: Node, layer: usize) {
		while self.layers.len() <= layer {
			self.layers.push(Vec::new());
		}
		self.layers[layer].push(node);
		if self.layers[layer].len() > self.layer_width {
			self.layer_width = self.layers[layer].len();
		}
	}
	pub fn add(&mut self, data: Vec<(u8,usize)>) {
		let mut i = 0;
		loop {
			if i >= data.len() {
				break;
			}
			self.addnode(Node::new(data[i].0), data[i].1);
			i += 1;
		}
	}
	pub fn connectnode(&mut self, x1: usize, y1: usize, x2: usize, y2: usize, weight: f32) {
		self.layers[y2][x2].weights.push((x1, y1, weight));
	}
	pub fn connect(&mut self, data: Vec<(usize,usize,usize,usize,f32)>) {
		let mut i = 0;
		loop {
			if i >= data.len() {
				break;
			}
			self.connectnode(data[i].0, data[i].1, data[i].2, data[i].3, data[i].4);
			i += 1;
		}
	}
	pub fn run(&mut self) {
		let mut i1 = 0usize;
		loop {
			if i1 >= self.layers.len() {
				break;
			}
			let mut i2 = 0usize;
			loop {
				if i2 >= self.layers[i1].len() {
					break;
				}
				if self.layers[i1][i2].nodetype == 1 {
					self.layers[i1][i2].value = (self.layers[i1][i2].nodefn)();
				} else if self.layers[i1][i2].nodetype == 2 {
					self.layers[i1][i2].value = true;
				} else {
					let mut i3 = 0usize;
					let mut total = 0f32;
					loop {
						if i3 >= self.layers[i1][i2].weights.len() {
							break;
						}
						let weight = self.layers[i1][i2].weights[i3];
						let mut v = 0f32;
						if self.layers[weight.1][weight.0].value {
							v = 1f32;
						}
						total += weight.2*v;
						self.layers[i1][i2].weights[i3] = weight;
						i3 += 1;
					}
					if total >= 1f32 {
						self.layers[i1][i2].value = true;
					} else {
						self.layers[i1][i2].value = false;
					}
				}
				i2 += 1;
			}
			i1 += 1;
		}
	}
	pub fn print(&mut self) {
		let mut i1 = 0usize;
		loop {
			if i1 >= self.layers.len() {
				break;
			}
			let mut i3 = 0usize;
			loop {
				if i3 >= self.layer_width*5-1 {
					break;
				}
				print!("_");
				i3 += 1;
			}
			print!("\n");
			let mut i2 = 0usize;
			loop {
				if i2 >= self.layers[i1].len() {
					break;
				}
				print!("| {} ", self.layers[i1][i2].nodetype);
				i2 += 1;
			}
			print!("|\n");
			i1 += 1;
		}
		let mut i3 = 0usize;
		loop {
			if i3 >= self.layer_width*5-1 {
				break;
			}
			print!("_");
			i3 += 1;
		}
		print!("\n");
	}
}

pub struct Node {
	pub nodetype : u8,
	pub weights : Vec<(usize, usize, f32)>,
	pub nodefn : fn()->bool,
	pub value : bool,
}

impl Node {
	pub fn new(nt: u8) -> Node {
		return Node{nodetype:nt, weights:Vec::new(), nodefn:nodefnf, value:false};
	}
}