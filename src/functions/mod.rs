use std::io::stdin;

use colored::Colorize;

use crate::pieces::Piece;

const KNIGHT_REPRESENTATION: char = 'C';

pub fn check_if_pawn_can_kill(all_pieces: &mut Vec<Vec<Piece>>) {
    let white_pawn_idexes = all_pieces[0]
        .iter()
        .enumerate()
        .filter(|(_, piece)| piece.representation == 'p')
        .map(|(index, _)| index)
        .collect::<Vec<_>>();

    let black_pawn_idexes = all_pieces[1]
        .iter()
        .enumerate()
        .filter(|(_, piece)| piece.representation == 'p')
        .map(|(index, _)| index)
        .collect::<Vec<_>>();

    for pawn_index in white_pawn_idexes {
        let pawn = &mut all_pieces.clone()[0][pawn_index];
        all_pieces[0][pawn_index] = check_next_diagonal_moves(pawn, all_pieces, 'B');
    }

    for pawn_index in black_pawn_idexes {
        let pawn = &mut all_pieces.clone()[1][pawn_index];
        all_pieces[1][pawn_index] = check_next_diagonal_moves(pawn, all_pieces, 'W');
    }
    /*
    for index in white_pawn_idexes {
        let all_pieces_inmutable = all_pieces.clone();

        let mut pawn = &mut all_pieces[0][index];

        if !is_oob(&mut pawn, [1, 1]) {
            if !(check_cords(pawn.cords[0] + 1, pawn.cords[1] + 1, &all_pieces_inmutable).is_none())
            {
                if check_cords(pawn.cords[0] + 1, pawn.cords[1] + 1, &all_pieces_inmutable)
                    .unwrap()
                    .team
                    == 'B' && check_cords(pawn.cords[0] + 1, pawn.cords[1] + 1, &all_pieces_inmutable)
                    .unwrap().movements.iter().find(|movement| movement == &&[1 as i8, 1 as i8]).is_none()
                {
                    pawn.movements.push([1, 1])
                } else {
                    if pawn
                        .movements
                        .iter()
                        .find(|movement| movement == &&[1 as i8, 1 as i8])
                        .is_some()
                    {
                        pawn.movements.remove(
                            pawn.movements
                                .iter()
                                .position(|movement| movement == &[1 as i8, 1 as i8])
                                .unwrap(),
                        );
                    }
                }
            }else {
                if pawn
                    .movements
                    .iter()
                    .find(|movement| movement == &&[1 as i8, 1 as i8])
                    .is_some()
                {
                    pawn.movements.remove(
                        pawn.movements
                            .iter()
                            .position(|movement| movement == &[1 as i8, 1 as i8])
                            .unwrap(),
                    );
                }
            }
        }
        if !is_oob(&mut pawn, [1, -1]) {
            if !check_cords(pawn.cords[0] + 1, pawn.cords[1] - 1, &all_pieces_inmutable).is_none() {
                if check_cords(pawn.cords[0] + 1, pawn.cords[1] - 1, &all_pieces_inmutable)
                    .unwrap()
                    .team
                    == 'B' &&
                    check_cords(pawn.cords[0] + 1, pawn.cords[1] - 1, &all_pieces_inmutable)
                    .unwrap().movements.iter().find(|movement| movement == &&[1 as i8, -1 as i8]).is_none()
                {
                    pawn.movements.push([1, -1])
                } else {
                    if pawn
                        .movements
                        .iter()
                        .find(|movement| movement == &&[1 as i8, -1 as i8])
                        .is_some()
                    {
                        pawn.movements.remove(
                            pawn.movements
                                .iter()
                                .position(|movement| movement == &[1 as i8, -1 as i8])
                                .unwrap(),
                        );
                    }
                }
            }else {
                if pawn
                    .movements
                    .iter()
                    .find(|movement| movement == &&[1 as i8, -1 as i8])
                    .is_some()
                {
                    pawn.movements.remove(
                        pawn.movements
                            .iter()
                            .position(|movement| movement == &[1 as i8, -1 as i8])
                            .unwrap(),
                    );
                }
            }
        }
    }

    for index in black_pawn_idexes {
        let all_pieces_inmutable = all_pieces.clone();

        let mut pawn = &mut all_pieces[1][index];

        if !is_oob(&mut pawn, [-1, 1]) {
            if !check_cords(pawn.cords[0] - 1, pawn.cords[1] + 1, &all_pieces_inmutable).is_none() {
                if check_cords(pawn.cords[0] - 1, pawn.cords[1] + 1, &all_pieces_inmutable).unwrap().team == 'W' && check_cords(pawn.cords[0] - 1, pawn.cords[1] + 1, &all_pieces_inmutable).unwrap().movements.iter().find(|movement| movement == &&[1 as i8, 1 as i8]).is_none() {
                    pawn.movements.push([-1, 1])
                } else {
                    if pawn
                        .movements
                        .iter()
                        .find(|movement| movement == &&[-1 as i8, 1 as i8])
                        .is_some()
                    {
                        pawn.movements.remove(
                            pawn.movements
                                .iter()
                                .position(|movement| movement == &[-1 as i8, 1 as i8])
                                .unwrap(),
                        );
                    }
                }
            }else {
                if pawn
                    .movements
                    .iter()
                    .find(|movement| movement == &&[-1 as i8, 1 as i8])
                    .is_some()
                {
                    pawn.movements.remove(
                        pawn.movements
                            .iter()
                            .position(|movement| movement == &[-1 as i8, 1 as i8])
                            .unwrap(),
                    );
                }
            }
        }
        if !is_oob(&mut pawn, [1, -1]) {
            if !check_cords(pawn.cords[0] - 1, pawn.cords[1] - 1, &all_pieces_inmutable).is_none() {
                if check_cords(pawn.cords[0] - 1, pawn.cords[1] - 1, &all_pieces_inmutable)
                    .unwrap()
                    .team
                    == 'W' && check_cords(pawn.cords[0] - 1, pawn.cords[1] - 1, &all_pieces_inmutable).unwrap().movements.iter().find(|movement| movement == &&[1 as i8, 1 as i8]).is_none()
                {
                    pawn.movements.push([-1, -1])
                } else {
                    if pawn
                        .movements
                        .iter()
                        .find(|movement| movement == &&[-1 as i8, -1 as i8])
                        .is_some()
                    {
                        pawn.movements.remove(
                            pawn.movements
                                .iter()
                                .position(|movement| movement == &[-1 as i8, -1 as i8])
                                .unwrap(),
                        );
                    }
                }
            }else {
                if pawn
                    .movements
                    .iter()
                    .find(|movement| movement == &&[-1 as i8, -1 as i8])
                    .is_some()
                {
                    pawn.movements.remove(
                        pawn.movements
                            .iter()
                            .position(|movement| movement == &[-1 as i8, -1 as i8])
                            .unwrap(),
                    );
                }
            }
        }
    } */
}

