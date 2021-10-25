type Board = Vec<Vec<String>>;

pub struct Game {
    board: Board,
    is_player_one_turn: bool,
}

impl Game {
    fn new() -> Game {
        let row_one = vec![
            String::from("-"),
            String::from("-"),
            String::from("-"),
        ];
        let row_two = vec![
            String::from("-"),
            String::from("-"),
            String::from("-"),
        ];
        let row_three = vec![
            String::from("-"),
            String::from("-"),
            String::from("-"),
        ];
        Game {
            board: vec![row_one, row_two, row_three],
            is_player_one_turn: true,
        }
    }

    fn draw(self: &Self) {
        for row in &self.board {
            for column in row {
                print!("{} ", column);
            }
            println!("")
        }
    }

    pub fn start() {
        let game = Game::new();
        game.draw();
    }
}
