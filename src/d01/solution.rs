#[path = "input.rs"] mod input;

pub fn run() {
	let inputs = input::get_input();
	println!("{}", inputs.len());
}
