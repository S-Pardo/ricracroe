struct Game {
    board: [[i8; 3]; 3],
    is_over: bool,
}

impl Game {
    fn new() -> Game {
        let board = [[0i8; 3], [1i8; 3], [-1i8; 3]];
        Game {
            board: board,
            is_over: false,
        }
    }

    fn print_board(&self) {
        for i in 0..3 {
            println!("+---+---+---+");
            for j in 0..3 {
                print!(
                    "| {} ",
                    if self.board[i][j] == 0 {
                        " "
                    } else if self.board[i][j] == 1 {
                        "X"
                    } else {
                        "O"
                    }
                )
            }
            print!("|");
            println!("");
        }
        println!("+---+---+---+");
    }
}

fn main() {
    let game = Game::new();
    game.print_board();
}
