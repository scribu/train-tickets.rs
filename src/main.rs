type SeatNr = uint;

#[deriving(Show)]
struct Seat(SeatNr, bool);

#[deriving(Show)]
struct Coach {
	comp_size: uint,
	seats: Vec<Seat>
}

impl Coach {
	fn new(num_comp: uint, comp_size: uint) -> Coach {
		let seats = Vec::from_fn(num_comp*comp_size, |idx| Seat(idx+1, false)); 

		Coach {comp_size: comp_size, seats: seats}
	}

	fn empty_seats(&self) -> Vec<SeatNr> {
		let mut empty_seats = Vec::new();

		for seat in self.seats.iter() {
			match *seat {
				Seat(_, true) => (),
				Seat(seat_nr, false) => empty_seats.push(seat_nr)
			}
		}

		empty_seats
	}
}

fn main() {
	let coach = Coach::new(3, 4);

	println!("{}", coach.empty_seats());
}
