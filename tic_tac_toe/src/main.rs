use std::io;

#[derive(Debug)]
struct Game {
    actual_player: i32,
    board: Vec<String>
}

fn winner(board: &Vec<String>) -> String {
    let mut empty_positions = 0;
    let mut winner = "".to_string();

    // Check if there's a empty position
    for position in board.iter() {
        if position == "0" {
            empty_positions += 1;
        } else {
            continue
        }
    }

    // Check the winner
    if empty_positions == 0 {
        if board[0] == board[1] && board[1] == board[2] {
            winner = board[0].clone();
        } else if board[3] == board[4] && board[4] == board[5] {
            winner = board[0].clone();
        } else if board[6] == board[7] && board[7] == board[8] {
            winner = board[6].clone();
        } else if board[0] == board[3] && board[3] == board[6] {
            winner = board[0].clone();
        } else if board[1] == board[4] && board[4] == board[5] {
            winner = board[1].clone();
        } else if board[2] == board[5] && board[5] == board[8] {
            winner = board[2].clone();
        } else if board[0] == board[4] && board[4] == board[8] {
            winner = board[0].clone();
        } else if board[2] == board[4] && board[4] == board[6] {
            winner = board[2].clone();
        } else {
            winner = "DRAW".to_string();
        }
    }

    return winner;
}

fn print_board(game: &Game) {
    println!("-------");
    println!("|{}|{}|{}|", game.board[0], game.board[1], game.board[2]);
    println!("|{}|{}|{}|", game.board[3], game.board[4], game.board[5]);
    println!("|{}|{}|{}|", game.board[6], game.board[7], game.board[8]);
    println!("-------");
}

fn main() {
    println!("@ Tic-Tac-Toe Game @");
    let mut game = Game {
        actual_player: 1,
        board: vec!["0".to_string(); 9]
    };

    game.board[0] = "X".to_string();
    game.board[1] = "X".to_string();
    game.board[2] = "X".to_string();
    game.board[3] = "O".to_string();
    game.board[4] = "X".to_string();
    game.board[5] = "O".to_string();
    game.board[6] = "O".to_string();
    game.board[7] = "X".to_string();
    game.board[8] = "O".to_string();

    print_board(&game);
    println!("Winner: {}", winner(&game.board));

}
