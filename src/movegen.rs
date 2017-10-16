use board::{Board, Color, Position, Direction, Move};

pub fn shoot_ray(pos_: &Position, dir: Direction, board: &Board, color: Color) -> Vec<Move> {
    let mut pos = pos_.clone();
    let mut result: Vec<Move> = Vec::new();

    loop {
        pos = Position::new(pos.file + dir.0, pos.rank + dir.1);

        // TODO: Check occupation
        if pos.is_valid() {
            let piece = board.get_pos(&pos);
            if let Some(p) = *piece {
                if p.color != color {
                    result.push(Move::normal(pos_, &pos));
                }
                break;
            } else {
                result.push(Move::normal(pos_, &pos));
            }
        } else {
            break;
        }
    }

    result
}

pub fn rook_moves(pos: &Position, board: &Board, color: Color) -> Vec<Move> {
    let mut result: Vec<Move> = Vec::new();

    result.extend(shoot_ray(pos, Direction( 1, 0), board, color));
    result.extend(shoot_ray(pos, Direction(-1, 0), board, color));
    result.extend(shoot_ray(pos, Direction(0,  1), board, color));
    result.extend(shoot_ray(pos, Direction(0, -1), board, color));

    result
}

pub fn bishop_moves(pos: &Position, board: &Board, color: Color) -> Vec<Move> {
    let mut result: Vec<Move> = Vec::new();

    result.extend(shoot_ray(pos, Direction( 1,  1), board, color));
    result.extend(shoot_ray(pos, Direction( 1, -1), board, color));
    result.extend(shoot_ray(pos, Direction(-1,  1), board, color));
    result.extend(shoot_ray(pos, Direction(-1, -1), board, color));

    result
}

pub fn queen_moves(pos: &Position, board: &Board, color: Color) -> Vec<Move> {
    let mut result: Vec<Move> = Vec::new();

    result.extend(rook_moves(pos, board, color));
    result.extend(bishop_moves(pos, board, color));

    result
}

static KNIGHT_DIRS: &'static[Direction] = &[
    Direction( 2,  1),
    Direction( 2, -1),
    Direction(-2,  1),
    Direction(-2, -1),
    Direction( 1,  2),
    Direction( 1, -2),
    Direction(-1,  2),
    Direction(-1, -2),
];

static KING_DIRS: &'static[Direction] = &[
    Direction( 0, -1),
    Direction( 0,  1),
    Direction( 1,  0),
    Direction(-1,  0),
    Direction(-1, -1),
    Direction(-1,  1),
    Direction( 1, -1),
    Direction( 1,  1),
];

pub fn is_valid_move(to: &Position, board: &Board, color: Color) -> bool {
    if let Some(piece) = *board.get_pos(&to) {
        piece.color != color
    } else {
        true
    }
}

pub fn knight_moves(pos: &Position, board: &Board, color: Color) -> Vec<Move> {
    let result: Vec<Move> =
        KNIGHT_DIRS
            .iter()
            .map(|dir| Position::new(pos.file + dir.0, pos.rank + dir.1))
            .filter(|p| p.is_valid())
            .filter(|p| is_valid_move(p, board, color))
            .map(|p| Move::normal(pos, &p))
            .collect();
    result
}

pub fn king_moves(pos: &Position, board: &Board, color: Color) -> Vec<Move> {
    let result: Vec<Move> =
        KING_DIRS
            .iter()
            .map(|dir| Position::new(pos.file + dir.0, pos.rank + dir.1))
            .filter(|p| p.is_valid())
            .filter(|p| is_valid_move(p, board, color))
            .map(|p| Move::normal(pos, &p))
            .collect();
    result
}

// White pawns starts at rank 1 and move towards rank 7
// If they are in rank 1, the can move 2 steps foreward (up)
// Black pawns start at rank 6 and move towards rank 0
// If they are in rank 6, the can move 2 steps foreward (down)
pub fn pawn_moves(pos: &Position, board: &Board, color: Color) -> Vec<Move> {
    if color == Color::White {
        white_pawn_moves(pos, board)
    } else {
        black_pawn_moves(pos, board)
    }
}

