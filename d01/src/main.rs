mod input;

fn main() {
	let inputs = input::get_input();
	println!("{}", second(inputs));
}

fn _first(inputs: [i32; 2000]) -> i32 {
	let mut previous = i32::min_value();
	let mut count = -1;

	for num in inputs.iter() {
		if num > &previous {
			count += 1;
		}

		previous = *num;
	}

	return count;
}

fn second(inputs: [i32; 2000]) -> i32 {
	let mut count = 0;
	for i in 0..inputs.len() - 3 {
		let window_a = inputs[i] + inputs[i + 1] + inputs[i + 2];
		let window_b = window_a - inputs[i] + inputs[i + 3];
		if window_b > window_a {
			count += 1;
		}
	}

	return count;
}
