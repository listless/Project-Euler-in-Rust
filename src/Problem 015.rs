use std::hashmap::HashMap;

fn main() {
	let mut cache: HashMap<(int, int), i64> = HashMap::new();
	let x_length = 20;
	let y_length = 20;

	println!("{:d}", routes_in_grid(x_length, y_length, 0, 0, &mut cache));
}

fn routes_in_grid(max_x: int, max_y: int, x: int, y: int, cache: &mut HashMap<(int, int), i64>) -> i64 {
	let mut number_of_routes: i64 = 0;

	if x < max_x {
		if cache.contains_key(&(x, y)) || cache.contains_key(&(y, x)) {
			number_of_routes = *cache.get(&(x, y));
			return number_of_routes;
		} else {
			number_of_routes += routes_in_grid(max_x, max_y, x + 1, y, cache);
		}
	}

	if y < max_y {
		if cache.contains_key(&(x, y)) || cache.contains_key(&(y, x)) {
			number_of_routes = *cache.get(&(x, y));
			return number_of_routes;
		} else {
			number_of_routes += routes_in_grid(max_x, max_y, x, y + 1, cache);
		}
	}

	if x == max_x && y == max_y {
		1
	} else {
		cache.insert((x, y), number_of_routes);
		cache.insert((y, x), number_of_routes);
		number_of_routes
	}
}