use std::int;
use std::num;
use std::hashmap;

fn main() {
	let mut sum: i64 = 0;
	let mut cache: hashmap::HashSet<int> = hashmap::HashSet::new();
	for i in range(0, 10000) {
		if check_if_unique_digits(i) {
			for j in range(0, 10000 / int::pow(10, num::log10(i as f32) as uint)) {
				if check_if_unique_digits(j) {
					let product = i * j;
					if check_if_pandigital(&mut append_numbers(product, i, j)) {
						if !cache.contains(&product) {
							sum += product as i64;
							cache.insert(product);
						}
					}
				}
			}
		}
	}
	println(sum.to_str());
}

fn check_if_unique_digits(mut number: int) -> bool {
	let mut digits: ~[int] = ~[];
	while number > 0 {
		digits.push(number % 10);
		number /= 10;
	}
	digits.sort_by(|a, b| a.cmp(b));

	for i in range(1, digits.len()) {
		if digits[i-1] == digits[i] {
			return false;
		}
	}
	true
}

fn check_if_pandigital(digits: &mut ~[int]) -> bool {
	if digits.len() != 9 {
		return false;
	}
	digits.sort_by(|a, b| a.cmp(b));

	for i in range(1, 10) {
		if digits[i - 1] != i {
			return false;
		}
	}
	true
}

// TODO: Make append_numbers variadic
fn append_numbers(mut first: int, mut second: int, mut third: int) -> ~[int] {
	let mut digits: ~[int] = ~[];
	while first > 0 {
		digits.push(first % 10);
		first /= 10;
	}
	while second > 0 {
		digits.push(second % 10);
		second /= 10;
	}
	while third > 0 {
		digits.push(third % 10);
		third /= 10;
	}
	digits
}