fn check_next_diagonal_moves<'a>(
    pawn: &'a mut Piece,
    all_pieces: &'a Vec<Vec<Piece>>,
    enemy_team: char,
) -> Piece {
    let posible_movements;
    if enemy_team == 'B' {
        posible_movements = [[1, 1], [1, -1]];
    } else {
        posible_movements = [[-1, 1], [-1, -1]];
    }

    for posible_movement in posible_movements {
        if !is_oob(pawn, posible_movement) {
            let new_cords = add_arrays_i8_2(posible_movement, pawn.cords);

            let piece_in_new_cords = check_cords(new_cords[0], new_cords[1], all_pieces);

            if piece_in_new_cords.is_none() {
                if pawn
                    .movements
                    .iter()
                    .position(|movement| movement == &posible_movement)
                    .is_some()
                {
                    pawn.movements.remove(
                        pawn.movements
                            .iter()
                            .position(|movement| movement == &posible_movement)
                            .unwrap(),
                    );
                }
                continue;
            }

            if piece_in_new_cords.is_some()
                && piece_in_new_cords.unwrap().team == enemy_team
                && pawn
                    .movements
                    .iter()
                    .find(|movement| movement == &&posible_movement)
                    .is_none()
            {
                pawn.movements.push(posible_movement);
            }
        } else {
            if pawn
                .movements
                .iter()
                .position(|movement| movement == &posible_movement)
                .is_some()
            {
                pawn.movements.remove(
                    pawn.movements
                        .iter()
                        .position(|movement| movement == &posible_movement)
                        .unwrap(),
                );
            }
        }
    }
    return pawn.clone();
}

