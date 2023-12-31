

use std::borrow;

use std::{cell::RefCell, rc::Rc};

use crate::game::{Board, Vec2};
use crate::piece::{Piece, PieceType};



use crate::{ReadSignal, Prop};
use sycamore::prelude::*;
use sycamore::view;
use console_error_panic_hook;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[macro_export]
macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => ($crate::html::log(&format_args!($($t)*).to_string()))
}

impl Board {
    pub fn square_html(&self, position: Vec2) -> String {
        let square_color = if (position.to_index()+(position.y%2) as usize)%2 == 0 {"black-square"} else {"white-square"};
        let mut inner_pieces = self.get_square(position);
        inner_pieces.sort_by(|a,b| a.percent.partial_cmp(&b.percent).unwrap_or(std::cmp::Ordering::Equal));
        let inner_html = inner_pieces.iter().map(|x|x.inner_html()).collect::<String>();
        let pos = format!("{},{}", position.x, position.y);

        format!(
            "<div
            id = '{pos}'
            class='square {square_color}'
            >
                {inner_html}
            </div>
            "
        )
    }
    pub fn all_html(&self) -> String {
        let inner_html = Board::square_positions().as_slice().iter().map(|f|
            self.square_html(*f)).collect::<String>();
        format!("
            <div class='game-div'>
            {inner_html}
            </div>
            <img src='data:image/png;base64, iVBORw0KGgoAAAANSUhEUgAAAAUA
    AAAFCAYAAACNbyblAAAAHElEQVQI12P4//8/w38GIAXDIBKE0DHxgljNBAAO
        9TXL0Y4OHwAAAABJRU5ErkJggg==' style='display:none', onload='add_events()'></img>
        "
        )
    }
}

impl Piece {
    pub fn inner_html(&self) -> String {
        let height = self.calculated_values.relative_percent*100.0;
        let name = self.type_.svg();
        let team = match self.team {
            crate::piece::Team::White => "piece-team-white",
            crate::piece::Team::Black => "piece-team-black",
        };
        let moves = &self.calculated_values.moves;
        let move_to_class = moves.iter().map(|f|format!("move-{}-{} ",f.x,f.y)).collect::<String>();

        format!(
            "<div 
                class='piece {team} {move_to_class}'
                style='
                    width: var(--square-size);
                    height: {height:.4}%'
            '>
                {name}
            </div>
            "
        )
    }
}


impl PieceType {
    fn name(&self) -> String {
        match self {
            PieceType::Pawn { first_move: _} => "pawn",
            PieceType::King => "king",
            PieceType::Queen => "queen",
            PieceType::Knight => "knight",
            PieceType::Bishop => "bishop",
            PieceType::Rook => "rook",
        }.to_owned()
    }
    pub fn svg(&self) -> String {
        match self {
            PieceType::Pawn { first_move:_ } => include_str!("svg\\pawn.svg"),
            PieceType::King => include_str!("svg\\king.svg"),
            PieceType::Queen => include_str!("svg\\queen.svg"),
            PieceType::Knight => include_str!("svg\\knight.svg"),
            PieceType::Bishop => include_str!("svg\\bishop.svg"),
            PieceType::Rook => include_str!("svg\\rook.svg"),
        }.to_owned()
    } 
}




pub fn init_web(board: Board) {

    sycamore::render(|cx| {
    let board_signal = create_signal(cx, RefCell::new(board));
    let html = create_memo(cx,|| {
        let a = board_signal.get().borrow_mut().all_html();
        a
    });
    let style = include_str!("main.css");

    


    view! { cx,
        button(id="update", on:click=|_| {
            if let Some(window) = web_sys::window() {
                if let Some(click_js) = window.get("click_pos") {
                    if let Some(str) = click_js.as_string() {
                        
                        let split_ = str.split(",").collect::<Vec<&str>>();
        
                        let x = split_[0].parse::<i32>().unwrap_or(-1);
                        let y = split_[1].parse::<i32>().unwrap_or(-1);
                        if x != -1 && y != -1 {
                        board_signal.get().borrow_mut().move_piece(Vec2 {x, y});
                        board_signal.get().borrow_mut().update_piece_data();
                        board_signal.trigger_subscribers();
                        }
                    }
                }
            };
        })
        
    
        style(dangerously_set_inner_html=&style)
        style(id="style2")
        div(dangerously_set_inner_html=&html.get())

        script() {
            ({include_str!("main.js")})
        }
    }
});
}