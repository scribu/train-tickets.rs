type SeatNr = uint;

#[deriving(Show)]
struct Coach {
	comp_size: uint,
	seats: Vec<bool>
}

impl Coach {
	fn new(num_comp: uint, comp_size: uint) -> Coach {
		let seats = Vec::from_elem(num_comp*comp_size, false);

		Coach {comp_size: comp_size, seats: seats}
	}

	fn get_empty_seats(&self) -> Vec<SeatNr> {
		let mut empty_seats = Vec::new();

		let counter = std::iter::count(1u, 1);

		for (seat_nr, occupied) in counter.zip(self.seats.iter()) {
			if *occupied == false {
				empty_seats.push(seat_nr)
			}
		}

		empty_seats
	}

	fn occupy_seats(&mut self, seats_to_mark: &[SeatNr]) -> Result<(), Vec<SeatNr>> {
		let mut occupied_seats = Vec::new();

		for &seat_nr in seats_to_mark.iter() {
			// needs https://github.com/rust-lang/rust/pull/17934
			// let seat = self.seats[seat_nr-1]
			let seat = self.seats.get_mut(seat_nr-1);

			if *seat == true {
				occupied_seats.push(seat_nr)
			} else {
				*seat = true;
			}
		}

		if occupied_seats.len() == 0 {
			Ok(())
		} else {
			Err(occupied_seats)
		}
	}
}

fn main() {
	let mut coach = Coach::new(3, 4);

	match coach.occupy_seats([1u, 2u, 5u]) {
		Ok(_) => {}
		Err(occupied_seats) => println!("Seats {} were already occupied.", occupied_seats)
	}

	println!("{}", coach.get_empty_seats());
}
