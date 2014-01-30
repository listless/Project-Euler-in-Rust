fn main() {
	let mut first = 1;
	let mut second = 2;
	let mut sum = 0;
	for i in range(3, 4000000) {
		if (i == (first + second)) {
			if(i % 2 == 0) {
				sum += i;
			}
			first = second;
			second = i;
			println!("{:d}", i);
		}
	}
	println!("{:d}", sum);
}