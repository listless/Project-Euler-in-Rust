use std::hashmap::HashMap;

fn main() {
	let mut cache: HashMap<(uint, uint), i64> = HashMap::new();
	let triangle: ~[~[int]] = ~[~[75],
	                            ~[95, 64],
	                            ~[17, 47, 82],
	                            ~[18, 35, 87, 10],
	                            ~[20, 04, 82, 47, 65],
	                            ~[19, 01, 23, 75, 03, 34],
	                            ~[88, 02, 77, 73, 07, 63, 67],
	                            ~[99, 65, 04, 28, 06, 16, 70, 92],
	                            ~[41, 41, 26, 56, 83, 40, 80, 70, 33],
	                            ~[41, 48, 72, 33, 47, 32, 37, 16, 94, 29],
	                            ~[53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14],
	                            ~[70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57],
	                            ~[91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48],
	                            ~[63, 66, 04, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31],
	                            ~[04, 62, 98, 27, 23, 09, 70, 98, 73, 93, 38, 53, 60, 04, 23]];

	println("The Maximum total from top to bottom of the triangle is... " + greatest_path_sum(triangle.len() - 1, 0, 0, &triangle, &mut cache).to_str() + " !!");
}

fn greatest_path_sum(height: uint, x: uint, y: uint, triangle: &~[~[int]], cache: &mut HashMap<(uint, uint), i64>) -> i64 {
	let mut i: i64 = 0;
	let mut j: i64 = 0;

	if cache.contains_key(&(x, y)) {
		return *cache.get(&(x, y));
	} else if y < height {
		i = triangle[y][x] as i64 + greatest_path_sum(height, x, y + 1, triangle, cache);
		j = triangle[y][x] as i64 + greatest_path_sum(height, x + 1, y + 1, triangle, cache);
	}

	if y == height {
		triangle[y][x] as i64
	} else if i > j {
		cache.insert((x, y), i);
		i
	} else {
		cache.insert((x, y), j);
		j
	}
}