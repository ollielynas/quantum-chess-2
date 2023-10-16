use crate::{piece::{Piece, Team}, console_log};
use js_sys;
use sycamore::web::html::tr; 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Vec2 {
    pub const ZERO: Vec2 = Vec2 { x: 0, y: 0 };
    pub fn from_index(index: usize) -> Vec2 {
        Vec2 {
            x: (index % 8) as i32,
            y: (index as f32 / 8.0).floor() as i32,
        }
    }
    pub fn to_index(&self) -> usize {
        (self.x + self.y * 8) as usize
    }
    pub fn new<T: Into<Vec2>>(a:T) -> Vec2 {
        a.into()
    }
    pub fn add<T: Into<Vec2>>(&self, a:T) -> Option<Vec2> {
        let b = a.into();
        let new = Vec2 { x: self.x + b.x, y: self.y+b.y };
        if (0..8).contains(&new.x) && (0..8).contains(&new.y) {
            return Some(new);
        }else{
            return None;
        }
    }
}

impl Into<Vec2> for (i32,i32) {
    fn into(self) -> Vec2 {
        Vec2 { x: self.0, y: self.1 }
    }
}
#[derive(Clone, Debug)]
pub struct Board {
    grid: [[Vec<Piece>; 8]; 8],
    current_player: Team,
}

impl Board {
    pub fn get_square_mut(&mut self, position: Vec2) -> &mut Vec<Piece> {
        self.grid
            .get_mut(position.y as usize)
            .unwrap()
            .get_mut(position.x as usize)
            .unwrap()
    }

    pub fn get_square(&self, position: Vec2) -> Vec<Piece> {
        self.grid
            .get(position.y as usize)
            .unwrap()
            .get(position.x as usize)
            .unwrap()
            .to_vec()
    }

    pub fn square_positions() -> [Vec2; 64] {
        let mut pos = [Vec2::ZERO; 64];
        for i in 0..64 {
            pos[i] = Vec2::from_index(i);
        }
        return pos;
    }

    pub fn add_pieces_to_square(&mut self, mut pieces: Vec<Piece>, position: Vec2) {
        let square = &mut self.grid[position.y as usize][position.x as usize];
        let mut duplicates = (0..pieces.len()).map(|_| false).collect::<Vec<bool>>();

        

        if square.len() == 1 && pieces.len() ==1 && square[0].team != pieces[0].team  {
            self.grid[position.y as usize][position.x as usize] =  pieces;
            return;
        }

        for i in &pieces {
            let mut index = 0;
            for o in &mut *square {
                if o.team == i.team && o.type_ == i.type_ {
                    o.percent += i.percent;
                    duplicates[index] = true;
                    break;
                }
                index += 1;
                
            }
        }

        for (i,k) in pieces.iter().enumerate() {
            if !duplicates[i] {
                square.push(k.clone());
            }
        };

        // self.grid[position.y as usize][position.x as usize].append(&mut pieces);
    }

    pub fn reset(&mut self) {
        self.grid = Default::default();
        for p in Board::square_positions() {
            if let Some(new_piece) = Piece::from_setup_pos(&p) {
                self.add_pieces_to_square(vec![new_piece], p);
            }
        }
    }
    pub fn new_blank() -> Board {
        let grid: [[Vec<Piece>; 8]; 8] = Default::default();
        Board { grid, current_player: Team::White}
    }
    pub fn new_setup() -> Board {
        let mut new = Board::new_blank();
        new.reset();
        return new;
    }

    pub fn update_piece_data(&mut self) {
        let mut valid_moves = [[true; 8]; 8];

        // generate a list of occupied squares
        for p in Board::square_positions() {
            let p_vec = self.get_square(p);
            if matches!(p_vec.first(), Some(a) 
                if a.calculated_values.relative_percent == 1.0 && p_vec.len() == 1) {
                    valid_moves[p.y as usize][p.x as usize] = false;
            }
        }

        for p in Board::square_positions() {
            let mut square = self.get_square(p);
            let total: f32 = square.iter().map(|x| x.percent).sum();
            for i in square.iter_mut() {
                i.update_calculated_values(&p, valid_moves, total, self.current_player);
            }
            self.grid[p.y as usize][p.x as usize] = square;
        }

        


    }

    pub fn move_piece(&mut self, position: Vec2) {
        let mut to_move:Vec<(Vec2, usize)> = vec![];
        for p in Board::square_positions() {
            for (a,i) in self.grid[p.y as usize][p.x as usize].iter().enumerate() {
                if i.calculated_values.moves.contains(&position) {
                    to_move.push((p, a))
                }
            }
        }

        let count = to_move.len();
        let mut new_pieces: Vec<Piece> = vec![];
        for (piece_pos, piece_idx) in &to_move {
            let piece = self.grid[piece_pos.y as usize][piece_pos.x as usize][*piece_idx].clone();
            let take_percent = piece.percent / count as f32;
            let remaining_percent = piece.percent - take_percent;
            let new = piece.with_percent(take_percent);
            self.grid[piece_pos.y as usize][piece_pos.x as usize][*piece_idx] = piece.with_percent(remaining_percent);
            let mut is_dupe = false;
            for i in 0..new_pieces.len() {
                if new.team == new_pieces[i].team && new.type_ == new_pieces[i].type_ {
                    new_pieces[i].percent += new.percent;
                    is_dupe = true;
                    break;
                }
            }
            if !is_dupe {
                new_pieces.push(piece.with_percent(take_percent));
            }
        }
        let length = new_pieces.len();
        self.add_pieces_to_square(new_pieces, position);
        if length > 0 {
        self.current_player = match self.current_player {
            Team::White => Team::Black,
            Team::Black => Team::White,
        }
        }

    }

}
