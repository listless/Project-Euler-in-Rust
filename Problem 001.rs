fn main() {
	let mut sum: int = 0;
	for i in range(1, 1000) {
		if i % 3 == 0 {
			sum += i;
		}
		else if i % 5 == 0 {
			sum += i;
		}
	}
	println!("{:d}", sum);
}