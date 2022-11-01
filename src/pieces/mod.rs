#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Piece {
    pub representation: char,
    pub team: char,
    pub movements: Vec<[i8; 2]>,
    pub cords: [i8;2],
    pub selected: bool,
    pub firts_movement:bool
}