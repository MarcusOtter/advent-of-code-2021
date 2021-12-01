mod input;

fn main() {
    let inputs = input::get_input();
    let mut previous = i32::min_value();
    let mut count = -1;

    for num in inputs.iter() {
        if num > &previous {
            count += 1;
        }

        previous = *num;
    }

	println!("{}", count);
}
