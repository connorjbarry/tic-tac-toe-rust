mod game;
use game::Board;
use std::io;

fn game_flow(board: &mut Board, row: usize, col: usize) {
    board.make_move(row, col);
    board.print_board();
}

fn main() {
    let mut board: Board = Board::new();
    println!("Player {}, please enter the position of your move: ", board.turn);
    let mut move_made = String::new();
    io::stdin().read_line(&mut move_made).unwrap();
    let mut move_made = move_made.split_whitespace();
    let row: usize = move_made.next().unwrap().parse().unwrap();
    let col: usize = move_made.next().unwrap().parse().unwrap();
    game_flow(&mut board, row, col);

    while !board.is_winner() {
        println!("Player {}, please enter the position of your move: ", board.turn);
        let mut move_made = String::new();
        io::stdin().read_line(&mut move_made).unwrap();
        let mut move_made = move_made.split_whitespace();
        let row: usize = move_made.next().unwrap().parse().unwrap();
        let col: usize = move_made.next().unwrap().parse().unwrap();
        game_flow(&mut board, row, col);    
    }
    println!("Player {} wins!", board.winner);
}
