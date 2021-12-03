use std::fs::read_to_string;

fn main() {
	let input =
		read_to_string("example.txt")
		.expect("Something went wrong reading the input file");

	let binary_numbers = input
		.lines()
		.collect::<Vec<&str>>();

	let bits = binary_numbers[0].chars().count();

	let mut ones = vec![0; bits];
	let mut zeroes = vec![0; bits];

	for num in binary_numbers {
		for i in 0..bits {
			if num.as_bytes()[i] as char == '1' {
				ones[i] += 1;
			}
			else {
				zeroes[i] += 1;
			}
		}
	}

	println!("{:?}", ones);
	println!("{:?}", zeroes);
}

