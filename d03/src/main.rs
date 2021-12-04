use std::fs::read_to_string;

fn main() {
	let input =
		read_to_string("input.txt")
		.expect("Something went wrong reading the input file");

	let binary_numbers = input
		.lines()
		.collect::<Vec<&str>>();

	println!("-- First --");
	first(&binary_numbers);

	println!("\n-- Second --");
	second(&binary_numbers);
}

fn first(numbers: &Vec<&str>) {
	let bits = numbers[0].chars().count();

	let mut ones = vec![0; bits];
	let mut zeroes = vec![0; bits];

	// Count zeroes and ones in each position
	for num in numbers {
		for i in 0..bits {
			if num.as_bytes()[i] as char == '1' {
				ones[i] += 1;
			}
			else {
				zeroes[i] += 1;
			}
		}
	}

	// I could've probably done this with some bitwise magic but 2s complement logic does not work
	// well when the amount of bits in an integer are not divisible by 8 and/or unknown
	let mut gamma_rate_str = "".to_owned();
	let mut epsilon_rate_str = "".to_owned();
	for i in 0..ones.len() {
		if ones[i] > zeroes[i] {
			gamma_rate_str.push_str("1");
			epsilon_rate_str.push_str("0");
		}
		else {
			gamma_rate_str.push_str("0");
			epsilon_rate_str.push_str("1");
		}
	}

	let gamma_rate   = u64::from_str_radix(&gamma_rate_str, 2).unwrap();
	let epsilon_rate = u64::from_str_radix(&epsilon_rate_str, 2).unwrap();

	println!("Gamma: {}\nEpsilon: {}", gamma_rate, epsilon_rate);
	println!("Power consumption: {}", gamma_rate * epsilon_rate);
}

fn second(numbers: &Vec<&str>) {

}

