use std::iter;

fn main() {
	let coins: ~[int] = ~[1, 2, 5, 10, 20, 50, 100, 200];
	let target: int = 200;

	println("The number of different ways to make £2 using any number of coins is " + number_of_ways(coins.len() - 1, 0, &coins, target).to_str() + "!");
}

fn number_of_ways(index: uint, collected_value: int, coins: &~[int], target: int) -> int {
	if collected_value == target {
		1
	} else if collected_value > target {
		0
	} else if index == 0 {
		let mut ways: int = 0;
		for value in iter::count(0, coins[0]).take_while(|&value| value <= target - collected_value) {
			if collected_value + value == target {
				ways += 1;
			}
		}
		ways
	} else {
		let mut ways: int = 0;
		for value in iter::count(0, coins[index]).take_while(|&value| value <= target - collected_value) {
			ways += number_of_ways(index - 1, collected_value + value, coins, target);
		}
		ways
	}
}