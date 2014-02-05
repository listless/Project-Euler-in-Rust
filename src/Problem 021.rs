use std::iter::count;
use std::hashmap::HashMap;

fn main() {
	// num_spd_pair stores a number with its sum of proper divisors (spd)
	let mut num_spd_pairs: HashMap<int, int> = HashMap::new();
	let mut amicable_pairs: ~[int] = ~[];

	for i in range(1, 10000) {
		num_spd_pairs.insert(i, sum_of_proper_divisors(i));
	}

	for i in range(1, 10000) {
		// If hashmap contains the key equal to the spd of i...
		if num_spd_pairs.contains_key(num_spd_pairs.get(&i)) {
			// ... and if i is also equal to the spd of that key...
			// ... and if i's spd is NOT equal to itself (because cannot have an amicable pair with itself!)...
			// ... then it's an amicable pair!
			if (i == *num_spd_pairs.get(num_spd_pairs.get(&i))) && (i != *num_spd_pairs.get(&i)) {
				amicable_pairs.push(i);
			}
		}
	}

	println!("{:?}", amicable_pairs);

	let mut sum_of_amicable_pairs: int = 0;
	for pair in amicable_pairs.iter() {
		sum_of_amicable_pairs += *pair;
		print(pair.to_str() + " ");
	}

	println("\n" + sum_of_amicable_pairs.to_str());
}

// Proper divisors: Numbers less than n which divide evenly into n
fn sum_of_proper_divisors(number: int) -> int {
	if number == 1 {
		0
	} else {
		let mut sum: int = - number;

		for i in count(1, 1).take_while(|&i| i * i < number) {
			if number % i == 0 {
				sum += i;
				sum += number / i;

				if i * i == number {
					sum -= i;
				}
			}
		}
		sum
	}
}