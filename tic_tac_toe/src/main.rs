use std::io;

#[derive(Debug, Clone, PartialEq)]
enum Piece {
    X,
    O,
    Empty
}

fn piece_to_string(piece: &Piece) -> String {
    match piece {
        &Piece::Empty => " ".to_string(),
        &Piece::X => "X".to_string(),
        &Piece::O => "O".to_string()
    }
}


#[derive(Debug, Clone)]
struct Board {
    player: Piece,
    pieces: Vec<Piece>
}

impl Board {
    fn new() -> Board {
        Board {
            player: Piece::X,
            pieces: vec![Piece::Empty; 9]
        }
    }

    fn set_piece(&mut self, position: usize) {
        self.pieces[position] = self.player.clone();

        // Change the actual player
        if self.player == Piece::X {
            self.player = Piece::O;
        } else {
            self.player = Piece::X;
        }
    }

    fn index_to_piece(&mut self, index: usize) -> Piece {
        match &self.pieces[index] {
            &Piece::Empty => Piece::Empty,
            &Piece::X => Piece::X,
            &Piece::O => Piece::O 
        }
    }

    fn get_winner(&mut self) -> Piece {
        let mut empty_positions = 0;

        // Check if there's a empty position
        for position in self.pieces.iter() {
            match position {
                &Piece::Empty => {
                    empty_positions += 1
                },
                _ => continue
            }
        }

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
            } else if self.pieces[1] == self.pieces[4] && self.pieces[4] == self.pieces[5] {
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

    fn print(&mut self) {
        println!("-------");
        println!("|{}|{}|{}|", piece_to_string(&self.pieces[0]), piece_to_string(&self.pieces[1]), piece_to_string(&self.pieces[2]));
        println!("|{}|{}|{}|", piece_to_string(&self.pieces[3]), piece_to_string(&self.pieces[4]), piece_to_string(&self.pieces[5]));
        println!("|{}|{}|{}|", piece_to_string(&self.pieces[6]), piece_to_string(&self.pieces[7]), piece_to_string(&self.pieces[8]));
        println!("-------");
    }
}

fn main() {
    println!("@ Tic-Tac-Toe Game @");
    let mut game = Board::new();

    while game.get_winner() == Piece::Empty {
        println!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@");
        game.print();
        println!("Player {} whats your move? (from 0 to 8)", piece_to_string(&game.player));
        let mut movement = String::new();

        io::stdin().read_line(&mut movement).expect("Failed to read the movement");
        let movement: usize = movement.trim().parse().expect("Failed to read the movement as number");

        game.set_piece(movement);
        println!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@");
    }

    game.print();
    println!("There's a winner: {}", piece_to_string(&game.get_winner()));  
}
