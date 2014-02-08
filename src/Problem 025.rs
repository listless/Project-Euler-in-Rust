use std::hashmap::HashMap;

fn main() {
	let mut cache: HashMap<int, ~str> = HashMap::new();
	let mut term = 0;
	let mut result = ~"";
	while result.len() < 1000 {
		term += 1;
		result = fibonacci(term, &mut cache);
	}
	println!("The first term in the Fibonacci sequence to contain 1000 digits is {:d}!", term);
}

fn fibonacci(term: int, cache: &mut HashMap<int, ~str>) -> ~str {
	if term == 1 || term == 2 {
		~"1"
	} else if cache.contains_key(&term) {
		cache.get(&term).clone()
	} else {
		let fibonacci = add_strings(&fibonacci(term - 1, cache), &fibonacci(term - 2, cache));
		cache.insert(term, fibonacci.clone());
		fibonacci
	}
}

// Addition function for two strings of digits of different lengths!
fn add_strings(string_one: &~str, string_two: &~str) -> ~str{
	let mut first_str = string_one.to_owned();
	let mut second_str = string_two.to_owned();

	let mut index: uint = match first_str.len() > second_str.len() {
		true  => {
			for i in range(0, first_str.len() - second_str.len()) {
				second_str = "0" + second_str;
			}
			first_str.len()
		}
		false => {
			for i in range(0, second_str.len() - first_str.len()) {
				first_str = "0" + first_str;
			}
			second_str.len()
		}
	};

	let mut column_sum: int;
	let mut carry: int = 0;
	let mut result: ~str = ~"";

	for i in range(0, first_str.len()) {
		column_sum = 0;
		column_sum += match from_str::<int>(first_str.slice_chars(index - 1, index)) {
				Some(x) => x,
				None => fail!("Conversion failed!")
			};
		column_sum += match from_str::<int>(second_str.slice_chars(index - 1, index)) {
				Some(x) => x,
				None => fail!("Conversion failed!")
			};
		column_sum += carry;
		carry = column_sum / 10;
		result = (column_sum % 10).to_str() + result;
		index -= 1;
	}
	match carry {
		0 => return result,
		_ => return (carry.to_str() + result)
	}
}