/// Una función que verifica si hay una pieza en un determinado cuadrado y \
/// devuelve una pieza si la hay o devulve un none si no
///
/// ## Parametros
///
/// * `Row` la fila a comprobar
/// * `col` la columna a comprobar
/// * `all_pieces` un vector de las piezas
///
/// ## Devuelve
///
/// * `Some(&Piece)` Devuleve una pieza si hay algo en esa casilla
/// * `None` Devuelve None si no hay una pieza
///

fn check_cords(row: i8, col: i8, all_pieces: &Vec<Vec<Piece>>) -> Option<&Piece> {
    for team in all_pieces {
        for piece in team {
            if [row, col] == piece.cords {
                return Some(piece);
            }
        }
    }
    return None;
}

/// Una funcion que renderiza el tablero en pantalla
///
/// ## Parametros
///
/// * `all_pieces` Un vector con todas las piezas del tablero
pub fn render_board(all_pieces: &Vec<Vec<Piece>>) {
    for row in 0..=7 {
        for col in 0..=7 {
            let piece = check_cords(row, col, all_pieces);

            if piece.is_none() {
                print!("_ ");
                continue;
            }

            let piece = piece.unwrap();

            match piece.team {
                'W' => {
                    if piece.selected {
                        print!("{} ", format!("{}", piece.representation).green().bold());
                    } else {
                        print!("{} ", format!("{}", piece.representation).green());
                    }
                }

                'B' => {
                    if piece.selected {
                        print!("{} ", format!("{}", piece.representation).blue().bold());
                    } else {
                        print!("{} ", format!("{}", piece.representation).blue());
                    }
                }
                _ => {
                    if piece.selected {
                        print!("{} ", format!("{}", piece.representation).red().bold());
                    } else {
                        print!("{} ", format!("{}", piece.representation).red());
                    }
                }
            }
        }
        print!(" {} \n", row + 1)
    }
    println!("\nA B C D E F G H\n")
}

fn add_arrays_i8_2(a: [i8; 2], b: [i8; 2]) -> [i8; 2] {
    return [a[0] + b[0], a[1] + b[1]];
}

/// Una funcion que comprueba si hay una pieza el el camino en horizontal de la pieza que estamos moviendo\
/// devuelve `true` si es posible y `false` si no lo es
fn check_horizontal_moves(
    movement: [i8; 2],
    piece_cords: [i8; 2],
    all_pieces: &Vec<Vec<Piece>>,
) -> bool {
    if movement[1] > 0 {
        // Comprobamos los movimientos hacia la derecha
        for i in 1..movement[1] {
            let square = check_cords(piece_cords[0], piece_cords[1] + i, all_pieces);
            if square.is_some() {
                println!(
                    "no te puedes mover por encima de un: {}",
                    square.unwrap().representation
                );
                return false;
            }
        }
    } else {
        // Comprobamos los movimientos hacia la izquierda
        for i in movement[1]..0 {
            let square = check_cords(piece_cords[0], piece_cords[1] + i, all_pieces);
            if square.is_some() {
                println!(
                    "no te puedes mover por encima de un: {}",
                    square.unwrap().representation
                );
                return false;
            }
        }
    }
    return true;
}

