fn main() {
	let mut highest_starting_number: int = 0;
	let mut highest_chain: i64 = 0;
	let mut current_chain: i64;
	for i in range(1, 1000000) {
		current_chain = numbers_in_chain(i as i64);
		if highest_chain < current_chain {
			highest_chain = current_chain;
			highest_starting_number = i;
		}
	}
	println(highest_starting_number.to_str());
}

fn numbers_in_chain(num: i64) -> i64 { // Use 64 bit as 32 bit is too small (causes stack overflow if arg is 999167).
	if num == 1 { 
		1
	} else if num % 2 == 0 {
		1 + numbers_in_chain(num/2)
	} else {
		1 + numbers_in_chain(num*3 + 1)
	}
}