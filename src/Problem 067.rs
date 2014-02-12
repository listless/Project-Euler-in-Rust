use std::path::Path;
use std::io::fs::File;
use std::io::buffered::BufferedReader;
use std::hashmap::HashMap;

fn main() {
	let path : Path   = Path::new("../lib/triangle.txt");
	let on_error      = || fail!("open of {:?} failed", path);
	let reader : File = File::open(&path).unwrap_or_else(on_error);

	let mut reader = BufferedReader::new(reader);

	let mut triangle: ~[~[int]] = ~[];
	let mut rows_loaded = 0;

	for line in reader.lines() {
		let mut temp_vec: ~[int] = ~[];
		
		for i in range (0, 100) {

			if i <= rows_loaded {
				match from_str::<int>(line.slice((i*3) as uint, ((i*3) + 2) as uint)) {
					Some(x) => temp_vec.push(x),
					None => fail!("Conversion failed!")
				};
			}
		}
		triangle.push(temp_vec);
		rows_loaded += 1;
	}
	// Print triangle in rows to check everything came out fine
	// for row in triangle.iter() {
	// 	println!("{:?}", row);
	// }

	let mut cache: HashMap<(uint, uint), i64> = HashMap::new();

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