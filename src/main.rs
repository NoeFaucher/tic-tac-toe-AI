use std::io;
use game::{TicTacToe, Cell};

mod game;
mod test;

fn main() {

    let mut game = TicTacToe::new();

    println!("You are the first player X:");
    println!();

    while !game.ended().is_some() {
        game.show();

        let coord_to_play = get_player_move();

        // Check if the chosen cell is empty before making the move
        if game.play_move(Cell::X, coord_to_play) {

            game.show();
            // Check if the game is finished after the move
            if game.ended().is_some() {
                break;
            }

            // Simulate opponent's move (you can replace this with actual opponent logic)
            println!("Opponent to play (O):");
            let opponent_move = game.get_best_move(Cell::O).unwrap();
            game.play_move(Cell::O, opponent_move);
        } else {
            println!("Invalid move. The selected cell is already occupied. Try again.");
        }

    }

    match game.ended().unwrap() {
        game::EndStatus::Draw => println!("That's a draw !"),
        game::EndStatus::Winner(Cell::O) => println!("You lose unfortunately!"),
        game::EndStatus::Winner(Cell::X) => println!("You win congratulation !"),
        _ => (),
    }
}

fn get_player_move() -> (usize, usize) {
    loop {
        println!("Enter your move (column and row, separated by a space):");
        println!("Top left is 0 0 and top right is 2 0 for example.");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let coordinates: Vec<usize> = input
            .trim()
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if coordinates.len() == 2 {
            return (coordinates[0], coordinates[1]);
        } else {
            println!("Invalid input. Please enter two numbers separated by a space.");
        }
    }
}