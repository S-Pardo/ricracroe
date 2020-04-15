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
}

fn main() {
    let game = Game::new();
}
