// square (0..64) -> (row, col)
pub fn decode_square(square: u64) -> (i64, i64) {
    ((square / 8) as i64, (square % 8) as i64)
}

pub fn encode_square(row: i64, col: i64) -> u64 {
    (row * 8 + col) as u64
}

pub fn position_mask(row: i64, col: i64) -> u64 {
    if row < 0 || col < 0 || row >= 8 || col >= 8 {
        0
    } else {
        1 << (row * 8 + col)
    }
}

pub fn position_valid(row: i64, col: i64) -> bool {
    row >= 0 && col >= 0 && row < 8 && col < 8
}

pub fn position_occupied(occupancy: u64, row: i64, col: i64) -> bool {
    !position_free(occupancy, row, col)
}

pub fn position_free(occupancy: u64, row: i64, col: i64) -> bool {
    // TODO: Remove this later
    if row < 0 || col < 0 || row >= 8 || col >= 8 {
        panic!("Invalid position");
    }

    (occupancy & position_mask(row, col)) == 0
}


// A1 -> 0, ..., H8 -> 63
pub fn encode_pos(pos: &str) -> u64 {
    let mut chars = pos.chars();
    let col: u64 = match chars.next().unwrap() {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        'D' => 3,
        'E' => 4,
        'F' => 5,
        'G' => 6,
        'H' => 7,
        _ => panic!("Invalid position '{}'", pos),
    };

    let row = chars.next().unwrap().to_digit(10).unwrap() - 1;

    (row as u64) * 8 + col
}

