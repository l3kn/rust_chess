use occupancy_masks;
use helper::{decode_square, encode_square};

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

// TODO: A move must encode from, to, attack?, promotion_to
// TODO: Handle promotion
// TODO: Handle en-passant
// TODO: Pawns beat on diagonal
//
// FIDE rules:
// 3.7e. When a pawn reaches the rank furthest from its starting position it __must be exchanged__ as
//   part of the same move on the same square for a new queen, rook, bishop or knight of the same
//   colour. The player’s choice is __not restricted to pieces that have been captured previously__.
//   This exchange of a pawn for another piece is called ‘promotion’ and the effect of the new
//   piece is immediate.
pub fn generate_pawn_moves_white(square: u64, occupancy: u64) -> u64 {
    let mut moves = 0_u64;
    let (row, col) = decode_square(square);

    match row {
        0 => panic!("A white pawn can never be on rank 1 because it can't move back"),
        1 => {
            // If a white pawn is on rank 2, it can move one or two squares
        },
        2...5 {
            // If a white pawn is on rank 3 to 6, it can move one square
        },
        6 => {
            // TODO: Handle promotion
        }
        7 => panic!("A white pawn can never be on rank 7 because it has to promote")
    }

    moves
}

