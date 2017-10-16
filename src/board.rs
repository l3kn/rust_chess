use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    King,
    Queen,
    Rook,
    Bishop,
    Pawn,
    Knight,
}
use self::Type::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Black,
    White,
}
use self::Color::*;

#[derive(Debug)]
pub struct Direction(pub i64, pub i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub rank: i64,
    pub file: i64
}

// Algebraic chess notation: A1 ... H8
//   files: columns, a - h, here 0...8
//   ranks: rows, 1 - 8, here 0...8
impl Position {
    pub fn new(file: i64, rank: i64) -> Self {
        Self{file, rank}
    }

    pub fn is_valid(&self) -> bool {
        self.file <= 7 && self.file >= 0 &&
        self.rank <= 7 && self.rank >= 0
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let file = (b'A' + (self.file as u8)) as char;
        write!(f, "{}{}", file, self.rank + 1)
    }
}

#[derive(Debug, Clone)]
pub struct Move {
    pub from: Position,
    pub to: Position,
    pub promotion: Option<Type>,
    // Used to update the square that can be attacked en passant
    // or the piece that will be captured en passant
    // pub en_passant_square: Option<Position>,
    // pub en_passant_capture: Option<Position>
}

impl Move {
    pub fn normal(from: &Position, to: &Position) -> Self {
        Self {
            from: from.clone(),
            to: to.clone(),
            promotion: None,
            // en_passant_square: None,
            // en_passant_capture: None
        }
    }

    // pub fn promotion(from: &Position, to: &Position, kind: Type) -> Self {
    //     Self {
    //         from: from.clone(),
    //         to: to.clone(),
    //         promotion: Some(kind),
    //         en_passant_square: None,
    //         en_passant_capture: None
    //     }
    // }

    // // Here en passant referes to a move that __creates__ an en passant square
    // pub fn en_passant(from: &Position, to: &Position, ep: &Position) -> Self {
    //     Self {
    //         from: from.clone(),
    //         to: to.clone(),
    //         promotion: None,
    //         en_passant_square: ep,
    //         en_passant_capture: None
    //     }
    // }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.promotion {
            Some(kind) => {
                write!(f, "{} -> {} (promote to {:?})", self.from, self.to, kind)
            },
            None => {
                write!(f, "{} -> {}", self.from, self.to)
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Piece {
    pub kind: Type,
    pub color: Color
}

impl Piece {
    fn new(kind: Type, color: Color) -> Self {
        Self{kind, color}
    }

    fn from_fen(fen: char) -> Self {
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
            Self::new(t, White)
        } else {
            Self::new(t, Black)
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ch = match self.kind {
            King => 'k',
            Queen => 'q',
            Rook => 'r',
            Bishop => 'b',
            Knight => 'n',
            Pawn => 'p',
        };

        if self.color == Black {
            write!(f, "{}", ch)
        } else {
            write!(f, "{}", ch.to_uppercase())
        }
    }
}

#[derive(Copy)]
pub struct Board {
    pieces: [Option<Piece>; 64],
    pub turn: Color,
    pub en_passant_white: Option<Position>,
    pub en_passant_black: Option<Position>,
}

impl Board {
    pub fn empty() -> Board {
        Board {
            pieces: [None; 64],
            turn: Color::White,
            en_passant_white: None,
            en_passant_black: None,
        }
    }

    pub fn from_fen(fen: &str) -> Board {
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
                    board.set(col, 7 - row, Some(piece));
                    col += 1;
                }
            }
        }

        board
    }

    pub fn starting_position() -> Board {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1)";
        Self::from_fen(fen)
    }

    pub fn get_pos(&self, pos: &Position) -> &Option<Piece> {
        self.get(pos.file as usize, pos.rank as usize)
    }

    pub fn set_pos(&mut self, pos: &Position, piece: Option<Piece>) {
        self.set(pos.file as usize, pos.rank as usize, piece);
    }

    pub fn get(&self, col: usize, row: usize) -> &Option<Piece> {
        self.pieces.get(row * 8 + col).unwrap()
    }

    pub fn set(&mut self, col: usize, row: usize, piece: Option<Piece>) {
        self.pieces[row * 8 + col] = piece;
    }

    pub fn make_move(&mut self, m: &Move) {
        match *self.get_pos(&m.from) {
            Some(piece) => {
                self.set_pos(&m.to, Some(piece));
                self.set_pos(&m.from, None);

                // Handle moves that set en passant squares
                if piece.color == Color::White {
                    if piece.kind == Type::Pawn && m.from.rank == 1 && m.to.rank == 3 {
                        self.en_passant_white = Some(Position::new(m.from.file, 2));
                    } else {
                        self.en_passant_white = None;
                    }
                } else {
                    if piece.kind == Type::Pawn && m.from.rank == 6 && m.to.rank == 4 {
                        self.en_passant_black = Some(Position::new(m.from.file, 2));
                    } else {
                        self.en_passant_black = None;
                    }
                }

                // Handle moves that use en passant squares
                if piece.color == Color::White {
                    if piece.kind == Type::Pawn && Some(m.to) == self.en_passant_black {
                        let beaten = Position::new(m.to.file, m.to.rank + 1);
                        self.set_pos(&beaten, None);
                    }
                } else {
                    if piece.kind == Type::Pawn && Some(m.to) == self.en_passant_white {
                        let beaten = Position::new(m.to.file, m.to.rank - 1);
                        self.set_pos(&beaten, None);
                    }
                }

            },
            None => {
                panic!("Can't make move {}, there is no piece at {}", m, m.from);
            }
        }
        if self.turn == Color::White {
            self.turn = Color::Black;
        } else {
            self.turn = Color::White;
        }
    }

    pub fn occupancy(&self) -> u64 {
        let mut mask = 0_u64;
        for i in 0..64 {
            if self.pieces[i].is_some() {
                mask |= 1 << i;
            }
        }
        mask
    }
}

impl Clone for Board {
    fn clone(&self) -> Self { *self }
}

// Unicode Box Characters copied from
// <https://en.wikipedia.org/wiki/Box-drawing_character>
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res = String::new();
        res.push_str("\n ╔════════╗\n");
        for row in 0..8 {
            res += &format!("{}║", 8 - row)[..];
            for col in 0..8 {
                // Flip the board so that A1 (0, 0) is the lower left corner
                match *self.get(col, 7 - row) {
                    Some(p) => res.push_str(&p.to_string()),
                    None => res.push_str(" "),
                }
            }
            res.push_str("║\n");
        }
        res.push_str(" ╚════════╝\n");
        res.push_str("  ABCDEFGH");

        write!(f, "Board {}", res)
    }
}

