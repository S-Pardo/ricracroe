struct Game {
    board: [[u8;3];3],
    is_over: bool
}

impl Game {
    fn new() -> Game {
        let board = [
            [0u8;3],
            [0u8;3],
            [0u8;3]
        ];
        Game{board: board, is_over: false}
    }

    fn print_board(&self) {
        for i in 0..3 {
            println!("+---+---+---+");
            println!("| {} | {} | {} |", self.board[i][0], self.board[i][1], self.board[i][2]);
        }
        println!("+---+---+---+");
    }
}

fn main() {
    let game = Game::new();
    game.print_board();
}
