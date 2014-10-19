use std::collections::SmallIntMap;
use std::cmp::min;

pub type SeatNr = uint;
pub type SeatList = Vec<SeatNr>;

#[deriving(Show)]
pub struct Coach {
	comp_size: uint,
	num_comp: uint,
	occupied_seats: SmallIntMap<()>,
}

impl Coach {
	pub fn new(num_comp: uint, comp_size: uint) -> Coach {
		Coach {
			comp_size: comp_size,
			num_comp: num_comp,
			occupied_seats: SmallIntMap::new()
		}
	}

	pub fn get_empty_seats(&self) -> SeatList {
		range(1u, self.comp_size * self.num_comp + 1)
			.filter(|seat_nr| !self.occupied_seats.contains_key(seat_nr))
			.collect::<SeatList>()
	}

	fn empty_seats_in_comp(&self, comp_nr: uint) -> SeatList {
		let seat_start = comp_nr * self.comp_size + 1;
		let comp_seats = range(seat_start, seat_start + self.comp_size);

		comp_seats
			.filter(|seat_nr| !self.occupied_seats.contains_key(seat_nr))
			.collect::<SeatList>()
	}

	fn remainder_empty_seats(&self, found_seats: &SmallIntMap<()>) -> SeatList {
		let all_empty_seats = self.get_empty_seats();
		let mut empty_seats: SeatList = Vec::new();

		for &seat_nr in all_empty_seats.iter() {
			if !found_seats.contains_key(&seat_nr) {
				empty_seats.push(seat_nr);
			}
		}

		empty_seats
	}

	pub fn find_seats(&self, count: uint) -> Option<SeatList> {
		let mut found_seats: SmallIntMap<()> = SmallIntMap::new();

		// first, try to allocate contiguous portions of compartments
		for comp_nr in range(0u, self.num_comp) {
			if found_seats.len() == count {
				break;
			}

			let empty_seats = self.empty_seats_in_comp(comp_nr);
			let empty_seats_count = empty_seats.len();

			let need_to_find = min(self.comp_size, count - found_seats.len());

			if empty_seats_count < need_to_find {
				continue
			}

			for &seat_nr in empty_seats.slice(0, need_to_find).iter() {
				found_seats.insert(seat_nr, ());
			}
		}

		// then, find empty seats wherever available
		for &seat_nr in self.remainder_empty_seats(&found_seats).iter()
			.take(count - found_seats.len()) {
			found_seats.insert(seat_nr, ());
		}

		if found_seats.len() < count {
			None
		} else {
			Some(found_seats.keys().collect::<SeatList>())
		}
	}

	pub fn occupy_seats(&mut self, seats_to_mark: &[SeatNr]) -> Result<(), SeatList> {
		let mut occupied_seats = Vec::new();

		for seat_nr in seats_to_mark.iter() {
			if self.occupied_seats.contains_key(seat_nr) {
				occupied_seats.push(*seat_nr);
			} else {
				self.occupied_seats.insert(*seat_nr, ());
			}
		}

		if occupied_seats.len() == 0 {
			Ok(())
		} else {
			Err(occupied_seats)
		}
	}
}
