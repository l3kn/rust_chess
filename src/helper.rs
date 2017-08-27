// square (0..64) -> (row, col)
pub fn decode_square(square: u64) -> (u64, u64) {
    (square / 8, square % 8)
}

pub fn encode_square(row: u64, col: u64) -> u64 {
    row * 8 + col
}

// A1 -> 0, ..., H8 -> 63
pub fn encode_pos(pos: &str) -> u64 {
    let mut chars = pos.chars();
    let col = match chars.next().unwrap() {
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

    row * 8 + (col as u64)
}

