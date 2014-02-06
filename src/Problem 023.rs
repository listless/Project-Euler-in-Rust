use std::iter::count;

fn main() {
	let mut abundant_numbers: ~[int] = ~[];
	let mut not_sum_of_two_abundants: ~[bool] = ~[];
	for i in range(0, 28123) {
		not_sum_of_two_abundants.push(true);
		if sum_of_proper_divisors(i) > i {
			abundant_numbers.push(i);
		}
	}

	for i in range(0, abundant_numbers.len()) {
		for j in range(0, abundant_numbers.len()) {
			if abundant_numbers[i] + abundant_numbers[j] < 28123 {
				not_sum_of_two_abundants[abundant_numbers[i]+abundant_numbers[j]] = false;
			}
		}
	}

	let mut result_sum = 0;
	for i in range(0, not_sum_of_two_abundants.len()) {
		if not_sum_of_two_abundants[i] == true {
			result_sum += i;
		}
	}

	println(result_sum.to_str());
}

fn sum_of_proper_divisors(number: int) -> int {
	if number == 1 {
		0
	} else {
		let mut sum: int = -number;

		for i in count(1, 1).take_while(|&i| i * i <= number) {
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