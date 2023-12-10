use super::TicTacToe;
use super::Cell;




pub fn minmax(game: TicTacToe, depth: u32, max_player: bool, player_to_max: Cell) -> i32 {
    if depth == 0 || game.ended().is_some() {
        return game.heuristic(player_to_max);
    }

    let mut val = 0;

    if max_player {
        val = i32::MIN;
        let empty_spaces = game.empty_space();
        for cell in empty_spaces {
            let mut child = game.clone();
            child.play_move(player_to_max, cell);

            val = val.max(minmax(child, depth-1, false, player_to_max))
        }
    } else {
        val = i32::MAX;
        let empty_spaces = game.empty_space();

        for cell in empty_spaces {
            let mut child = game.clone();
            child.play_move(player_to_max.other_player(), cell);
            
            val = val.min(minmax(child, depth-1, true, player_to_max))
        }
        
    }

    return val;
}

