fn main() {
	/* The limit can be found by checking n * 9!, where the value of n should be equal to the number of digits of the value.
	 * We know the limit is 7 * 9! because 8 * 9! gives us 2903040, a value with less than 8 digits,
	 * whereas 7 * 9! gives us 2540160, a number with exactly 7 digits.
	 */
	let limit = 7 * factorial_of_digit(9);

	let mut result = 0;
	for i in range(3, limit) {
		if sum_of_factorials_of_digits(i) == i {
			result += i;
		}
	}
	println!("The sum of all numbers which are equal to the sum of the factorial of their digits is {:d}!", result);
}

fn sum_of_factorials_of_digits(mut number: int) -> int {
	let mut sum = 0;
	while number > 0 {
		sum += factorial_of_digit(number % 10);
		number /= 10;
	}
	sum
}

fn factorial_of_digit(number: int) -> int {
	match number {
		0 => 1,
		1 => 1,
		2 => 2,
		3 => 6,
		4 => 24,
		5 => 120,
		6 => 720,
		7 => 5040,
		8 => 40320,
		9 => 362880,
		_ => fail!("A digit can only be a value from 0 till 9!")
	}
}