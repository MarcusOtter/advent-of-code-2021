use std::fs::read_to_string;

fn main() {
	let input =
		read_to_string("example.txt")
		.expect("Something went wrong reading the input file");

	let commands = input
		.lines()
		.collect::<Vec<&str>>();

	println!("First:");
	first(commands);

	let commands = input
		.lines()
		.collect::<Vec<&str>>();

	println!("\nSecond:");
	second(commands);
}

fn first(commands: Vec<&str>) {
	let mut position_x = 0;
	let mut depth = 0;

	for x in commands {
		let mut split = x.split_whitespace();
		let command = split.next().unwrap();
		let number = split.next().unwrap().parse::<i32>().unwrap();
		
		match command {
			"down"    => depth += number,
			"up"      => depth -= number,
			"forward" => position_x += number,
			_ => unreachable!()
		}
	}

	println!("Horizontal position: {}", position_x);
	println!("Depth: {}", depth);
	println!("Multiplied: {}", position_x * depth);
}

fn second(commands: Vec<&str>) {
	let mut position_x = 0;
	let mut depth = 0;
	let mut aim = 0;

	for x in commands {
		let mut split = x.split_whitespace();
		let command = split.next().unwrap();
		let number = split.next().unwrap().parse::<i32>().unwrap();
		
		match command {
			"down"    => aim += number,
			"up"      => aim -= number,
			"forward" => {
				position_x += number;
				depth += aim * number;
			},
			_ => unreachable!()
		}
	}

	println!("Horizontal position: {}", position_x);
	println!("Depth: {}", depth);
	println!("Multiplied: {}", position_x * depth);
}