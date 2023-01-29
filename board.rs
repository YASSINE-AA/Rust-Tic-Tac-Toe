use std::collections::BTreeMap;

mod winner;

pub fn gameBoard(positionTable: BTreeMap<i32, String>) {

	// Draw the game Board with the given positionTable
	println!(" {:?} | {:?} | {:?}", positionTable[&0], positionTable[&1], positionTable[&2]);
	println!("-----------------");
	println!(" {:?} | {:?} | {:?}",  positionTable[&3], positionTable[&4], positionTable[&5]);
	println!("-----------------");
	println!(" {:?} | {:?} | {:?}",  positionTable[&6], positionTable[&7], positionTable[&8]);
	if winner::winner(positionTable.clone(), "X".to_string()) {
		println!("X Won!");
		print!("Game Over!");
		std::process::exit(0);
	}else if winner::winner(positionTable.clone(), "O".to_string()) {
		println!("O Won!");
		print!("Game Over!");
		std::process::exit(0);
	} else {
		println!("Nobody Won! Game Over!");
	}
}