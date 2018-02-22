use std::io;
use std::cmp;

#[derive(Debug, Clone, PartialEq, Copy)]
enum Piece {
    X,
    O,
    Empty
}

// Convert piece to string
fn piece_to_string(piece: &Piece) -> String {
    match piece {
        &Piece::Empty => " ".to_string(),
        &Piece::X => "X".to_string(),
        &Piece::O => "O".to_string()
    }
}


struct Computer {

}

impl Computer {
    fn new() -> Computer {
        Computer {}
    }

    fn calc_score(&self, winner: Piece) -> i32 {
        if winner == Piece::X {
            return -10;
        } else if winner == Piece::O {
            return 10;
        } else {
            return 0;
        }
    }

    fn mini_max_vec(&mut self, board: &Board, alpha: &mut i32, beta: &mut i32) -> i32 {
        let moves = Board::available_moves(board);

        // Leaf
        if moves.len() == 0 {
            let score: i32 = self.calc_score(board.get_winner());
            return score;
        }else {
            // Alpha-beta prunning
            if board.player == Piece::X {
                let mut movement_score = -20;

                for movement in &moves {
                    movement_score = cmp::max(movement_score, self.mini_max_vec(&movement, alpha, beta));
                    *alpha = cmp::max(*alpha, movement_score);

                    if beta < alpha {
                        break;
                    }
                }
                return movement_score;
            } else {
                let mut movement_score = 20;
                for movement in &moves {
                    movement_score = cmp::min(movement_score, self.mini_max_vec(&movement, alpha, beta));
                    *beta = cmp::min(*beta, movement_score);

                    if beta < alpha {
                        break;
                    }
                }
                return movement_score;
            }
        }
    }
}


#[derive(Debug, Clone)]
struct Board {
    player: Piece,
    last_movement: usize,
    pieces: Vec<Piece>
}

impl Board {
    fn new() -> Board {
        Board {
            player: Piece::X,
            last_movement: 0,
            pieces: vec![Piece::Empty; 9]
        }
    }

    // Return all the available movements
    fn available_moves(board: &Board) -> Vec<Board> {
        let mut moves: Vec<Board> = vec![];

        if board.get_winner() == Piece::Empty {
            for position in board.get_empty_positions() {
                let mut board = board.clone();
                board.set_piece(position);
                moves.push(board);
            }
        }

        return moves;
    }
    
     // Change piece by the other one
    fn change_piece(piece: &Piece) -> Piece {
        
        if piece == &Piece::X {
            return Piece::O;
        } else {
            return Piece::X;
        }
    }

    // Realize a movement
    fn set_piece(&mut self, position: usize) {
        if self.pieces[position] != Piece::Empty {
            println!("Invalid movement!");
        } else {
            self.pieces[position] = self.player.clone();
            self.last_movement = position;

            self.player = Board::change_piece(&self.player);
        }
        
    }

    // Convert a board index to a Piece Enum
    fn index_to_piece(&self, index: usize) -> Piece {
        match &self.pieces[index] {
            &Piece::Empty => Piece::Empty,
            &Piece::X => Piece::X,
            &Piece::O => Piece::O 
        }
    }

    // Return the indexes of the empty positions
    fn get_empty_positions(&self) -> Vec<usize> {
        let mut empty_positions: Vec<usize> = vec![];

        // Check if there's a empty position
        for (index, piece) in self.pieces.iter().enumerate() {
            if piece == &Piece::Empty {
                empty_positions.push(index)
            } else {
                continue
            }
        }

        return empty_positions;
    }

    // Return the piece winner (Empty == draw)
    fn get_winner(&self) -> Piece {
        let empty_positions = self.get_empty_positions().len();

        // Check the winner
        if empty_positions <= 6 {
            if self.pieces[0] == self.pieces[1] && self.pieces[1] == self.pieces[2] {
                return self.index_to_piece(0);
            } else if self.pieces[3] == self.pieces[4] && self.pieces[4] == self.pieces[5] {
                return self.index_to_piece(3);
            } else if self.pieces[6] == self.pieces[7] && self.pieces[7] == self.pieces[8] {
                return self.index_to_piece(6);
            } else if self.pieces[0] == self.pieces[3] && self.pieces[3] == self.pieces[6] {
                return self.index_to_piece(0);
            } else if self.pieces[1] == self.pieces[4] && self.pieces[4] == self.pieces[7] {
                return self.index_to_piece(1);
            } else if self.pieces[2] == self.pieces[5] && self.pieces[5] == self.pieces[8] {
                return self.index_to_piece(2);
            } else if self.pieces[0] == self.pieces[4] && self.pieces[4] == self.pieces[8] {
                return self.index_to_piece(0);
            } else if self.pieces[2] == self.pieces[4] && self.pieces[4] == self.pieces[6] {
                return self.index_to_piece(2);
            } else {
                return Piece::Empty;
            }
        }

        return Piece::Empty
    }

    // Print the board on the screen
    fn print(board: &Board) {
        println!("-------");
        println!("|{}|{}|{}|", piece_to_string(&board.pieces[0]), piece_to_string(&board.pieces[1]), piece_to_string(&board.pieces[2]));
        println!("|{}|{}|{}|", piece_to_string(&board.pieces[3]), piece_to_string(&board.pieces[4]), piece_to_string(&board.pieces[5]));
        println!("|{}|{}|{}|", piece_to_string(&board.pieces[6]), piece_to_string(&board.pieces[7]), piece_to_string(&board.pieces[8]));
        println!("-------");
    }
}

fn main() {
    println!("@ Tic-Tac-Toe Game @");
    let mut board = Board::new();
    let mut computer = Computer::new();

    // While there's moves left and no winner
    while Board::available_moves(&board).len() > 0 && board.get_winner() == Piece::Empty {
        // Computer plays
        if board.player == Piece::O {
            let moves = Board::available_moves(&board);
            let mut score: i32 = -10;
            let mut best_move: &Board = &moves[0];
            let mut alpha = -20;
            let mut beta = 20;

            for movement in &moves {
                let m_score = computer.mini_max_vec(&movement, &mut alpha, &mut beta);

                // Check if the actual move is better than the actual best_move
                if m_score > score {
                    best_move = movement;
                    score = m_score;
                }
            }

            board.set_piece(best_move.last_movement);
        } else { // User plays
            println!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@");
            Board::print(&board);
            println!("Player {} what's your move? (from 1 to 9)", piece_to_string(&board.player));
            let mut movement = String::new();

            io::stdin().read_line(&mut movement).expect("Failed to read the movement");
            let movement: usize = movement.trim().parse().expect("Failed to read the movement as number");

            board.set_piece(movement - 1);
            println!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@");
        }
    }

    // End of game
    Board::print(&board);

    // Show the game result
    if board.get_winner() == Piece::Empty {
        println!("Draw game!");
    } else {
        println!("There's a winner: {}", piece_to_string(&board.get_winner()));  
    }
    
}
