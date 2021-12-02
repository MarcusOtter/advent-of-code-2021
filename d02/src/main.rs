use std::fs::read_to_string;

fn main() {
	let input =
		read_to_string("input.txt")
		.expect("Something went wrong reading the input file");

	let commands = input
		.lines()
		.collect::<Vec<&str>>();

	first(commands);
}

fn first(commands: Vec<&str>) {
	let mut position_x = 0;
	let mut depth = 0;

	for x in commands {
		let mut split = x.split_whitespace();
		let command = split.next().unwrap();
		let number = split.next().unwrap().parse::<i32>().unwrap();
		
		if      command == "down"    { depth += number; }
		else if command == "up"      { depth -= number; }
		else if command == "forward" { position_x += number; }
	}

	println!("Horizontal position: {}", position_x);
	println!("Depth: {}", depth);
	println!("Multiplied: {}", position_x * depth);
}
