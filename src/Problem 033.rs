struct Fraction {
	numerator: int,
	denominator: int
}

impl Fraction {
	fn new(numerator: int, denominator: int) -> Fraction {
		Fraction { numerator: numerator, denominator: denominator }
	}

	fn simplify(&mut self) -> Fraction {
		let gcd = self.gcd();
		self.numerator = self.numerator / gcd;
		self.denominator = self.denominator / gcd;
		*self
	}

	fn value(&self) -> f64 {
		self.numerator as f64 / self.denominator as f64
	}

	fn gcd(&self) -> int {
		greatest_common_denominator(self.numerator, self.denominator)
	}
}

fn main() {
	let mut curious_fractions: ~[Fraction] = ~[];
	for i in range(11, 90) {
		for j in range(i+1, 100) {
			/* To find our curious fractions, check if:
			 * - The 2nd digit of Numerator = 1st digit of Denominator
			 * - The 2nd digit of Denominator is not 0 (else we may divide by 0)
			 * - The possible curious fraction can be correctly simpified by the fraction created when removing numerator and denominator digits
			 */
			if (i % 10 == j / 10) && (j % 10 != 0) && ((i as f32 / j as f32) % ((i / 10) as f32 / (j % 10) as f32) == 0.0) {
				let temp_fraction = Fraction::new(i, j);
				if !check_if_duplicate_fraction(&curious_fractions, temp_fraction) {
					curious_fractions.push(temp_fraction);
				}
			}
		}
	}

	let product_fraction = multiply_fractions(&curious_fractions).simplify();
	println!("The value of the denominator of the product of the 4 curious fractions in its lowest common terms is {:d}!", product_fraction.denominator);
}

fn multiply_fractions(fractions: &~[Fraction]) -> Fraction {
	let mut numerator = fractions[0].numerator;
	let mut denominator = fractions[0].denominator;
	for i in range(1, fractions.len()) {
		numerator *= fractions[i].numerator;
		denominator *= fractions[i].denominator;
	}
	Fraction::new(numerator, denominator)
}

fn check_if_duplicate_fraction(fractions: &~[Fraction], target: Fraction) -> bool {
	let mut values: ~[f64] = ~[];
	values.push(target.value());
	for fraction in fractions.iter() {
		values.push(fraction.value());
	}
	merge_sort(&mut values);

	for i in range(1, values.len()) {
		if values[i-1] == values[i] {
			return true;
		}
	}
	false
}

fn greatest_common_denominator(a: int, b: int) -> int {
	if b == 0 {
		return a;
	} else {
		return greatest_common_denominator(b, a % b);
	}
}

fn merge_sort(floats: &mut ~[f64]) {
	if floats.len() <= 1 {
		return;
	}

	let middle = floats.len() / 2;
	let mut left = floats.slice_to(middle).to_owned();
	let mut right = floats.slice_from(middle).to_owned();

	merge_sort(&mut left);
	merge_sort(&mut right);

	let mut l = 0;
	let mut r = 0;
	while (l + r < floats.len()) {
		if (l < left.len() && r < right.len()) {
			if (left[l] < right[r]) {
				floats[l + r] = left[l].clone();
				l += 1;
			} else {
				floats[l + r] = right[r].clone();
				r += 1;
			}
		} else if (l < left.len()) {
			floats[l + r] = left[l].clone();
			l += 1;
		} else {
			floats[l + r] = right[r].clone();
			r += 1;
		}
	}
}