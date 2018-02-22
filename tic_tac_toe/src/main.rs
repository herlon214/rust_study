use std::io;

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

// Search for max number in the vector
fn max(numbers: &Vec<i32>) -> i32 {
    let mut max = numbers[0];

    for &number in numbers {
        if number > max {
            max = number;
        }
    }

    return max;
}

// Search for min number in the vector
fn min(numbers: &Vec<i32>) -> i32 {
    let mut min = numbers[0];

    for &number in numbers {
        if number < min {
            min = number;
        }
    }

    return min;
}

struct Computer {
}

impl Computer {
    fn calc_score(&self, winner: Piece) -> i32 {
        if winner == Piece::X {
            return -10;
        } else if winner == Piece::O {
            return 10;
        } else {
            return 0;
        }
    }

    fn mini_max(&mut self, board: &Board) -> i32 {
        let moves = Board::available_moves(board);

        // Leaf
        if moves.len() == 0 {
            return self.calc_score(board.get_winner());
        }else {
            let mut scores: Vec<i32> = vec![];

            // Check the children's score
            for movement in &moves {
                scores.push(self.mini_max(&movement));
            }

            if board.player == Piece::X {
                return min(&scores);
            } else {
               return max(&scores);
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
    let mut computer = Computer{};

    // While there's moves left and no winner
    while Board::available_moves(&board).len() > 0 && board.get_winner() == Piece::Empty {
        // Computer plays
        if board.player == Piece::O {
            let moves = Board::available_moves(&board);
            let mut score: i32 = -10;
            let mut best_move: &Board = &moves[0];

            for movement in &moves {
                let m_score = computer.mini_max(&movement);
                if m_score > score {
                    best_move = movement;
                    score = m_score;
                }
            }

            board.set_piece(best_move.last_movement);
        } else { // User plays
            println!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@");
            Board::print(&board);
            println!("Player {} what's your move? (from 0 to 8)", piece_to_string(&board.player));
            let mut movement = String::new();

            io::stdin().read_line(&mut movement).expect("Failed to read the movement");
            let movement: usize = movement.trim().parse().expect("Failed to read the movement as number");

            board.set_piece(movement);
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
