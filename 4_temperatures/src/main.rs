use std::io;

fn fahrenheit_to_celsius(temp: f32) -> f32 {
	(temp - 32.0) * 5.0/9.0
}

fn celsius_to_fahrenheit(temp: f32) -> f32 {
	temp * 9.0/5.0 + 32.0
}

fn main() {
	println!("Temperature converter!");

	loop {
		println!("Input the temperature.");

		let mut buffer = String::new();

		io::stdin()
			.read_line(&mut buffer)
			.expect("Failed to read line");

		let temp: f32 = match buffer.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};

		println!("Convert to\n1-Celsius\n2-Fahrenheit");

		let mut option = String::new();

		io::stdin()
			.read_line(&mut option)
			.expect("Failed to read line");

		let result: f32;
		match option.trim() {
			"1" => {
					result = fahrenheit_to_celsius(temp);
					println!("{temp}ºF to Celsius: {result}ºC");
				},
			"2" => {
				result = celsius_to_fahrenheit(temp);
				println!("{temp}ºC to Fahrenheit: {result}ºF")
			},
			_ => println!("Invalid option")
		};
	}
}
