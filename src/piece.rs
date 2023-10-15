use crate::game::Vec2;


#[derive(Clone, Copy, Debug)]
pub enum PieceType {
    Pawn {first_move: bool},
    King,
    Queen,
    Knight,
    Bishop,
    Rook,
}



#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Team {
    White,
    Black,
}

#[derive(Debug, Clone)]
pub struct CalculatedValues {
    pub moves: Vec<Vec2>,
    pub relative_percent: f32,
}

impl Default for CalculatedValues {
    fn default() -> Self {
        CalculatedValues {
            moves: vec![],
            relative_percent: 1.0
        }
    }

}
#[derive(Debug, Clone)]
pub struct Piece {
    pub type_: PieceType,
    pub percent: f32,
    pub team: Team,
    pub calculated_values: CalculatedValues
}

impl Piece {
    pub fn from_type(type_: PieceType, team:Team) -> Piece {
        Piece {
            type_,
            percent: 1.0,
            team,
            calculated_values: CalculatedValues::default()
        }
    }
    pub fn with_percent(&self, percent: f32) -> Piece {
        Piece { 
            type_: self.type_,
            percent: percent,
            team:self.team,
            calculated_values: CalculatedValues::default()
        }
    }
    pub fn from_setup_pos(position: &Vec2) -> Option<Piece> {

        let team = match position.y {
            0 | 1 => Team::Black,
            7 | 6 => Team::White,
            _ => {return None;}
        };

        let type_ = match position.x {
            _ if matches!(position.y, 1|6) => PieceType::Pawn {first_move: true},
            0 | 7 => PieceType::Rook,
            1 | 6 => PieceType::Knight,
            2 | 5 => PieceType::Bishop,
            3 =>     PieceType::Queen,
            4 =>     PieceType::King,
            _ => return None,
        };

        return Some(Piece {
            percent: 1.0,
            type_,
            team,
            calculated_values:CalculatedValues::default()
        });

    }


    pub fn valid_moves(&self, position: &Vec2, free_squares: [[bool;8];8]) -> Vec<Vec2> {
        self.type_.valid_moves(position, free_squares, self.team)
    }

    pub fn update_calculated_values(&mut self, position: &Vec2, free_squares: [[bool;8];8], total: f32) {
        
        let rel = if total <= 1.0 {self.percent} else {self.percent/total};
        let new = CalculatedValues {
            moves: self.valid_moves(position, free_squares),
            relative_percent: rel,
        };
        self.calculated_values = new;
    }

}

impl PieceType {
    fn valid_moves(&self, position: &Vec2, free_squares: [[bool;8];8], team: Team) -> Vec<Vec2> {
        match self {
            PieceType::Pawn { first_move } => {
                let mut moves = vec![];
                let dir = if team==Team::Black {Vec2::new((0,1))}else{Vec2::new((0,-1))};
                if matches!(position.add(dir),Some(n) if free_squares[n.y as usize][n.x as usize]) {
                    moves.push(position.add(dir).unwrap());
                    let dir = if team==Team::Black {Vec2::new((0,2))}else{Vec2::new((0,-2))};
                    if *first_move && matches!(position.add(dir),Some(n) if free_squares[n.y as usize][n.x as usize]) {
                        moves.push(position.add(dir).unwrap());
                    };
                };
                moves   
            },
            PieceType::King => {
                let mut new = vec![
                    position.add((-1,0)),
                    position.add((1,0)),
                    position.add((0,1)),
                    position.add((0,-1)),
                    position.add((1,-1)),
                    position.add((-1,-1)),
                    position.add((-1,1)),
                    position.add((1,1)),
                ];
                
                new.retain(|l| matches!(l,Some(n) if free_squares[n.y as usize][n.x as usize]));
                new.iter().map(|f| f.unwrap()).collect::<Vec<Vec2>>()
            },
            PieceType::Queen => vec![],
            PieceType::Knight => {
                let mut new = vec![
                    position.add((1,-2)),
                    position.add((-1,-2)),
                    position.add((1,2)),
                    position.add((-1,2)),
                    position.add((2,-1)),
                    position.add((-2,-1)),
                    position.add((2,1)),
                    position.add((-2,1)),
                    ];
                    new.retain(|l| matches!(l,Some(n) if free_squares[n.y as usize][n.x as usize]));
                new.iter().map(|f| f.unwrap()).collect::<Vec<Vec2>>()
            }
            PieceType::Bishop => vec![],
            PieceType::Rook => vec![],
        }
    }
}