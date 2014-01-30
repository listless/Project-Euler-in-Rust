use std::num;

fn main() {
	let mut a: int = 0;
	let mut b: int = 0;
	let mut c_squared: int;
	let mut c: f32 = 0.0;

	'top_loop:
	for i in range(1, 1000) {
		for j in range(1, 1000) {
			c_squared = (num::pow(i as f32, 2.0) as int) + (num::pow(j as f32, 2.0) as int);
			c = num::sqrt(c_squared as f32);
			if c % 1.0 == 0.0 {
				if i + j + (c as int) == 1000 {
					println!("We've found the Pythagorean triplet! a: {:d} b: {:d} c: {:d}", i, j, c as int);
					a = i;
					b = j;
					break 'top_loop;
				}
			}
		}
	}

	println!("Product abc = {:d}", a * b * (c as int));
}