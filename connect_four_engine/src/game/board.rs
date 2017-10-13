
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

#[derive(PartialEq)]
pub enum GamePiece {
    X,
    O,
    Dash
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
        if self.streak(-1,1,0) || self.streak(-1,0,0) || self.streak(-1,-1,0) ||
            self.streak(0,-1,0) || self.streak(1,1,0) || self.streak(1,0,0) ||
            self.streak(1,-1,0) {
            if let GamePiece::X = self.get_last_move() {
                1
            } else {
                2
            }
        } else {
            0
        }
    }

    pub fn streak(&self, x_delta: i8, y_delta: i8, mut length: u8) -> bool {
        if length == 0 {
            length = self.connect_number;
        }
        let mut current_loc = self.last_move_location();
        if (x_delta < 0 && current_loc.0 < length - 1) || (y_delta < 0 && current_loc.1 < length - 1){
            return false;
        }
        let mut streak_length = 1; //current piece counts as one!
        let piece = self.get_last_move();
        while streak_length < length {
            match x_delta {
                -1 => current_loc.0 -= 1,
                1 => current_loc.0 += 1,
                _ => ()
            }
            match y_delta {
                -1 => current_loc.1 -= 1,
                1 => current_loc.1 += 1,
                _ => ()
            }
            if self.game_piece_at(current_loc.0, current_loc.1) == piece {
                streak_length += 1;
            } else {
                return false;
            }
        }
        true
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

    fn get_current_move(&self) -> GamePiece {
        if let GamePiece::X = self.next_move {
            GamePiece::X
        } else {
            GamePiece::O
        }
    }

    fn get_next_move(&self) -> GamePiece {
        if let GamePiece::X = self.next_move {
            GamePiece::O
        } else {
            GamePiece::X
        }
    }

    fn get_last_move(&self) -> GamePiece {
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