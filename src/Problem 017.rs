fn main() {
	let mut sum: int = 0;

	for i in range(1, 1001) {
		let digits = int_to_digits(i);
		// Teens and single numbers
		match digits {
			[.., 1, 1] | [.., 1, 2]     => sum += 6,
			[.., 1, 5] | [.., 1, 6]     => sum += 7,
			[.., 1, 3] | [.., 1, 4] | [.., 1, 8] | [.., 1, 9] => sum += 8,
			[.., 1, 7]  => sum += 9,
			[.., 1] | [.., 2] | [.., 6] => sum += 3,
			[.., 4] | [.., 5] | [.., 9] => sum += 4,
			[.., 3] | [.., 7] | [.., 8] => sum += 5,
			_ => {}
		}
		// The tens
		match digits {
			[.., 1, 0] => sum += 3,
			[.., 4, _] | [.., 5, _] | [.., 6, _] => sum += 5, // Watch out for spelling 'forty' as 'fourty'!
			[.., 2, _] | [.., 3, _] | [.., 8, _] | [.., 9, _] => sum += 6,
			[.., 7, _] => sum += 7,
			_ => {}
		}
		// The hundreds and 'and'
		match digits {
			[.., 1, _, _] | [.., 2, _, _] | [.., 6, _, _] => sum += 13,
			[.., 4, _, _] | [.., 5, _, _] | [.., 9, _, _] => sum += 14,
			[.., 3, _, _] | [.., 7, _, _] | [.., 8, _, _] => sum += 15,
			_ => {}
		}
		// minus the 'and' from the hundreds if number is exactly equal to a multiple of 100 (and not 1000)
		if (i % 100 == 0) && (i != 1000) {
			sum -= 3;
		}
		if i == 1000 {
			sum += 11;
		}
	}

	println(sum.to_str());
}

fn int_to_digits(num: int) -> ~[int] {
	let mut digits: ~[int] = ~[];
	let mut temp_num = num;

	while temp_num > 0 {
		digits.push(temp_num % 10);
		temp_num /= 10;
	}
	digits.reverse();
	digits
}