use occupancy_masks;
use helper::*;

// NOTE: This generates pseudo-legal moves, too
pub fn generate_rook_moves(square: u64, occupancy: u64) -> u64 {
    // Bitboard of target squares
    let mut moves = 0_u64;
    // let relevant_occupancy = occupancys & occupancys::rook[square];

    let (row, col) = decode_square(square);

    let mut x = col;
    while x < 7 {
        x += 1;
        let target = encode_square(row, x);
        // Mark the target square
        moves |= (1 << target);

        // If the move will capture a pice, add it and stop
        if (occupancy >> target) & 1 == 1 {
            break
        }
    }

    x = col;
    while x > 0 {
        x -= 1;
        let target = encode_square(row, x);
        moves |= (1 << target);
        if (occupancy >> target) & 1 == 1 {
            break
        }
    }

    let mut y = row;
    while y < 7 {
        y += 1;
        let target = encode_square(y, col);
        // Mark the target square
        moves |= (1 << target);

        // If the move will capture a pice, add it and stop
        if (occupancy >> target) & 1 == 1 {
            break
        }
    }

    y = row;
    while y > 0 {
        y -= 1;
        let target = encode_square(y, col);
        moves |= (1 << target);
        if (occupancy >> target) & 1 == 1 {
            break
        }
    }

    moves
}

pub fn generate_bishop_moves(square: u64, occupancy: u64) -> u64 {
    // Bitboard of target squares
    let mut moves = 0_u64;
    // let relevant_occupancy = occupancys & occupancys::rook[square];

    let (row, col) = decode_square(square);

    let mut x = col;
    let mut y = row;
    while x < 7 && y < 7 {
        x += 1;
        y += 1;
        let target = encode_square(y, x);
        // Mark the target square
        moves |= (1 << target);

        // If the move will capture a pice, add it and stop
        if (occupancy >> target) & 1 == 1 {
            break
        }
    }

    x = col;
    y = row;
    while x < 7 && y > 0 {
        x += 1;
        y -= 1;
        let target = encode_square(y, x);
        // Mark the target square
        moves |= (1 << target);

        // If the move will capture a pice, add it and stop
        if (occupancy >> target) & 1 == 1 {
            break
        }
    }

    x = col;
    y = row;
    while x > 0 && y < 7 {
        x -= 1;
        y += 1;
        let target = encode_square(y, x);
        // Mark the target square
        moves |= (1 << target);

        // If the move will capture a pice, add it and stop
        if (occupancy >> target) & 1 == 1 {
            break
        }
    }

    x = col;
    y = row;
    while x > 0 && y > 0 {
        x -= 1;
        y -= 1;
        let target = encode_square(y, x);
        // Mark the target square
        moves |= (1 << target);

        // If the move will capture a pice, add it and stop
        if (occupancy >> target) & 1 == 1 {
            break
        }
    }

    moves
}

pub fn generate_queen_moves(square: u64, occupancy: u64) -> u64 {
    let rook = generate_rook_moves(square, occupancy);
    let bishop = generate_bishop_moves(square, occupancy);

    rook | bishop
}

// FIDE rules:
// 3.7e. When a pawn reaches the rank furthest from its starting position it __must be exchanged__ as
//   part of the same move on the same square for a new queen, rook, bishop or knight of the same
//   colour. The player’s choice is __not restricted to pieces that have been captured previously__.
//   This exchange of a pawn for another piece is called ‘promotion’ and the effect of the new
//   piece is immediate.
pub fn generate_pawn_moves_white(square: u64, occupancy: u64, en_passant_square: u64) -> u64 {
    let mut moves = 0_u64;
    let (row, col) = decode_square(square);

    // Non-capture moves
    if position_free(occupancy, row + 1, col) {
        moves |= position_mask(row + 1, col);

        if row == 1 && position_free(occupancy, row + 2, col) {
            moves |= position_mask(row + 2, col);
        }
    }

    // Captures
    if position_valid(row + 1, col + 1) {
        let mask = position_mask(row + 1, col + 1);
        if position_occupied(occupancy, row + 1, col + 1) || mask == en_passant_square {
            moves |= mask;
        }
    }

    if position_valid(row + 1, col - 1) {
        let mask = position_mask(row + 1, col - 1);
        if  position_occupied(occupancy, row + 1, col - 1) || mask == en_passant_square {
            moves |= mask;
        }
    }

    moves
}

pub fn generate_pawn_moves_black(square: u64, occupancy: u64, en_passant_square: u64) -> u64 {
    let mut moves = 0_u64;
    let (row, col) = decode_square(square);

    // Non-capture moves
    if position_free(occupancy, row - 1, col) {
        moves |= position_mask(row - 1, col);

        if row == 6 && position_free(occupancy, row - 2, col) {
            moves |= position_mask(row - 2, col);
        }
    }

    // Captures
    if position_valid(row - 1, col + 1) {
        let mask = position_mask(row - 1, col + 1);
        if position_occupied(occupancy, row - 1, col + 1) || mask == en_passant_square {
            moves |= mask;
        }
    }

    if position_valid(row - 1, col - 1) {
        let mask = position_mask(row - 1, col - 1);
        if  position_occupied(occupancy, row - 1, col - 1) || mask == en_passant_square {
            moves |= mask;
        }
    }

    moves
}

pub fn generate_knight_moves(square: u64, occupancy: u64) -> u64 {
    let mut moves: u64 = 0;
    let (row, col) = decode_square(square);

    moves |= position_mask(row + 2, col + 1);
    moves |= position_mask(row + 2, col - 1);
    moves |= position_mask(row - 2, col + 1);
    moves |= position_mask(row - 2, col - 1);

    moves |= position_mask(row + 1, col + 2);
    moves |= position_mask(row + 1, col - 2);
    moves |= position_mask(row - 1, col + 2);
    moves |= position_mask(row - 1, col - 2);

    moves
}

// TODO: Handle castling
pub fn generate_king_moves(square: u64, occupancy: u64) -> u64 {
    let mut moves: u64 = 0;
    let (row, col) = decode_square(square);

    moves |= position_mask(row + 1, col);
    moves |= position_mask(row - 1, col);
    moves |= position_mask(row, col + 1);
    moves |= position_mask(row, col - 1);

    moves |= position_mask(row + 1, col + 1);
    moves |= position_mask(row + 1, col - 1);
    moves |= position_mask(row - 1, col + 1);
    moves |= position_mask(row - 1, col - 1);

    moves
}
