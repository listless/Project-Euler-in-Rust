fn main() {
	let mut num: int = 1;

	'main_loop:
	loop {
		for i in range(1, 21) {
			if num % i != 0 {
				num += 1;
				continue 'main_loop;
			}
		}
		break;
	}
	println!("The smallest positive number that is evenly divided by all nums from 1-20 is {:d}!", num);
}