fn main() {
	let mut digits = ~[0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

	// Start from 1 as 0123456789 counts as the first lexicographic permutation
	for i in range(1, 1000000) {
		next_lexicographic_permutation(&mut digits);
	}

	println!("The millionth lexicographic permutation is {:?}!", digits);
}

fn next_lexicographic_permutation(vec: &mut ~[int]) {
	let size = vec.len() - 1;

	for i in range(0, size).invert() { // invert() has been changed to rev() in the Rust Master branch (Post ver.0.9)
		let distance_from_end = size - i;

		for j in range(0, distance_from_end) {
			let right_index = size - j;

			if vec[i] < vec[right_index] {
				vec.swap(i, right_index);

				if distance_from_end > 1 {
					vec.mut_slice(i + 1, size + 1).sort_by(|a, b| a.cmp(b));
				}
				return;
			}
		}
	}
}