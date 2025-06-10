use std::io;

fn fibonacci(n: u32) -> u32 {
	if n <= 1 {
		return n;
	}

	fibonacci(n - 1) + fibonacci(n - 2)
}

fn main() {
	println!("Please type a number: ");

	let mut buffer = String::new();

	io::stdin()
		.read_line(&mut buffer)
		.expect("Failed to read line");

	let num: u32 = match buffer.trim().parse() {
		Ok(num) => num,
		Err(_) => 0,
	};

	let res = fibonacci(num);
	println!("The Fib of {num} is {res}");
}
