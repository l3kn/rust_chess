use std::fmt;
use std::cmp;

mod bitscan;
use bitscan::*;

mod helper;
mod occupancy_masks;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Type {
    King,
    Queen,
    Rook,
    Bishop,
    Pawn,
    Knight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    Black,
    White,
}

use Color::*;
use Type::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Piece(Type, Color);

impl Piece {
    fn from_fen(fen: char) -> Piece {
        let t = match fen {
            'k' | 'K' => King,
            'q' | 'Q' => Queen,
            'r' | 'R' => Rook,
            'b' | 'B' => Bishop,
            'n' | 'N' => Knight,
            'p' | 'P' => Pawn,
            _ => panic!("Not a valid fen char: '{}'", fen),
        };

        if fen.is_uppercase() {
            Piece(t, White)
        } else {
            Piece(t, Black)
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ch = match self.0 {
            King => 'k',
            Queen => 'q',
            Rook => 'r',
            Bishop => 'b',
            Knight => 'n',
            Pawn => 'p',
        };

        if self.1 == Black {
            write!(f, "{}", ch)
        } else {
            write!(f, "{}", ch.to_uppercase())
        }
    }
}

struct Board {
    pieces: [Option<Piece>; 64],
}

impl Board {
    fn empty() -> Board {
        Board { pieces: [None; 64] }
    }

    fn from_fen(fen: &str) -> Board {
        let mut board = Board::empty();
        let mut row: usize = 0;
        let mut col: usize = 0;

        for c in fen.chars() {
            match c {
                '/' => {
                    row += 1;
                    col = 0;
                }
                '0'...'8' => {
                    let offset = c.to_digit(10).unwrap() as usize;
                    col += offset;
                }
                ' ' => return board,
                other => {
                    let piece = Piece::from_fen(other);
                    board.set(7 - row, col, Some(piece));
                    col += 1;
                }
            }
        }

        board
    }

    fn starting_position() -> Board {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1)";
        Self::from_fen(fen)
    }

    fn get(&self, row: usize, col: usize) -> &Option<Piece> {
        self.pieces.get(row * 8 + col).unwrap()
    }

    fn set(&mut self, row: usize, col: usize, piece: Option<Piece>) {
        self.pieces[row * 8 + col] = piece;
    }
}

// Unicode Box Characters copied from
// <https://en.wikipedia.org/wiki/Box-drawing_character>
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res = String::new();
        res.push_str("\n╔════════╗\n");
        for row in 0..8 {
            res.push_str("║");
            for col in 0..8 {
                // Flip the board so that (0, 0) is the lower left corner
                match *self.get(7 - row, col) {
                    Some(p) => res.push_str(&p.to_string()),
                    None => res.push_str(" "),
                }
            }
            res.push_str("║\n");
        }
        res.push_str("╚════════╝");

        write!(f, "Board {}", res)
    }
}

fn print_mask(mask: u64) {
    // A1 (bottom left, from whites perspective) is the LSB,
    // so we need to reverse both x and y directions
    for i in (0..8).rev() {
        let row = (mask >> (8 * i)) & 0xff;

        // See: http://graphics.stanford.edu/~seander/bithacks.html
        let row_rev = (row * 0x0202020202 & 0x010884422010) % 1023;
        println!("{:08b}", row_rev);
    }
}


fn main() {
    // for i in 0..64 {
    //     println!("0x{:016x},", generate_bishop_occupancy_mask(i));
    // }
    // let pos = encode_pos("D4");
    // let mask = generate_bishop_occupancy_mask(pos);
    // print_mask(mask);
}
