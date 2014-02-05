use std::iter::range_inclusive;

fn main() {
	
	let mut total_days: int = 0;
	let mut days_on_1st: ~[int] = ~[];

	for year in range_inclusive(1900, 2000) {
		for month in range_inclusive(1, 12) {
			match month {
				1  => {
					days_on_1st.push(total_days);
					total_days += 31;
				}
				2  => {
					days_on_1st.push(total_days);
					total_days += 29;
				}
				3  => {
					days_on_1st.push(total_days);
					total_days += 31;
				}
				4  => {
					days_on_1st.push(total_days);
					total_days += 30;
				}
				5  => {
					days_on_1st.push(total_days);
					total_days += 31;
				}
				6  => {
					days_on_1st.push(total_days);
					total_days += 30;
				}
				7  => {
					days_on_1st.push(total_days);
					total_days += 31;
				}
				8  => {
					days_on_1st.push(total_days);
					total_days += 31;
				}
				9  => {
					days_on_1st.push(total_days);
					total_days += 30;
				}
				10 => {
					days_on_1st.push(total_days);
					total_days += 31;
				}
				11 => {
					days_on_1st.push(total_days);
					total_days += 30;
				}
				_ => {
					days_on_1st.push(total_days);
					total_days += 31;
				}
			}
		}
		if year % 4 != 0 {
			total_days -= 1;
		}
	}
	// Subtract total_days in year 1900 from total total_days as we are looking from 1901 - 2000
	total_days -= 365;

	let mut sundays: ~[int] = ~[];
	for i in range(0, (total_days / 7)) {
		sundays.push(6 + (7 * i));
	}

	println!("total_days on first jan: {:?}", days_on_1st);
	println!("suntotal_days occur on: {:?}", sundays);

	let mut sundays_on_1st: int = 0;

	for day in days_on_1st.iter() {
		for sunday in sundays.iter() {
			if day == sunday {
				println(day.to_str() + " " + sunday.to_str());
				sundays_on_1st += 1;
			}
		}
	}

	println(sundays_on_1st.to_str());
}