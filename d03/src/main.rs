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
	// I could've probably done this with some bitwise magic but 2s complement logic does not work
	// well when the amount of bits in an integer are not divisible by 8 and/or unknown
	let mut gamma_rate_str = "".to_owned();
	let mut epsilon_rate_str = "".to_owned();
	for i in 0..numbers[0].chars().count() {
		if is_majority('1', numbers, i) {
			gamma_rate_str.push_str("1");
			epsilon_rate_str.push_str("0");
		}
		else {
			gamma_rate_str.push_str("0");
			epsilon_rate_str.push_str("1");
		}
	}

	let gamma_rate   = i32::from_str_radix(&gamma_rate_str, 2).unwrap();
	let epsilon_rate = i32::from_str_radix(&epsilon_rate_str, 2).unwrap();

	println!("Gamma: {}\nEpsilon: {}", gamma_rate, epsilon_rate);
	println!("Power consumption: {}", gamma_rate * epsilon_rate);
}

// Dear lord this is ugly and confusing, sorry!
// Implemented .filter() manually because I couldn't figure out how it worked lol
fn second(numbers: &Vec<&str>) {
	let mut oxygen_rating = numbers.clone();
	let mut co2_rating = numbers.clone();
	
	for i in 0..numbers[0].chars().count() {
		let oxygen_majority_ones = is_majority('1', &oxygen_rating, i);
		let co2_majority_ones    = is_majority('1', &co2_rating, i);

		for num in numbers {
			let character = num.as_bytes()[i] as char;
			if oxygen_rating.len() == 1 && co2_rating.len() == 1 { break; }

			if oxygen_rating.len() > 1 && (character == '0' && oxygen_majority_ones || character == '1' && !oxygen_majority_ones) {
				let index = oxygen_rating.iter().position(|x| x == num);
				if index.is_some() {
					oxygen_rating.remove(index.unwrap());

				}
			}
			
			if co2_rating.len() > 1 && (character == '1' && co2_majority_ones || character == '0' && !co2_majority_ones) {
				let index = co2_rating.iter().position(|x| x == num);
				if index.is_some() {
					co2_rating.remove(index.unwrap());
				}
			}
		}
	}

	let oxygen = i32::from_str_radix(oxygen_rating[0], 2).unwrap();
	let co2    = i32::from_str_radix(co2_rating[0],    2).unwrap();

	println!("Oxygen generator rating: {}", oxygen);
	println!("CO2 scrubber rating: {}", co2);
	println!("Life support rating: {}", oxygen * co2);
}

fn is_majority(character: char, numbers: &Vec<&str>, index: usize) -> bool {
	let mut char_count = 0;
	let mut other_count = 0;

	for num in numbers {
		if num.as_bytes()[index] as char == character {
			char_count += 1;
		}
		else {
			other_count += 1;
		}
	}

	return char_count >= other_count;
}
