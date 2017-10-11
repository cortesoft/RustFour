
pub fn new_board(num_rows: u8, num_columns: u8) -> Board {
    let mut board = Board {
        num_rows,
        num_columns,
        last_move: num_rows + 1,
        columns: Vec::new(),
        next_move: GamePiece::X
    };
    while board.columns.len() < (num_columns as usize) {
        board.columns.push(Column::new(num_rows));
    }
    board
}

enum GamePiece {
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
}
pub struct Board {
    num_rows: u8,
    num_columns: u8,
    last_move: u8,
    columns: Vec<Column>,
    next_move: GamePiece
}

impl Board {
    pub fn print(&self) {
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

    pub fn play_piece(&mut self, col: u8) {
        if col >= self.num_columns {
            panic!("Move outside of range");
        }
        let nm = self.get_next_move();
        self.columns[(col as usize)].play_piece(nm);
        match self.next_move {
            GamePiece::X => self.next_move = GamePiece::O,
            GamePiece::O => self.next_move = GamePiece::X,
            GamePiece::Dash => panic!("We should never have dash!")
        }
    }

    fn get_next_move(&self) -> GamePiece {
        if let GamePiece::X = self.next_move {
            GamePiece::X
        } else {
            GamePiece::O
        }
    } 
}