/// Una funcion que comprueba si hay una pieza el el camino en vertical de la pieza que estamos moviendo\
/// devuelve `true` si es posible y `false` si no lo es
fn check_vertical_moves(
    movement: [i8; 2],
    piece_cords: [i8; 2],
    all_pieces: &Vec<Vec<Piece>>,
) -> bool {
    if movement[0] > 0 {
        // Comprobamos los movimientos hacia abajo
        for i in 1..movement[0] {
            let square = check_cords(piece_cords[0] + i, piece_cords[1], all_pieces);
            if square.is_some() {
                println!(
                    "no te puedes mover por encima de un: {}",
                    square.unwrap().representation
                );
                return false;
            }
        }
    } else {
        // Comprobamos los movimientos hacia arriba
        for i in movement[0]..0 {
            let square = check_cords(piece_cords[0] + i, piece_cords[1], all_pieces);
            if square.is_some() {
                println!(
                    "no te puedes mover por encima de un: {}",
                    square.unwrap().representation
                );
                return false;
            }
        }
    }
    return true;
}

/// Una funcion que comprueba si hay una pieza el el camino en diagonal de la pieza que estamos moviendo\
/// devuelve `true` si es posible y `false` si no lo es
fn check_diagonal_moves(
    movement: [i8; 2],
    piece_cords: [i8; 2],
    all_pieces: &Vec<Vec<Piece>>,
) -> bool {
    if movement[0] > 0 && movement[1] > 0 {
        for i in 1..movement[0] {
            let square = check_cords(piece_cords[0] + i, piece_cords[1] + i, all_pieces);
            if square.is_some() {
                println!(
                    "no te puedes mover por encima de un: {}",
                    square.unwrap().representation
                );
                return false;
            }
        }
    }
    if movement[0] > 0 && movement[1] < 0 {
        for i in 1..movement[0] {
            let square = check_cords(piece_cords[0] + i, piece_cords[1] - i, all_pieces);
            if square.is_some() {
                println!(
                    "no te puedes mover por encima de un: {}",
                    square.unwrap().representation
                );
                return false;
            }
        }
    }
    if movement[0] < 0 && movement[1] > 0 {
        for i in 1..movement[1] {
            let square = check_cords(piece_cords[0] - i, piece_cords[1] + i, all_pieces);
            if square.is_some() {
                println!(
                    "no te puedes mover por encima de un: {}",
                    square.unwrap().representation
                );
                return false;
            }
        }
    }
    if movement[0] < 0 && movement[1] < 0 {
        for i in movement[1]..0 {
            let square = check_cords(piece_cords[0] - i, piece_cords[1] - i, all_pieces);
            if square.is_some() {
                println!(
                    "no te puedes mover por encima de un: {}",
                    square.unwrap().representation
                );
                return false;
            }
        }
    }
    return true;
}

/// Una funcion que comprueba que una ficha se oueda mover
/// en la direccion indicada
fn check_if_can_move(
    movement: [i8; 2],
    all_pieces: &Vec<Vec<Piece>>,
    piece_cords: [i8; 2],
) -> bool {
    //Comprobamso los movimientos horizontales
    if movement[0] == 0 {
        return check_horizontal_moves(movement, piece_cords, all_pieces);

    //Comprobamso los movimientos vericales
    } else if movement[1] == 0 {
        return check_vertical_moves(movement, piece_cords, all_pieces);

    // Comprovamos los movimientos diagonales
    } else {
        return check_diagonal_moves(movement, piece_cords, all_pieces);
    }
}

