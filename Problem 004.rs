use std::iter;

fn main() {
	let mut highest_palindrome: int = 0;
	let mut first_num: int = 0;
	let mut second_num: int = 0;

	for i in iter::range_step(1000, 100, -1) {
		for j in iter::range_step(1000, 100, -1) {
			if check_if_palindromic(i*j) {
				if i*j > highest_palindrome {
					highest_palindrome = i*j;
					first_num = i;
					second_num = j;
				}
			}
		}
	}
	println!("{:d} is the highest palindromic number from the product of two 3-digit numbers!", highest_palindrome);
	println!("The two digits are: {:d} and {:d}!", first_num, second_num);
}



fn check_if_palindromic(i: int) -> bool {
	let mut temp: int = i;
	let mut vec: ~[int] = ~[];

	while temp > 0 {
		vec.push(temp % 10);
		temp /= 10;
	}

	let half_len = vec.len() / 2;

	let start_seq = vec.mut_slice_to(half_len).to_owned();
	let mut end_seq   = 
	if vec.len() % 2 == 0 {
		vec.mut_slice_from(half_len).to_owned()
	} else {
		vec.mut_slice_from(half_len + 1).to_owned()
	};

	end_seq.reverse();

	for i in range(0, half_len) {
		if end_seq[i] != start_seq[i] {
			return false;
		}
	}
	
	true
}