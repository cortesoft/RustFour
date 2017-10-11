mod board;

pub fn play() {
    let mut board = board::new_board(8, 8);
    board.print();
    board.play_piece(4);
    board.print();
    board.play_piece(6);
    board.print();
    board.play_piece(4);
    board.print();
    board.play_piece(4);
    board.print();
}