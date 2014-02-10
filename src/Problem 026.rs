use std::num;

fn main() {
	let mut value_containing_longest_repetend = 0;
	let mut longest_repetend_length = 0;

	for d in range(2, 1000) {
		match generate_recurring_decimal(d) {
			(x, true) => {
				let repetend_length = match find_repetend(&x, d).len() {
					0 => fail!("Could not find repetend!"),
					x => x
				};
				if repetend_length > longest_repetend_length {
					longest_repetend_length = repetend_length;
					value_containing_longest_repetend = d;
				}
			}
			(_, false) => {}
		}
	}

	println(value_containing_longest_repetend.to_str());
}


/* Must provide positive integer greater than 1.
   Calculates decimal using long division.
   Returns tuple containing:
   - str: Decimal number of length (denominator*2 + 3*(1 + log(denominator))), a decimal of this length
   should include the repetend twice (if exists) 
   - bool: True if decimal is recurring, False if it terminates */
fn generate_recurring_decimal(denominator: int) -> (~str, bool) {
	let mut decimal: ~str = ~"0.";
	let mut remainder: int = 10;
	let mut is_recurring = true;
	let starting_zeros = 1 + num::log10(denominator as f32) as int;
	for i in range(0, starting_zeros) {
		decimal = decimal + "0";
		remainder *= 10;
	}
	for i in range(0, (denominator * 2) + (starting_zeros*2)) {
		let result = (remainder / denominator) as int;
		decimal = decimal + result.to_str();
		remainder = (remainder - (result * denominator)) * 10;

		if remainder == 0 {
			is_recurring = false;
			break;
		}
	}
	return (decimal, is_recurring);
}

// Denominator param is used to help ensure that function starts checking on a digit within the repetend
fn find_repetend(decimal: &~str, denominator: int) -> ~str {
	let mut start = 2 + (1 + num::log10(denominator as f32) as uint) * 3;
	let mut left_index = start;
	let mut right_index = start + 1;

	for i in range(left_index, decimal.len()) {

		// Possible start of repetend found, move both indices forwards to check all digits of repetend are correct
		if (decimal.char_at(right_index) == decimal.char_at(left_index)) {
			while (decimal.char_at(right_index + 1) == decimal.char_at(left_index + 1)) {
				right_index += 1;
				left_index += 1;

				// Repetend verified, move indices backwards before returning it to find correct start of repetend
				if (decimal.char_at(right_index) == decimal.char_at(start)) && (right_index - left_index == left_index - start) {
					while decimal.char_at(start - 1) == decimal.char_at(left_index - 1) {
						start -= 1;
						left_index -= 1;
					}
					return decimal.slice(start, left_index).to_owned();
				}
			}
			// Invalid repetend, reset indices whilst retaining same seperation
			right_index = start + (right_index - left_index);
			left_index = start;
		}
		right_index += 1;
	}
	// Return string of length 0 if function fails to find repetend
	return ~"";
}