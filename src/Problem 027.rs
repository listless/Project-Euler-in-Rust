use std::iter::count;

fn main() {
	let mut highest_num_of_primes = 0;
	let mut highest_a = 0;
	let mut highest_b = 0;

	for a in range(-999, 1000) {
		for b in range(-999, 1000) {
			let num_of_primes = num_of_primes(a, b);
			if highest_num_of_primes < num_of_primes {
				highest_num_of_primes = num_of_primes;
				highest_a = a;
				highest_b = b;
			}
		}
	}

	println!("Product of the coefficents, a and b, for the quadratic expression that produces the max num of primes is... {:d}!", highest_a*highest_b);
}

fn num_of_primes(a: int, b: int) -> int {
	let mut num_of_primes = 0;
	let mut result = b;
	let mut n = 1;
	while check_if_prime(&result) {
		num_of_primes += 1;
		result = (n*n) + (a*n) + b;
		n += 1;
	}

	num_of_primes
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