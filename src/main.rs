mod pieces;

use functions::green_turn;

use crate::pieces::Piece;

mod functions;
use crate::functions::{
    generate_bishop_movements, generate_queen_movements, generate_tower_movements, render_board,
};

fn main() {
    
    let mut all_pieces = vec![
        vec![
            
            Piece {
                representation: 'p',
                team: 'W',
                movements: vec![[1, 0], [2, 0], [1, 1], [1, -1]],
                cords: [1, 0],
                selected: false,
            },
            Piece {
                representation: 'p',
                team: 'W',
                movements: vec![[1, 0], [2, 0], [1, 1], [1, -1]],
                cords: [1, 1],
                selected: false,
            },
            Piece {
                representation: 'p',
                team: 'W',
                movements: vec![[1, 0], [2, 0], [1, 1], [1, -1]],
                cords: [1, 2],
                selected: false,
            },
            Piece {
                representation: 'p',
                team: 'W',
                movements: vec![[1, 0], [2, 0], [1, 1], [1, -1]],
                cords: [1, 3],
                selected: false,
            },
            Piece {
                representation: 'p',
                team: 'W',
                movements: vec![[1, 0], [2, 0], [1, 1], [1, -1]],
                cords: [1, 4],
                selected: false,
            },
            Piece {
                representation: 'p',
                team: 'W',
                movements: vec![[1, 0], [2, 0], [1, 1], [1, -1]],
                cords: [1, 5],
                selected: false,
            },
            Piece {
                representation: 'p',
                team: 'W',
                movements: vec![[1, 0], [2, 0], [1, 1], [1, -1]],
                cords: [1, 6],
                selected: false,
            },
            Piece {
                representation: 'p',
                team: 'W',
                movements: vec![[1, 0], [2, 0], [1, 1], [1, -1]],
                cords: [1, 7],
                selected: false,
            },
            Piece {
                representation: 'T',
                team: 'W',
                movements: generate_tower_movements(),
                cords: [0, 0],
                selected: false,
            },
            Piece {
                representation: 'C',
                team: 'W',
                movements: vec![
                    [-2, -1],
                    [-2, 1],
                    [-1, 2],
                    [-1, -2],
                    [2, 1],
                    [2, -1],
                    [1, -2],
                    [1, 2],
                ],
                cords: [0, 1],
                selected: false,
            },
            Piece {
                representation: 'B',
                team: 'W',
                movements: generate_bishop_movements(),
                cords: [0, 2],
                selected: false,
            },
            Piece {
                representation: 'K',
                team: 'W',
                movements: vec![
                    [1, 1],
                    [1, 0],
                    [1, -1],
                    [0, 1],
                    [0, -1],
                    [-1, 1],
                    [0, 0],
                    [-1, 0],
                ],
                cords: [0, 3],
                selected: false,
            },
            Piece {
                representation: 'Q',
                team: 'W',
                movements: generate_queen_movements(),
                cords: [0, 4],
                selected: false,
            },
            Piece {
                representation: 'T',
                team: 'W',
                movements: generate_tower_movements(),
                cords: [0, 7],
                selected: false,
            },
            Piece {
                representation: 'C',
                team: 'W',
                movements: vec![
                    [-2, -1],
                    [-2, 1],
                    [-1, 2],
                    [-1, -2],
                    [2, 1],
                    [2, -1],
                    [1, -2],
                    [1, 2],
                ],
                cords: [0, 6],
                selected: false,
            },
            Piece {
                representation: 'B',
                team: 'W',
                movements: generate_bishop_movements(),
                cords: [0, 5],
                selected: false,
            },
        ],
        vec![
            Piece {
                representation: 'p',
                team: 'B',
                movements: vec![[1, 0], [2, 0], [1, 1], [1, -1]],
                cords: [6, 0],
                selected: false,
            },
            Piece {
                representation: 'p',
                team: 'B',
                movements: vec![[1, 0], [2, 0], [1, 1], [1, -1]],
                cords: [6, 1],
                selected: false,
            },
            Piece {
                representation: 'p',
                team: 'B',
                movements: vec![[1, 0], [2, 0], [1, 1], [1, -1]],
                cords: [6, 2],
                selected: false,
            },
            Piece {
                representation: 'p',
                team: 'B',
                movements: vec![[1, 0], [2, 0], [1, 1], [1, -1]],
                cords: [6, 3],
                selected: false,
            },
            Piece {
                representation: 'p',
                team: 'B',
                movements: vec![[1, 0], [2, 0], [1, 1], [1, -1]],
                cords: [6, 4],
                selected: false,
            },
            Piece {
                representation: 'p',
                team: 'B',
                movements: vec![[1, 0], [2, 0], [1, 1], [1, -1]],
                cords: [6, 5],
                selected: false,
            },
            Piece {
                representation: 'p',
                team: 'B',
                movements: vec![[1, 0], [2, 0], [1, 1], [1, -1]],
                cords: [6, 6],
                selected: false,
            },
            Piece {
                representation: 'p',
                team: 'B',
                movements: vec![[1, 0], [2, 0], [1, 1], [1, -1]],
                cords: [6, 7],
                selected: false,
            },
            Piece {
                representation: 'T',
                team: 'B',
                movements: generate_tower_movements(),
                cords: [7, 0],
                selected: false,
            },
            Piece {
                representation: 'C',
                team: 'B',
                movements: vec![
                    [-2, -1],
                    [-2, 1],
                    [-1, 2],
                    [-1, -2],
                    [2, 1],
                    [2, -1],
                    [1, -2],
                    [1, 2],
                ],
                cords: [7, 1],
                selected: false,
            },
            Piece {
                representation: 'B',
                team: 'B',
                movements: generate_bishop_movements(),
                cords: [7, 2],
                selected: false,
            },
            Piece {
                representation: 'K',
                team: 'B',
                movements: vec![
                    [1, 1],
                    [1, 0],
                    [1, -1],
                    [0, 1],
                    [0, -1],
                    [-1, 1],
                    [0, 0],
                    [-1, 0],
                ],
                cords: [7, 4],
                selected: false,
            },
            Piece {
                representation: 'Q',
                team: 'B',
                movements: generate_queen_movements(),
                cords: [7, 3],
                selected: false,
            },
            Piece {
                representation: 'T',
                team: 'B',
                movements: generate_tower_movements(),
                cords: [7, 7],
                selected: false,
            },
            Piece {
                representation: 'C',
                team: 'B',
                movements: vec![
                    [-2, -1],
                    [-2, 1],
                    [-1, 2],
                    [-1, -2],
                    [2, 1],
                    [2, -1],
                    [1, -2],
                    [1, 2],
                ],
                cords: [7, 6],
                selected: false,
            },
            Piece {
                representation: 'B',
                team: 'B',
                movements: generate_bishop_movements(),
                cords: [7, 5],
                selected: /*  */false,
            },
        ],
    ]; 

    let mut game_board: [[[char; 2]; 8]; 8] = [[['·', ' ']; 8]; 8];

    loop {
        for piece in &all_pieces[0] {
            game_board[piece.cords[0] as usize][piece.cords[1]  as usize] = [piece.representation, piece.team];
        }
        for piece in &all_pieces[1] {
            game_board[piece.cords[0]  as usize][piece.cords[1]  as usize] = [piece.representation, piece.team];
        }

        /* for row in game_board {
            println!("{:?}", row);
        } */

        render_board(&all_pieces);

        green_turn(&mut game_board, &mut all_pieces)
    }
}
