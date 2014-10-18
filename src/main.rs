use std::collections::HashSet;

type SeatNr = uint;

#[deriving(Show)]
struct Coach {
	comp_size: uint,
	num_comp: uint,
	occupied_seats: HashSet<SeatNr>,
}

impl Coach {
	fn new(num_comp: uint, comp_size: uint) -> Coach {
		Coach {
			comp_size: comp_size,
			num_comp: num_comp,
			occupied_seats: HashSet::new()
		}
	}

	fn get_empty_seats(&self) -> Vec<SeatNr> {
		let mut empty_seats = Vec::new();

		for seat_nr in range(1u, self.comp_size*self.num_comp) {
			if !self.occupied_seats.contains(&seat_nr) {
				empty_seats.push(seat_nr)
			}
		}

		empty_seats
	}

	fn occupy_seats(&mut self, seats_to_mark: &[SeatNr]) -> Result<(), Vec<SeatNr>> {
		let mut occupied_seats = Vec::new();

		for seat_nr in seats_to_mark.iter() {
			if self.occupied_seats.contains(seat_nr) {
				occupied_seats.push(*seat_nr);
			} else {
				self.occupied_seats.insert(*seat_nr);
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
