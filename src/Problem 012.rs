fn main() {
	let mut number: i64 = 1;

	while number_of_factors(triangle_number(number)) < 500 {
		number += 1;
	}

	println!("The first triangle number to have over 500 divisors is {:d} and it is the {:d}th triangle number!", triangle_number(number), number);

}

fn triangle_number(num: i64) -> i64 {
	let mut triangle_number: i64 = 0;
	for i in range(0, num + 1) {
		triangle_number += i;
	}
	triangle_number
}

fn number_of_factors(num: i64) -> i64 {
	let mut number_of_factors: i64 = 0;
	let mut i: i64 = 1;
	while (i * i <= num) {
		if num % i == 0 {
			number_of_factors += 2;
			if (i * i == num) {
				number_of_factors -= 1;
			}
		}
		i += 1;
	}
	number_of_factors
}