/// Una funcion que comprueba si una pieza se puede mover\
/// a una casilla indicada y en caso de ser posible la mueve\
/// devuelve `true` si se ha podido realizar el movimiento o\
/// `false` si no se ha podido
fn move_green_piece(
    game_board: &mut [[[char; 2]; 8]; 8],
    all_pieces: &mut Vec<Vec<Piece>>,
    selected_piece: &mut Piece,
    movement: [i8; 2],
    square_to_move: [i8; 2],
) -> bool {
    for piece_index in 0..all_pieces[0].len() {
        //He hecho esto, no tenia otra opcion, perdon 😥
        if format!("{:?}", all_pieces[0][piece_index]) == format!("{:?}", selected_piece) {
            // Si la pieza es un caballos no hacemos la comprobacion de si se puede mover
            if selected_piece.representation != KNIGHT_REPRESENTATION {
                if !check_if_can_move(movement, &all_pieces, selected_piece.cords) {
                    return false;
                }
            }

            // Comprobamos si hay una pieza de nuestro equipo en la casilla seleccionada
            if all_pieces[0]
                .iter()
                .find(|x| x.cords == square_to_move)
                .is_some()
            {
                println!("No te puedes mover ahi porque ya hay una pieza");

                return false;
            }

            // Comprobamos si hay una pieza de enemiga en la casilla seleccionada

            if all_pieces[1]
                .iter()
                .find(|x| x.cords == square_to_move)
                .is_some()
            {
                if selected_piece.representation == 'p'
                    && (movement == [1, 0] || movement == [2, 0])
                {
                    println!("No puedes matar una pieza de esa manera");
                    return false;
                }
                // Si la hay la eliminamos del vector de fichas
                let index_to_kill = all_pieces[1]
                    .iter()
                    .position(|x| x.cords == square_to_move)
                    .unwrap();
                all_pieces[1].remove(index_to_kill);
            }

            // Comprobamos si es el primer movimiento de una pieza
            if selected_piece.firts_movement {
                selected_piece.firts_movement = false;

                // Si es el primer movimiento de un peon qutamos que se pueda mover 2 casillas
                if selected_piece.representation == 'p' {
                    selected_piece.movements.remove(
                        selected_piece
                            .movements
                            .iter()
                            .position(|movement| movement == &[2 as i8, 0 as i8])
                            .unwrap(),
                    );
                }
            }

            //Actualizamos el tablero, el vector de piezas y la pieza a la nueva posicion
            game_board[selected_piece.cords[0] as usize][selected_piece.cords[1] as usize] =
                ['·', ' '];

            selected_piece.cords = square_to_move;

            selected_piece.selected = false;

            all_pieces[0][piece_index] = selected_piece.clone();

            return true;
        }
    }

    return false;
}

fn move_blue_piece(
    game_board: &mut [[[char; 2]; 8]; 8],
    all_pieces: &mut Vec<Vec<Piece>>,
    selected_piece: &mut Piece,
    movement: [i8; 2],
    square_to_move: [i8; 2],
) -> bool {
    for piece_index in 0..all_pieces[1].len() {
        //He hecho esto, no tenia otra opcion, perdon 😥
        if format!("{:?}", all_pieces[1][piece_index]) == format!("{:?}", selected_piece) {
            // Si la pieza es un caballos no hacemos la comprobacion de si se puede mover
            if selected_piece.representation != KNIGHT_REPRESENTATION {
                if !check_if_can_move(movement, &all_pieces, selected_piece.cords) {
                    return false;
                }
            }

            // Comprobamos si hay una pieza de nuestro equipo en la casilla seleccionada
            if all_pieces[1]
                .iter()
                .find(|x| x.cords == square_to_move)
                .is_some()
            {
                println!("No te puedes mover ahi porque ya hay una pieza");

                return false;
            }

            // Comprobamos si hay una pieza de enemiga en la casilla seleccionada

            if all_pieces[0]
                .iter()
                .find(|x| x.cords == square_to_move)
                .is_some()
            {
                if selected_piece.representation == 'p'
                    && (movement == [-1, 0] || movement == [-2, 0])
                {
                    println!("No puedes matar una pieza de esa manera");
                    return false;
                }
                // Si la hay la eliminamos del vector de fichas
                let index_to_kill = all_pieces[0]
                    .iter()
                    .position(|x| x.cords == square_to_move)
                    .unwrap();
                all_pieces[0].remove(index_to_kill);
            }

            // Comprobamos si es el primer movimiento de una pieza
            if selected_piece.firts_movement {
                selected_piece.firts_movement = false;

                // Si es el primer movimiento de un peon qutamos que se pueda mover 2 casillas
                if selected_piece.representation == 'p' {
                    selected_piece.movements.remove(
                        selected_piece
                            .movements
                            .iter()
                            .position(|movement| movement == &[-2 as i8, 0 as i8])
                            .unwrap(),
                    );
                }
            }

            //Actualizamos el tablero, el vector de piezas y la pieza a la nueva posicion
            game_board[selected_piece.cords[0] as usize][selected_piece.cords[1] as usize] =
                ['·', ' '];

            selected_piece.cords = square_to_move;

            selected_piece.selected = false;

            all_pieces[1][piece_index] = selected_piece.clone();

            return true;
        }
    }

    return false;
}

