pub mod ia;





#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Cell {
    Empty,
    X,
    O
}

impl Cell {
    pub fn other_player(&self) -> Cell {
        match self {
            Cell::Empty => Cell::Empty,
            Cell::X => Cell::O,
            Cell::O => Cell::X,
        }
    }
}    

#[derive(Debug)]
pub enum EndStatus {
    Draw,
    Winner(Cell)
}

#[derive(Clone, Copy)]
pub struct TicTacToe {
    board: [[Cell;3];3],
}


impl TicTacToe {
    pub fn new() -> Self {
        TicTacToe {
            board : [[Cell::Empty;3];3],
        }
    }

    /** Play a given move
     *  
     *  return true if the move was play else return false
     */
    pub fn play_move(&mut self, c :Cell, (x,y): (usize,usize)) -> bool {
        if x>2 || y>2 || self.board[x][y] != Cell::Empty || c == Cell::Empty {
            return false;
        }        

        self.board[x][y] = c;
        return true;
    }

    pub fn ended(&self) -> Option<EndStatus>{
        
        if self.is_winner(Cell::O) {
            return Some(EndStatus::Winner(Cell::O));
        }
        
        if self.is_winner(Cell::X) {
            return Some(EndStatus::Winner(Cell::X));
        }

        if self.is_full() {
            return Some(EndStatus::Draw);
        }

        return None;
    }

    fn is_winner(&self, player: Cell) -> bool {
        // Check rows
        for row in 0..3 {
            if (0..3).all(|col| self.board[row][col] == player) {
                return true;
            }
        }
        // Check columns
        for col in 0..3 {
            if (0..3).all(|row| self.board[row][col] == player) {
                return true;
            }
        }
        // Check diagonals
        if (0..3).all(|i| self.board[i][i] == player)
            || (0..3).all(|i| self.board[i][2 - i] == player)
        {
            return true;
        }

        return false;
    }

    fn is_full(&self) -> bool {
        self.board.iter().all(|row| row.iter().all(|&cell| cell != Cell::Empty))
    }

    pub fn heuristic(&self,player_to_max: Cell) -> i32 {
        if self.is_winner(player_to_max) {
            return 100;
        } else if self.is_winner(player_to_max.other_player()){
            return -100;
        }
        return 0;
    }

    pub fn empty_space(&self) -> Vec<(usize,usize)> {
        let mut res = vec![];

        self.board.iter().enumerate().for_each(|(x,row)| row.iter().enumerate().for_each(|(y,c)| {
            if *c == Cell::Empty {
                res.push((x,y))
            }
        }));


        res
    }


    pub fn show(&self) {
        println!("-------------");
        
        for i in 0..3 {
            print!("|");
            for j in 0..3 {
                match self.board[j][i] {
                    Cell::X => print!(" X "),
                    Cell::O => print!(" O "),
                    Cell::Empty => print!("   "),
                }
                print!("|")
            }
            println!();
            println!("-------------");
        }

        println!();

    }

    pub fn get_best_move(&self, player: Cell) -> Option<(usize,usize)> {
        assert_ne!(player, Cell::Empty);

        let empty_cells = self.empty_space();

        let mut game_copy = self.clone();

        let (mut max_score, mut best_move_coord): (i32,Option<(usize,usize)>) = (i32::MIN,None); 

        for coord_move in empty_cells {
            game_copy.play_move(player, coord_move);

            let score = ia::minmax(game_copy,1,false,player);
        
            if score > max_score {
                best_move_coord = Some(coord_move);
                max_score = score;
            }

            game_copy = self.clone();
        }

        return best_move_coord;
    }



}



#[test]
fn test_is_winner() {

    

    let mut t = TicTacToe::new();

    assert_eq!(t.is_winner(Cell::O),false);
    assert_eq!(t.is_winner(Cell::X),false);
    
    
    assert_eq!(t.is_winner(Cell::X),false);

}

#[test]
fn test_ended() {

    

    let mut t = TicTacToe::new();

    
    assert_eq!(t.ended().is_none(),true);

}