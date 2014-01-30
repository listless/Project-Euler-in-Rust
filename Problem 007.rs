fn main() {
	let mut num: int = 1;
	let mut prime_index: int = 0;
	
	while prime_index < 10001 {
		num += 1;
		if check_if_prime(&num) {
			prime_index += 1;
		}
	}
	println!("{:?}", prime_index);
	println!("{:?}", num);
}

fn check_if_prime(num: &int) -> bool {
	if *num == 2  {
		true
	} else {
		for i in range(2, *num) {
			if *num % i == 0 {
				return false;
			}
		} 
		true
	}
}