
use sycamore::prelude::*;

use game::Board;
use html::init_web;
use web_sys::Window;

pub mod game;
pub mod piece;
pub mod html;


fn main() {
    console_error_panic_hook::set_once();

    let mut board = Board::new_setup(); 
    board.update_piece_data();
    init_web(board);
}
