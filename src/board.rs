use std::io;

type Board = Vec<Vec<String>>;

pub struct Game {
    board: Board,
    is_player_one_turn: bool,
}

impl Game {
    fn new() -> Game {
        let row_one = vec![String::from("-"), String::from("-"), String::from("-")];
        let row_two = vec![String::from("-"), String::from("-"), String::from("-")];
        let row_three = vec![String::from("-"), String::from("-"), String::from("-")];
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
            print!("\n")
        }
    }

    fn validate(input: String) -> Result<usize, String> {
        match input.trim().parse::<usize>() {
            Err(_) => Err(String::from("Please input a valid unsigned integer!")),
            Ok(number) => {
                if number > 0 && number < 4 {
                    Ok(number - 1)
                } else {
                    Err(String::from(
                        "Please input a number, between \
                            1 and 9, not already chosen!",
                    ))
                }
            }
        }
    }

    pub fn get_u32(prompt: String) -> usize {
        loop {
            let mut player_input = String::new();
            println!("\n{}", prompt);
            match io::stdin().read_line(&mut player_input) {
                Err(_) => print!("Error reading input, try again!"),
                Ok(_) => match Game::validate(player_input) {
                    Err(err) => print!("{}", err),
                    Ok(num) => return num,
                },
            }
        }
    }

    pub fn get_move() -> (usize, usize) {
        let row = Game::get_u32(String::from("What is the row number?"));
        let column = Game::get_u32(String::from("What is the column number?"));
        return (row, column);
    }

    pub fn is_game_over() -> u32 {
        return 2;
    }

    pub fn start() {
        let mut game = Game::new();
        loop {
            match Game::is_game_over() {
                0 => println!("X WINS!"),
                1 => println!("O WINS"),
                _ => {
                    let (row, column) = Game::get_move();
                    if game.board[row][column] == "-" {
                        if game.is_player_one_turn {
                            game.board[row][column] = String::from("x");
                            game.is_player_one_turn = false;
                        } else {
                            game.board[row][column] = String::from("o");
                            game.is_player_one_turn = true;
                        }
                    } else {
                        println!("Spot taken.")
                    }
                    game.draw();
                }
            }
        }
    }
}