pub fn white_pawn_moves(pos: &Position, board: &Board) -> Vec<Move> {
    let mut result: Vec<Move> = Vec::new();

    let one_step = Position::new(pos.file, pos.rank + 1);
    let two_step = Position::new(pos.file, pos.rank + 2);
    if pos.rank == 1 {
        if board.get_pos(&one_step).is_none() {
            result.push(Move::normal(pos, &one_step));
        }
        if board.get_pos(&two_step).is_none() {
            result.push(Move::normal(pos, &two_step));
        }
    } else {
        // TODO: is_valid() is not really necessary,
        // because of pawn promotions there can never be a pawn
        // in the first or last rank
        if one_step.is_valid() && board.get_pos(&one_step).is_none() {
            result.push(Move::normal(pos, &one_step));
        }
    }

    let capture_left = Position::new(pos.file - 1, pos.rank + 1);
    let capture_right = Position::new(pos.file + 1, pos.rank + 1);

    // TODO: Maybe let get_pos return some special value
    // if the position is not valid
    if capture_left.is_valid() {
        if let Some(piece) = *board.get_pos(&capture_left) {
            if piece.color == Color::Black {
                result.push(Move::normal(pos, &capture_left));
            }
        } else if Some(capture_left) == board.en_passant_black {
            result.push(Move::normal(pos, &capture_left));
        }
    }
    if capture_right.is_valid() {
        if let Some(piece) = *board.get_pos(&capture_right) {
            if piece.color == Color::Black {
                result.push(Move::normal(pos, &capture_right));
            }
        } else if Some(capture_right) == board.en_passant_black {
            result.push(Move::normal(pos, &capture_right));
        }
    }

    // TODO: Handle promotions
    result
}

pub fn black_pawn_moves(pos: &Position, board: &Board) -> Vec<Move> {
    let mut result: Vec<Move> = Vec::new();

    let one_step = Position::new(pos.file, pos.rank - 1);
    let two_step = Position::new(pos.file, pos.rank - 2);
    if pos.rank == 6 {
        if board.get_pos(&one_step).is_none() {
            result.push(Move::normal(pos, &one_step));
        }
        if board.get_pos(&two_step).is_none() {
            result.push(Move::normal(pos, &two_step));
        }
    } else {
        // TODO: is_valid() is not really necessary,
        // because of pawn promotions there can never be a pawn
        // in the first or last rank
        if one_step.is_valid() && board.get_pos(&one_step).is_none() {
            result.push(Move::normal(pos, &one_step));
        }
    }

    let capture_left = Position::new(pos.file - 1, pos.rank - 1);
    let capture_right = Position::new(pos.file + 1, pos.rank - 1);

    // TODO: Maybe let get_pos return some special value
    // if the position is not valid
    if capture_left.is_valid() {
        if let Some(piece) = *board.get_pos(&capture_left) {
            if piece.color == Color::White {
                result.push(Move::normal(pos, &capture_left));
            }
        } else if Some(capture_left) == board.en_passant_white {
            result.push(Move::normal(pos, &capture_left));
        }
    }
    if capture_right.is_valid() {
        if let Some(piece) = *board.get_pos(&capture_right) {
            if piece.color == Color::White {
                result.push(Move::normal(pos, &capture_right));
            }
        } else if Some(capture_right) == board.en_passant_white {
            result.push(Move::normal(pos, &capture_right));
        }
    }

    // TODO: Handle promotions
    result
}

use board::Type;

pub fn all_moves(board: &Board, color: Color) -> Vec<Move> {
    let mut result: Vec<Move> = Vec::new();

    for row in 0..8 {
        for col in 0..8 {
            let pos = Position::new(col, row);
            if let Some(piece) = *board.get_pos(&pos) {
                if piece.color == color {
                    match piece.kind {
                        Type::Rook => {
                            result.extend(rook_moves(&pos, board, color));
                        },
                        Type::Bishop => {
                            result.extend(bishop_moves(&pos, board, color));
                        },
                        Type::Knight => {
                            result.extend(knight_moves(&pos, board, color));
                        },
                        Type::King => {
                            result.extend(king_moves(&pos, board, color));
                        },
                        Type::Queen => {
                            result.extend(queen_moves(&pos, board, color));
                        },
                        Type::Pawn => {
                            result.extend(pawn_moves(&pos, board, color));
                        },
                    }
                }
            }
        }
    }

    result
}
