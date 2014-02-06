use std::path::Path;
use std::io::fs::File;
use std::io::buffered::BufferedReader;
use std::char::to_digit;

fn main() {
	let path : Path   = Path::new("../lib/names.txt");
	let on_error      = || fail!("open of {:?} failed", path);
	let reader : File = File::open(&path).unwrap_or_else(on_error);

	let mut reader = BufferedReader::new(reader);

	let input = reader.read_to_str().replace("\"", "");
	let mut names: ~[&str] = input.split(',').collect();
	merge_sort(&mut names);

	let mut score: int;
	let mut total_score: int = 0;
	for i in range(0, names.len()) {
		score = 0;
		for character in names[i].chars() {
			score += match to_digit(character, 36) {
				Some(x) => x as int - 9, // -9 to get alphabetical value
				None => { 
					println("Conversion failed!");
					0
				}
			}
		}
		score *= i as int + 1;
		total_score += score;
	}

	println(total_score.to_str());
}


// Merge sort learnt from Computerphile and adapted for Rust 
fn merge_sort(string_vec: &mut ~[&str]) {
	if string_vec.len() <= 1 {
		return;
	}

	let middle = string_vec.len() / 2;
	let mut left = string_vec.slice_to(middle).to_owned();
	let mut right = string_vec.slice_from(middle).to_owned();

	merge_sort(&mut left);
	merge_sort(&mut right);

	let mut l = 0;
	let mut r = 0;
	while l + r < string_vec.len() {
		if l < left.len() && r < right.len() {
			if left[l] < right[r] {
				string_vec[l+r] = left[l].clone();
				l += 1;
			} else {
				string_vec[l+r] = right[r].clone();
				r += 1;
			}
		} else if l < left.len() {
			string_vec[l+r] = left[l].clone();
			l += 1;
		} else {
			string_vec[l+r] = right[r].clone();
			r += 1;
		}
	}
}