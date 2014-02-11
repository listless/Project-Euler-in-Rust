use std::iter::count;

fn main() {
	let max: i64 = 600851475143;
	let mut prime_factors: ~[i64] = prime_factors(max);

	println!("Largest prime factor is {:d}", prime_factors[prime_factors.len()-1] as int);
}

fn prime_factors(mut num: i64) -> ~[i64] {
	let mut prime_factors: ~[i64] = ~[];

	while num % 2 == 0 {
		prime_factors.push(2);
		num /= 2;
	}
	for i in count(3 as i64, 2 as i64).take_while(|&i| i*i <= num) {
		while num % i == 0 {
			prime_factors.push(i);
			num /= i;
		}
	}
	if num > 2 {
		prime_factors.push(num);
	}
	prime_factors
}