/// Pregunta al usuario unas cordenadas, se comprueba si se puede \
/// seleccionar la pieza y la seleccion
///
/// ## Devuelve
///
/// * `Piece` devuelve la pieza seleccionada
///
fn select_piece(
    all_pieces: &mut Vec<Vec<Piece>>,
    game_board: &mut [[[char; 2]; 8]; 8],
    turn: char,
) -> Piece {
    let selected_piece: Piece;
    'select_piece: loop {
        // Pedimos al usuario unas cordenadas
        let cords_selected = user_select_square();

        // Seleccionamos el cuadrado del tablero con las cordenadas
        let square = square_from_cords(cords_selected, game_board);

        match square[1] {
            'W' => {
                if turn == 'W' {
                    for piece in &mut all_pieces[0] {
                        if piece.cords != cords_selected {
                            continue;
                        }

                        println!(
                            "Se ha seleccionado {:?} en la casilla {:?} ",
                            piece, cords_selected
                        );

                        piece.selected = true;
                        selected_piece = piece.clone();
                        break 'select_piece;
                    }
                    continue 'select_piece;
                } else {
                    println!("No puedes seleccionar una ficha del otro equipo.");
                    continue 'select_piece;
                }
            }

            'B' => {
                if turn == 'W' {
                    println!("No puedes seleccionar una ficha del otro equipo.");
                    continue 'select_piece;
                }

                for piece in &mut all_pieces[1] {
                    if piece.cords != cords_selected {
                        continue;
                    }

                    println!(
                        "Se ha seleccionado {:?} en la casilla {:?} ",
                        piece, cords_selected
                    );

                    piece.selected = true;
                    selected_piece = piece.clone();
                    break 'select_piece;
                }
                continue 'select_piece;
            }

            _ => {
                println!("No puedes seleccionar un cuadrado blanco.");
                continue 'select_piece;
            }
        }
    }

    return selected_piece;
}

/// Devuelve `true` si las cordenadas de una pieza + las\
/// cordenadas de el movimiento estan fuera del tablero
fn is_oob(selected_piece: &mut Piece, movement: [i8; 2]) -> bool {
    return (selected_piece.cords[0] as i8 + movement[0]) < 0
        || (selected_piece.cords[1] as i8 + movement[1]) < 0
        || (selected_piece.cords[0] as i8 + movement[0]) > 7
        || (selected_piece.cords[1] as i8 + movement[1]) > 7;
}

pub fn green_turn(game_board: &mut [[[char; 2]; 8]; 8], all_pieces: &mut Vec<Vec<Piece>>) {
    check_if_pawn_can_kill(all_pieces);
    println!("{}\n", format!("Green turn").green());

    render_board(&all_pieces);
    // Almacenamos la pieza seleccionada por el usuario
    let mut selected_piece = select_piece(all_pieces, game_board, 'W');

    'move_piece: loop {
        render_board(&all_pieces);

        // Le pedimos al usuario las cordenadas a donde quiere moverse
        println!("Introduce a donde moverte");

        let square_to_move = user_select_square();

        // Por cada movimiento posible de la pieza comprobamos si esta el que el usuario ha introducido
        for movement in selected_piece.clone().movements {
            // Comprobamos que el movimiento no queda fuera del tablero
            if is_oob(&mut selected_piece, movement) {
                continue;
            }

            let posible_movement = add_arrays_i8_2(selected_piece.cords as [i8; 2], movement);

            // Si el movimiento introducido por el usuario esta en la lista de movimientos de la pieza llamamos a la funcion que movera la pieza
            if square_to_move == posible_movement {
                if move_green_piece(
                    game_board,
                    all_pieces,
                    &mut selected_piece,
                    movement,
                    square_to_move,
                ) {
                    break 'move_piece;
                } else {
                    continue 'move_piece;
                }
            } else {
                continue;
            }
        }
        println!("No te puedes mover a esa posicion")
    }
}

