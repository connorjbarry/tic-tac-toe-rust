mod game;
use game::Board;
use std::io;

fn game_flow(board: &mut Board, row: usize, col: usize) {
    board.make_move(row, col);
    board.print_board();
}

fn prompt_user_for_move(board: &mut Board) {
    println!("Player {}, please enter the position of your move: ", board.turn);
    let mut move_made = String::new();
    io::stdin().read_line(&mut move_made).unwrap();
    let mut move_made = move_made.split_whitespace();
    let row: usize = move_made.next().unwrap().parse().unwrap();
    let col: usize = move_made.next().unwrap().parse().unwrap();
    game_flow(board, row, col);
}

fn main() {
    let mut board: Board = Board::new();
    prompt_user_for_move(&mut board);

    while !board.is_winner() {
        prompt_user_for_move(&mut board);
    }
    println!("Player {} wins!", board.winner);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_board() {
        let board: Board = Board::new();
        assert_eq!(board.board, [[' '; 3]; 3]);
        assert_eq!(board.turn, 'X');
        assert_eq!(board.winner, ' ');
        assert_eq!(board.game_over, false);
    }

    #[test]
    fn test_make_move() {
        let mut board: Board = Board::new();
        board.make_move(0, 0);
        assert_eq!(board.board[0][0], 'X');
        assert_eq!(board.turn, 'O');
        assert_eq!(board.winner, ' ');
        assert_eq!(board.game_over, false);
    }

    #[test]
    fn test_change_turn() {
        let mut board: Board = Board::new();
        board.change_turn();
        assert_eq!(board.board, [[' '; 3]; 3]);
        assert_eq!(board.turn, 'O');
        assert_eq!(board.winner, ' ');
        assert_eq!(board.game_over, false);
    }

    #[test]
    fn test_check_winner() {
        let mut board: Board = Board::new();
        board.board = [['X'; 3]; 3];
        board.check_winner();
        assert_eq!(board.board, [['X'; 3]; 3]);
        assert_eq!(board.turn, 'X');
        assert_eq!(board.winner, 'X');
        assert_eq!(board.game_over, true);
    }
}