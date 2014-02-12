use std::int;

fn main() {
	let mut sum = 0;
	let limit = 6 * int::pow(9, 5);
	for i in range(2, limit) {
		if find_sum_of_raised_digits(i, 5) == i {
			sum += i;
		}
	}
	println("The sum of all the numbers that can be written as the sum of fifth powers of their digits is " + sum.to_str() + "!");
}

fn find_sum_of_raised_digits(mut number: int, exponent: uint) -> int {
	let mut sum = 0;
	let mut digits: ~[int] = ~[];
	while number > 0 {
		digits.push(number % 10);
		number /= 10;
	}
	for digit in digits.mut_iter() {
		sum += int::pow(*digit, exponent);
	}
	sum
}