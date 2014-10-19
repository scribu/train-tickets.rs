extern crate train_tickets;
use train_tickets::Coach;

#[test]
fn cant_buy_more_seats_than_the_coach_has() {
	let coach = Coach::new(3, 2);

	assert_eq!(None, coach.find_seats(7));
}

#[test]
fn can_fill_more_than_first_compartment() {
	let coach = Coach::new(3, 4);

	assert_eq!(Some(vec![1, 2, 3, 4, 5]), coach.find_seats(5));
}

#[test]
fn groups_seats_in_a_single_compartment() {
	let mut coach = Coach::new(3, 4);

	assert!(coach.occupy_seats([1, 2, 3]).is_ok());

	assert_eq!(Some(vec![5, 6, 7, 8]), coach.find_seats(4));
}

#[test]
fn group_seats_in_as_few_compartments_as_possible() {
	let mut coach = Coach::new(3, 4);

	assert!(coach.occupy_seats([1, 2, 3, 5, 6, 7]).is_ok());

	assert_eq!(Some(vec![4, 9, 10, 11, 12]), coach.find_seats(5));
}
