use std::fmt;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum GamePiece {
    X,
    O,
    Dash
}

impl fmt::Display for GamePiece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str_version = match self {
            &GamePiece::X => "X",
            &GamePiece::O => "O",
            &GamePiece::Dash => "-"
        };
        write!(f, "{}", str_version)
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Direction {
    UpLeft,
    Left,
    DownLeft,
    Down,
    UpRight,
    Right,
    DownRight,
    Up
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str_version = match self {
            &Direction::UpLeft => "Up Left",
            &Direction::Left => "Left",
            &Direction::DownLeft => "Down Left",
            &Direction::Down => "Down",
            &Direction::DownRight => "Down Right",
            &Direction::Right => "Right",
            &Direction::UpRight => "Up Right",
            &Direction::Up => "Up"
        };
        write!(f, "{}", str_version)
    }
}
struct Column {
    num_rows: u8,
    next_row: u8,
    rows: Vec<GamePiece>
}

impl Column {
    fn new(num_rows: u8) -> Column {
        let mut col = Column {
            num_rows,
            next_row: 0,
            rows: Vec::new()
        };
        while col.rows.len() < (num_rows as usize) {
            col.rows.push(GamePiece::Dash);
        }
        col
    }

    fn play_piece(&mut self, piece: GamePiece) {
        if self.next_row >= self.num_rows {
            panic!("Can't play in this column!");
        }
        self.rows[(self.next_row as usize)] = piece;
        self.next_row += 1;
    }

    fn have_room(&self) -> bool {
        self.next_row < self.num_rows
    }
}
pub struct Board {
    num_rows: u8,
    num_columns: u8,
    last_move: u8,
    columns: Vec<Column>,
    next_move: GamePiece,
    num_moves: u16,
    connect_number: u8
}

impl Board {
    pub fn new_board(num_rows: u8, num_columns: u8, connect_number: u8) -> Board {
        let mut board = Board {
            num_rows,
            num_columns,
            connect_number,
            last_move: num_rows + 1,
            columns: Vec::new(),
            next_move: GamePiece::X,
            num_moves: 0
        };
        while board.columns.len() < (num_columns as usize) {
            board.columns.push(Column::new(num_rows));
        }
        board
    }

    pub fn print(&self) {
        println!("\n\n");
        for i in 0..self.num_columns {
            print!(" {} ", i);
        }
        print!("\n");
        for i in 0..self.num_columns {
            print!("===");
        }
        print!("\n");
        for row in (0..self.num_rows).rev() {
            for col in self.columns.iter() {
                match col.rows[(row as usize)] {
                    GamePiece::Dash => print!(" - "),
                    GamePiece::X => print!(" X "),
                    GamePiece::O => print!(" O ")
                }
            }
            print!("\n");
        }
    }

    // 0 for no winner, 1 for player 1, 2 for player 2
    pub fn have_winner(&self) -> u8 {
        //We will go through each direction:
        //up left, left, down left, down, up right, right, and down right
        let targ_length = self.connect_number - 1;
        if self.combined_streak(Direction::UpLeft, Direction::DownRight) >= targ_length || 
        self.combined_streak(Direction::Left, Direction::Right) >= targ_length ||
        self.combined_streak(Direction::UpRight, Direction::DownLeft) >= targ_length ||
        self.streak(&Direction::Down) >= targ_length {
            if let GamePiece::X = self.get_last_move() {
                1
            } else {
                2
            }
        } else {
            0
        }
    }

    pub fn combined_streak(&self, direction1: Direction, direction2: Direction) -> u8 {
        //let dir1 = self.streak(&direction1);
        //let dir2 = self.streak(&direction2);
        //let cur = self.last_move_location();
        //println!("Last Move: {}, {} - Streak in {} is {}, Streak in {} is {}", cur.0, cur.1, &direction1, dir1, &direction2, dir2);
        //dir1 + dir2
        self.streak(&direction1) + self.streak(&direction2)
    }

    pub fn streak(&self, direction: &Direction) -> u8 {
        let mut current_loc = self.last_move_location();
        let mut streak_length = 0; //dont count current piece
        let piece = self.get_last_move();
        loop {
            match self.next_position_in_direction(&direction, current_loc.0, current_loc.1) {
                None => { break; },
                Some(new_loc) => {
                    if piece == self.game_piece_at(new_loc.0, new_loc.1) {
                        streak_length += 1;
                        current_loc = new_loc;
                    } else {
                        break;
                    }
                }
            }
        }
        streak_length
    }

    fn next_position_in_direction(&self, in_direction: &Direction, x: u8, y: u8) -> Option<(u8, u8)> {
        let (delta_x, delta_y) = match in_direction {
            &Direction::UpLeft => (-1, 1),
            &Direction::Left => (-1, 0),
            &Direction::DownLeft => (-1, -1),
            &Direction::Down => (0, -1),
            &Direction::DownRight => (1, -1),
            &Direction::Right => (1, 0),
            &Direction::UpRight => (1,1),
            &Direction::Up => (0,1)
        };
        if (delta_x < 0 && x == 0) || (delta_y < 0 && y == 0) {
            return None;
        }
        if (delta_x > 0 && self.num_columns <= x) || (delta_y > 0 && self.num_rows <= y) {
            return None;
        }
        let new_x = match delta_x {
            -1 => x - 1,
            1 => x + 1,
            _ => x
        };
        let new_y = match delta_y {
            -1 => y - 1,
            1 => y + 1,
            _ => y
        };
        Some((new_x, new_y))
    }

    pub fn play_piece(&mut self, col: u8) -> bool {
        if self.valid_move(col) {
            let nm = self.get_current_move();
            self.columns[(col as usize)].play_piece(nm);
            self.next_move = self.get_next_move();
            self.last_move = col;
            self.num_moves += 1;
            true
        } else {
            false
        }
    }

    pub fn get_current_move(&self) -> GamePiece {
        if let GamePiece::X = self.next_move {
            GamePiece::X
        } else {
            GamePiece::O
        }
    }

    pub fn get_next_move(&self) -> GamePiece {
        if let GamePiece::X = self.next_move {
            GamePiece::O
        } else {
            GamePiece::X
        }
    }

    pub fn get_last_move(&self) -> GamePiece {
        self.get_next_move()
    }

    pub fn valid_move(&self, col: u8) -> bool {
        col < self.num_columns && self.columns[(col as usize)].have_room()
    }

    pub fn last_move_location(&self) -> (u8, u8) {
        (self.last_move, self.columns[(self.last_move as usize)].next_row - 1)
    }

    pub fn game_piece_at(&self, x: u8, y: u8) -> GamePiece {
        if x >= self.num_columns || y >= self.num_rows {
            return GamePiece::Dash;
        }
        match self.columns[(x as usize)].rows[(y as usize)] {
            GamePiece::X => GamePiece::X,
            GamePiece::O => GamePiece::O,
            GamePiece::Dash => GamePiece::Dash
        }
    }
}