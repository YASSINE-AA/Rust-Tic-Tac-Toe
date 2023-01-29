use std::collections::BTreeMap;

pub fn winner(positionTable: BTreeMap<i32, String>, symbol: String) -> bool{
	let mut result: BTreeMap<i32, String> = BTreeMap::new();
	let mut counter = 0;
	let mut x = 1;
	let mut checkLine = false;
	for item in positionTable.clone() {
		if item.1 == symbol {
			result.insert(item.0, item.1);
		}
	}

	let mut temp = -1;
	// Checking horizontal lines.
	for item in result.clone() {
		if item.0 % 3 == 0{
			temp = item.0;
		}
		if temp >= 0 {
			if item.0 - temp == 1 {
				temp = item.0;
				counter += 1;
			}

			if counter == 2 {
				return true;
			}
		}
		
	}
	let lineStart = [0, 1, 2];
	counter = 0;
	temp = -1;
	// Checking vertical lines.
	for item in result.clone() {
		if lineStart.contains(&item.0) {
			temp = item.0;
		}
		if temp >=0 {
			if item.0 - temp == 3 {
				counter += 1;
				temp = item.0;
			}
		}

		if counter == 2 {
			return true;
		}
	}
	let diagLineStart = [0, 2];
	counter = 0;
	temp = -1;
	// Checking Diagonal Lines
	for item in result.clone() {
		if diagLineStart.contains(&item.0) {
			temp = item.0;
		}
		if temp >=0 {
			if (item.0 - temp) % 2 == 0 && item.0 != temp {
				counter += 1;
				temp = item.0;
			}
		}
		if counter == 2 {
			return true;
		}
	}

	return false;
}