struct Game {
    board: [[Option<Sign>; 3]; 3],
    turn: u8,
    state: GameState,
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum Sign {
    X,
    O,
}

#[derive(PartialEq, Eq)]
enum GameState {
    Active,
    Tie,
    Won { winner: u8 },
}

#[derive(Debug)]
enum GameError {
    InvalidPosition,
    NotCurrPlayer,
    PositionTaken,
}

impl Game {
    fn new() -> Self {
        Self {
            board: [[None; 3]; 3],
            turn: 0,
            state: GameState::Active,
        }
    }
    fn play(&mut self, row: usize, col: usize, player: u8) -> Result<(), GameError> {
        if row >= 3 || col >= 3 {
            return Err(GameError::InvalidPosition);
        }
        if self.board[row][col].is_some() {
            return Err(GameError::PositionTaken);
        }
        if player != self.turn % 2 {
            return Err(GameError::NotCurrPlayer);
        }
        self.board[row][col] = Some(if player == 0 { Sign::O } else { Sign::X });
        self.update_state();
        self.turn += 1;
        Ok(())
    }

    fn is_winning_trio(&self, trio: [(usize, usize); 3]) -> bool {
        let [first, second, third] = trio;
        if self.board[first.0][first.1].is_none()
            || self.board[second.0][second.1].is_none()
            || self.board[third.0][third.1].is_none()
        {
            return false;
        }
        let val1 = Self::extract_val(self.board[first.0][first.1]);
        let val2 = Self::extract_val(self.board[second.0][second.1]);
        let val3 = Self::extract_val(self.board[third.0][third.1]);
        val1 == val2 && val1 == val3
    }

    fn extract_val(opt: Option<Sign>) -> Sign {
        match opt {
            Some(x) => x,
            None => Sign::O, //never going to be executed
        }
    }

    fn update_state(&mut self) {
        for i in 0usize..3usize {
            if self.is_winning_trio([(i, 0), (i, 1), (i, 2)])
                || self.is_winning_trio([(0, i), (1, i), (2, i)])
            {
                self.state = GameState::Won {
                    winner: self.turn % 2,
                }
            }
        }
        if self.is_winning_trio([(0, 0), (1, 1), (2, 2)])
            || self.is_winning_trio([(0, 2), (1, 1), (2, 0)])
        {
            self.state = GameState::Won {
                winner: self.turn % 2,
            }
        }
        if self.turn == 8 {
            self.state = GameState::Tie
        }
    }

    fn is_playable(&self) -> bool {
        self.state == GameState::Active
    }

    fn print_board(&self) {
        println!("---------");
        for i in 0usize..3usize {
            for j in 0usize..3usize {
                match self.board[i][j] {
                    Some(Sign::O) => print!("|0|"),
                    Some(Sign::X) => print!("|X|"),
                    None => print!("| |"),
                }
            }
            println!("");
            println!("---------")
        }
    }
}

fn read(input: &mut String) {
    input.clear();
    std::io::stdin()
        .read_line(input)
        .expect("could not read input");
    input.pop();
}

fn main() {
    let mut row: usize;
    let mut col: usize;
    let mut curr_player = 0u8;
    let mut res;
    let mut input = String::with_capacity(3);
    let mut game = Game::new();

    game.print_board();
    while game.is_playable() {
        println!("Enter row & col");
        read(&mut input);
        row = input.parse().expect("invalid input");
        read(&mut input);
        col = input.parse().expect("invalid input");
        res = game.play(row, col, curr_player);
        match res {
            Ok(_) => curr_player = (curr_player + 1) % 2,
            Err(err) => println!("{:?}", err),
        }
        game.print_board();
    }
    match game.state {
        GameState::Tie => println!("Game tied!"),
        GameState::Won { winner } => println!("player{} won the game", winner),
        _ => println!(""), //never going to happen
    }
}