pub fn blue_turn(game_board: &mut [[[char; 2]; 8]; 8], all_pieces: &mut Vec<Vec<Piece>>) {
    check_if_pawn_can_kill(all_pieces);
    println!("{}\n", format!("Blue turn").blue());

    render_board(&all_pieces);

    // Almacenamos la pieza seleccionada por el usuario
    let mut selected_piece = select_piece(all_pieces, game_board, 'B');

    'move_piece: loop {
        render_board(&all_pieces);

        // Le pedimos al usuario las cordenadas a donde quiere moverse
        println!("Introduce a donde moverte");

        let square_to_move = user_select_square();

        // Por cada movimiento posible de la pieza comprobamos si esta el que el usuario ha introducido
        for movement in selected_piece.clone().movements {
            // Comprobamos que el movimiento no queda fuera del tablero
            if is_oob(&mut selected_piece, movement) {
                continue;
            }

            let posible_movement = add_arrays_i8_2(selected_piece.cords as [i8; 2], movement);

            // Si el movimiento introducido por el usuario esta en la lista de movimientos de la pieza llamamos a la funcion que movera la pieza

            if square_to_move == posible_movement {
                if move_blue_piece(
                    game_board,
                    all_pieces,
                    &mut selected_piece,
                    movement,
                    square_to_move,
                ) {
                    break 'move_piece;
                } else {
                    continue 'move_piece;
                }
            } else {
                continue;
            }
        }
        println!("B");

        println!("No te puedes mover a esa posicion")
    }
}

fn square_from_cords(cords: [i8; 2], game_board: &mut [[[char; 2]; 8]; 8]) -> [char; 2] {
    game_board[cords[0] as usize][cords[1] as usize]
}

/// Pregunta al usuario una pregunta dada y devuelve un i8
fn user_input_i8(question: &str) -> i8 {
    let mut input_value = String::new();

    println!("{}", question);

    stdin()
        .read_line(&mut input_value)
        .expect("Ha ocurrido un error");

    return input_value.clone().trim().parse::<i8>().unwrap();
}

/// Pregunta al usuario una letra y la convierte\
/// en un i8.
fn user_input_leter_to_i8(question: &str) -> i8 {
    let mut input_value = String::new();

    println!("{}", question);

    stdin()
        .read_line(&mut input_value)
        .expect("Ha ocurrido un error");

    match input_value.clone().trim() {
        "A" => return 1,
        "B" => return 2,
        "C" => return 3,
        "D" => return 4,
        "E" => return 5,
        "F" => return 6,
        "G" => return 7,
        "H" => return 8,
        "a" => return 1,
        "b" => return 2,
        "c" => return 3,
        "d" => return 4,
        "e" => return 5,
        "f" => return 6,
        "g" => return 7,
        "h" => return 8,
        _ => return 0,
    };
}

fn user_select_square() -> [i8; 2] {
    loop {
        let col = user_input_leter_to_i8("Introduce una columna (A-H)") as i8;
        let row = user_input_i8("Introduce una fila (1-8)") as i8;

        if !((row < 9 && col < 9) && (row >= 1 && col >= 1)) {
            println!("No puedes seleccionar esa casilla, vuelve a introducir donde moverte");

            continue;
        }

        return [row - 1, col - 1];
    }
}

pub fn generate_tower_movements() -> Vec<[i8; 2]> {
    let mut movements: Vec<[i8; 2]> = vec![];
    for i in -7..=7 {
        movements.push([0, i]);
        movements.push([i, 0]);
    }
    return movements;
}

pub fn generate_bishop_movements() -> Vec<[i8; 2]> {
    let mut movements: Vec<[i8; 2]> = vec![];
    for i in -7..=7 {
        movements.push([i, i]);
        movements.push([i, i * -1])
    }
    return movements;
}

pub fn generate_queen_movements() -> Vec<[i8; 2]> {
    let mut movements: Vec<[i8; 2]> = vec![];
    for i in -7..=7 {
        movements.push([i, i]);
    }
    for i in -7..=7 {
        movements.push([0, i]);
        movements.push([i, 0]);
    }
    return movements;
}
