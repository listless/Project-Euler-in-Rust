use std::num::sqrt;
use std::iter::range_step;

fn main() {
	let mut sum: u64 = 0;

	for i in range(2, 2000000) {
		if is_prime(i) {
			sum += i as u64;
		}
	}

	println(sum.to_str());
}

fn is_prime(n: int) -> bool {
	if n == 2 {
		return true;
	}
	if n % 2 == 0 {
		return false;
	}
	for i in range_step(3, ((sqrt(n as f32).ceil() as int) + 1), 2) {
		if n % i == 0 {
			return false;
		}
	}
	true
}