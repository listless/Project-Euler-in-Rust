fn main() {
	let starting_number = 100;

	println(sum_digits_in_string(&factorial_as_string(starting_number)).to_str());
}

// Finding factorial of number without BigInt, using only strings and ints
fn factorial_as_string(number: int) -> ~str {
	let mut factorial: ~str = number.to_str();
	let mut index: uint;
	let mut carry: int;
	let mut digits: ~[int];

	for i in range(1, number) {
		index = factorial.len();
		digits = ~[];
		for j in range(0, factorial.len()) {
			digits.push(match from_str::<int>(factorial.slice(index - 1, index)) {
				Some(x) => x,
				None => fail!("Conversion failed!")
			});
			index -= 1;
		}

		factorial = ~"";
		carry = 0;
		for digit in digits.iter() {
			factorial = (((digit * (number - (i - 1))) + carry) % 10).to_str() + factorial;
			carry  = ((digit * (number - (i - 1))) + carry) / 10;
		}
		if carry != 0 {
			factorial = carry.to_str() + factorial;
		}
	}
	factorial
}

fn sum_digits_in_string(string: &~str) -> int {
	let mut sum: int = 0;
	for i in range(0, string.len()) {
		sum += match from_str::<int>(string.slice(i, i + 1)) {
			Some(x) => x,
			None => fail!("Conversion failed!")
		};
	}
	sum
}