fn main() {
	let size_of_grid = 1001;
	let mut spiral = create_grid(size_of_grid);
	fill_spiral(&mut spiral);

	println("The sum of the numbers on the diagonals in a " + size_of_grid.to_str() 
		+ " by " + size_of_grid.to_str() + " spiral is " + calculate_sum_of_diagonals(&spiral).to_str() + "!");
}

// Provide an odd number so that the fill_sprial function will be able to start from a middle
fn create_grid(size: int) -> ~[~[int]] {
	let mut grid: ~[~[int]] = ~[];
	
	for i in range(0, size) {
		let mut width: ~[int] = ~[];
		for j in range(0, size) {
			width.push(0);
		}
		grid.push(width);
	}
	return grid;
}

fn fill_spiral(spiral: &mut ~[~[int]]) {
	let mut x_index = spiral.len() / 2;
	let mut y_index = spiral.len() / 2;
	let mut value = 1;
	spiral[y_index][x_index] = value;

	let mut distance_from_center = 1;

	for i in range(0, (spiral.len() / 2)) {
		x_index += 1;
		value += 1;
		spiral[y_index][x_index] = value;
		for j in range(0, distance_from_center + (distance_from_center - 1)) {
			y_index += 1;
			value += 1;
			spiral[y_index][x_index] = value;
		}
		for j in range(0, distance_from_center * 2) {
			x_index -= 1;
			value += 1;
			spiral[y_index][x_index] = value;
		}
		for j in range(0, distance_from_center * 2) {
			y_index -= 1;
			value += 1;
			spiral[y_index][x_index] = value;
		}
		for j in range(0, distance_from_center * 2) {
			x_index += 1;
			value += 1;
			spiral[y_index][x_index] = value;
		}
		distance_from_center += 1;
	}
}

fn calculate_sum_of_diagonals(spiral: & ~[~[int]]) -> int {
	let mut x_index = 0;
	let mut y_index = 0;

	// Start with -1 as the for loops will pass through 1 twice
	let mut sum = -1;

	for i in range(0, spiral.len()) {
		sum += spiral[y_index][x_index];
		x_index += 1;
		y_index += 1;
	}
	x_index = 0;
	y_index = spiral.len() - 1;
	for i in range(0, spiral.len()) {
		sum += spiral[y_index][x_index];
		x_index += 1;
		y_index -= 1;
	}

	sum
}