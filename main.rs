use std::collections::BTreeMap;
mod board;

fn userInput(chosenSymbol: &str, mut positionTable: BTreeMap<i32, String>) -> [String; 2]{
	let mut position = String::new();
	std::io::stdin().read_line(&mut position).unwrap();
	return [position, chosenSymbol.to_string()];
}

fn gameLogic() {
	let mut positionTable: BTreeMap<i32, String> = BTreeMap::new();
	// Insert default values
	for index in 0..=8 {
		positionTable.insert(index, " ".to_string());
	}
	// Generate Game Board
	board::gameBoard(positionTable.clone());
	let players:[i32; 2] = [1, 2];
	let symbols:[String; 2] = ["X".to_string(), "O".to_string()];
	let mut keepLoop = true;
	loop {
		for i in 0..=1 {
			println!("It's player {}'s turn ({})", players[i], symbols[i]);
			let userInfo = userInput(&symbols[i], positionTable.clone());
			if userInfo[0].trim().parse::<i32>().unwrap() == 9 {
				keepLoop = false;
				break;
			}
			positionTable.insert(userInfo[0].trim().parse::<i32>().unwrap(), userInfo[1].clone());
			board::gameBoard(positionTable.clone());
		}
		if keepLoop == false {
			break;
		}
	}
	
}
fn main() {
	// Main Menu 
	let mut userChoice = String::new();
	let arr:[i32; 2] = [1, 2]; // Store options in an array
	println!("Main Menu");
	println!("1 - Play ");
	println!("2 - Quit ");
	std::io::stdin().read_line(&mut userChoice).unwrap();
	let userChoice_int:i32 = userChoice.trim().parse::<i32>().unwrap();
	
	if arr.contains(&userChoice_int) {
		gameLogic(); // Calling Tic Tac Toe Board
	} 	

}