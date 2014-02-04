fn main() {
	let starting_number = ~"2";

	println(sum_digits_in_string(&double_string_digits(&starting_number, 1000)).to_str());
}

// Compute the value of 2 raised to the power 1000 without BigInt, using strings and ints!
fn double_string_digits(original_string: &~str, amount_of_times: uint) -> ~str {
	let mut result_string: ~str = original_string.to_owned();
	let mut index: uint;
	let mut carry: int;
	let mut digits: ~[int];

	for i in range(1, amount_of_times) {
		index = result_string.len();
		digits = ~[];
		for j in range(0, result_string.len()) {
			digits.push(match from_str::<int>(result_string.slice(index - 1, index)) {
				Some(x) => x,
				None => fail!("Conversion failed!")
			});
			index -= 1;
		}

		result_string = ~"";
		carry = 0;
		for digit in digits.iter() {
			result_string = (((digit * 2) + carry) % 10).to_str() + result_string;
			carry  = ((digit * 2) + carry) / 10;
		}
		if carry != 0 {
			result_string = carry.to_str() + result_string;
		}
	}
	result_string
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