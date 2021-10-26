use std::io;

type Board = Vec<Vec<String>>;

pub struct Game {
    board: Board,
    is_player_one_turn: bool,
    is_game_over: bool,
}

impl Game {
    fn new() -> Game {
        let row_one = vec![String::from("-"), String::from("-"), String::from("-")];
        let row_two = vec![String::from("-"), String::from("-"), String::from("-")];
        let row_three = vec![String::from("-"), String::from("-"), String::from("-")];
        Game {
            board: vec![row_one, row_two, row_three],
            is_game_over: false,
            is_player_one_turn: true,
        }
    }

    fn draw(self: &Self) {
        for row in &self.board {
            for column in row {
                print!("{} ", column);
            }
            print!("\n")
        }
    }

    pub fn get_move() -> u32 {
        loop {
            let mut player_input = String::new();
            println!(
                "\nPlease enter your move (an integer between \
                1 and 9): "
            );

            match io::stdin().read_line(&mut player_input) {
                Err(_) => print!("Error reading input, try again!"),
                Ok(n) => match Game::validate(player_input) {
                    Err(err) => print!("{}", err),
                    Ok(num) => return num,
                },
            }
        }
    }

    fn validate(input: String) -> Result<u32, String> {
        match input.trim().parse::<u32>() {
            Err(_) => Err(String::from("Please input a valid unsigned integer!")),
            Ok(number) => {
                if number > 0 && number < 10 {
                    Ok(number)
                } else {
                    Err(String::from(
                        "Please input a number, between \
                            1 and 9, not already chosen!",
                    ))
                }
            }
        }
    }

    pub fn start() {
        let game = Game::new();
        loop {
            if game.is_game_over {
                break;
            }
            game.draw();
            Game::get_move();
        }
    }
}
