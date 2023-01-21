pub struct Board {
    pub board: [[char; 3]; 3],
    pub turn: char,
    pub winner: char,
    pub game_over: bool,
}

impl Board {
    pub fn new() -> Board {
        Board {
            board: [[' '; 3]; 3],
            turn: 'X',
            winner: ' ',
            game_over: false,
        }
    }

    pub fn print_board(&self) {
        println!("{}|{}|{}", self.board[0][0], self.board[0][1], self.board[0][2]);
        println!("{}|{}|{}", self.board[1][0], self.board[1][1], self.board[1][2]);
        println!("{}|{}|{}", self.board[2][0], self.board[2][1], self.board[2][2]);
    }

    pub fn make_move(&mut self, row: usize, col: usize) {
        if self.board[row][col] == ' ' {
            self.board[row][col] = self.turn;
            self.check_winner();
            self.change_turn();
        }
    }

    pub fn change_turn(&mut self) {
        if self.turn == 'X' {
            self.turn = 'O';
        } else {
            self.turn = 'X';
        }
    }

    pub fn check_winner(&mut self) {
        for i in 0..3 {
            if self.board[i][0] == self.board[i][1] && self.board[i][1] == self.board[i][2] && self.board[i][0] != ' ' {
                self.winner = self.board[i][0];
                self.game_over = true;
            }
        }

        for i in 0..3 {
            if self.board[0][i] == self.board[1][i] && self.board[1][i] == self.board[2][i] && self.board[0][i] != ' ' {
                self.winner = self.board[0][i];
                self.game_over = true;
            }
        }

        if self.board[0][0] == self.board[1][1] && self.board[1][1] == self.board[2][2] && self.board[0][0] != ' ' {
            self.winner = self.board[0][0];
            self.game_over = true;
        }

        if self.board[0][2] == self.board[1][1] && self.board[1][1] == self.board[2][0] && self.board[0][2] != ' ' {
            self.winner = self.board[0][2];
            self.game_over = true;
        }
    }

    pub fn is_winner(&self) -> bool {
        self.game_over
    }

}
