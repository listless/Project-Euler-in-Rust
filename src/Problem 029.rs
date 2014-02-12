use std::hashmap::HashSet;
use std::iter::range_inclusive;
use std::iter::count;

fn main() {

	let mut distinct_terms: int = 0;
	let mut distinct_prime_factors: HashSet<~[int]> = HashSet::new();

	for a in range_inclusive(2, 100) {
		for b in range_inclusive(2, 100) {
			if check_if_prime(&a) {
				distinct_terms += 1;
			} else {
				let mut power_prime_factors: ~[int] = ~[];
				for i in range(0, b) {
					power_prime_factors = std::vec::append(power_prime_factors, prime_factors(a));
				}
				if power_prime_factors.len() <= 100 && check_same_elements(&power_prime_factors) {
					continue;
				} else {
					power_prime_factors.sort_by(|a, b| a.cmp(b));
					if !distinct_prime_factors.contains(&power_prime_factors) {
						distinct_prime_factors.insert(power_prime_factors);
						distinct_terms += 1;
					}
				}
			}
		}
	}
	println("The number of distinct terms are " + distinct_terms.to_str() + "!");
}

fn prime_factors(mut num: int) -> ~[int] {
	let mut prime_factors: ~[int] = ~[];

	while num % 2 == 0 {
		prime_factors.push(2);
		num /= 2;
	}
	for i in count(3, 2).take_while(|&i| i*i <= num) {
		while num % i == 0 {
			prime_factors.push(i);
			num /= i;
		}
	}
	if num > 2 {
		prime_factors.push(num);
	}
	prime_factors
}

fn check_if_prime(num: &int) -> bool {
	if *num < 2 {
		false
	} else if *num == 2  {
		true
	} else if *num % 2 == 0 {
		false
	} else {
		for i in count(3, 2).take_while(|&i| i*i <= *num) {
			if *num % i == 0 {
				return false;
			}
		} 
		true
	}
}

fn check_same_elements(vec: &~[int]) -> bool {
	for i in range(1, vec.len()) {
		if vec[i-1] != vec[i] {
			return false;
		}
	}
	true
}