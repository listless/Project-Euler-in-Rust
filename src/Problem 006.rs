fn main() {
	let mut natural_seq: ~[int] = ~[0, ..100];

	for i in range(0, natural_seq.len() as int) {
		natural_seq[i] = i + 1;
	}

	let sum_of_sq: int = sum_of_square(&natural_seq);
	let sq_of_sum: int = square_of_sum(&natural_seq);
	let diff_of_sums: int = sq_of_sum - sum_of_sq;

	println!("{:?}", diff_of_sums);
}

fn sum_of_square(natural_seq: &~[int]) -> int {
	let mut num: int = 0;

	for i in range(0, natural_seq.len() as int) {
		num += natural_seq[i] * natural_seq[i];
	}

	num
}

fn square_of_sum(natural_seq: &~[int]) -> int {
	let mut num: int = 0;

	for i in range(0, natural_seq.len() as int) {
		num += natural_seq[i];
	}

	num * num
}