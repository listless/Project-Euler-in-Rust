use std::iter::count;

fn main() {
	// let mut spd_of_index: ~[int] = ~[];
	// for i in range(0, 28123) {
	// 	spd_of_index.push(sum_of_proper_divisors(i));
	// }

	let mut abundant_numbers: ~[int] = ~[];
	let mut not_sum_of_abundants: ~[bool] = ~[];
	for i in range(0, 28123) {
		not_sum_of_abundants.push(true);
		if sum_of_proper_divisors(i) > i {
			abundant_numbers.push(i);
		}
	}

	// for i in range(0, 100) {
	// 	print(spd_of_index[i].to_str() + " ");
	// }

	// for i in range(0, abundant_numbers.len()) {
	// 	println(abundant_numbers[i].to_str());
	// }

	let mut sums_of_two_abundant_numbers: ~[int] = ~[];
	for i in range(0, abundant_numbers.len()) {
		for j in range(0, abundant_numbers.len()) {
			// sums_of_two_abundant_numbers.push(abundant_numbers[i] + abundant_numbers[j]);
			not_sum_of_abundants[i+j] = false;
		}
	}

	// sums_of_two_abundant_numbers.sort_by(|a, b| a.cmp(b));

	// for i in range(0, 100) {
	// 	println(sums_of_two_abundant_numbers[i].to_str());
	// }

	let mut result_sum = 0;

	for i in range(0, not_sum_of_abundants.len()) {
		if not_sum_of_abundants[i] == true {
			result_sum += i;
		}
	}
	// let mut counter: uint;
	// 'index_loop:
	// for i in range(1, 28123) {
	// 	counter = 0;
	// 	for j in range(0, sums_of_two_abundant_numbers.len()) {
	// 		if sums_of_two_abundant_numbers[j] == i {
	// 			continue 'index_loop;
	// 		}
	// 		if sums_of_two_abundant_numbers[j] > i {
	// 			counter += 1;
	// 			j += abundant_numbers.len() - 1;
	// 		}
	// 		if counter > abundant_numbers.len() {
	// 			break;
	// 		}

	// 	}
	// 	// println(i.to_str());
	// 	result_sum += i;
	// }

	println(result_sum.to_str());
}

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