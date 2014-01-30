use std::num::sqrt;

fn main() {
	let mut factors: ~[u64] = ~[];
	let max: u64 = 600851475143;
	//600851475143
	for i in range(1, max) {
		if (i >= sqrt(max as f64) as u64) {
			factors.push(max / i);	
			break;
		}
		if max % i == 0 {
			factors.push(i);
			factors.push(max / i);
		}
	}

	let mut prime_factors: ~[u64] = ~[];

	'factor_loop: 
	for i in factors.iter() {
		for num in range(2, i-1) {
			if i % num == 0 {
				continue 'factor_loop;
			}
		}
		prime_factors.push(*i);
	}

	println!("{:?}", factors);
	println!("{:?}", prime_factors);
	println!("Largest prime factor is {:d}", prime_factors[prime_factors.len()-1] as int);
}