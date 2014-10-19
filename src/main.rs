extern crate train_tickets;
use train_tickets::Coach;

use std::io;

#[cfg(not(test))]
fn main() {
	let mut coach = Coach::new(5, 5);

	loop {
		println!("Empty seats: {}", coach.get_empty_seats());

		print!("Number of seats to buy: ");

		let input = io::stdin().read_line().ok().expect("Failed to read line");

		let seats_to_find: uint = match from_str(input.as_slice().trim()) {
			Some(num) => num,
			None => {
				println!("Please input a number!");
				continue;
			}
		};

		match coach.find_seats(seats_to_find) {
			Some(found_seats) => {
				coach.occupy_seats(found_seats.as_slice())
					.ok().expect("Tried to occupy already occupied seats");
				println!("Bought seats: {}", found_seats);
			}
			None => println!("Couldn't find {} seats.", seats_to_find)
		}
	}
}
