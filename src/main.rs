// TODO: Remove this once we are done
#![allow(dead_code)]

use std::fmt;
use std::cmp;

mod bitscan;
use bitscan::*;

mod helper;
mod occupancy_masks;
mod movegen;
mod board;

use board::{Board, Color, Position, Move};

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

fn new_board(board: &Board, m: &Move) -> Board {
    // unimplemented!();
    // TODO: Implement board cloning
    let mut b = (*board).clone();
    b.make_move(m);
    b
}

pub fn perft(board: &Board, depth: i64) -> u64 {
    if depth == 0 {
        1
    } else {
        let mut count = 0;
        let moves = movegen::all_moves(board, board.turn);
        for m in moves.iter() {
            let b = new_board(board, m);
            count += perft(&b, depth - 1);
        }
        count
    }
}

fn main() {
    let board = Board::starting_position();
    println!("perft 1: {}", perft(&board, 3));

    // let moves = movegen::generate_pawn_moves_white(helper::encode_pos("E2"), occ, 0);
    // print_mask(moves);
    // for i in 0..64 {
    //     println!("0x{:016x},", generate_bishop_occupancy_mask(i));
    // }
    // let pos = encode_pos("D4");
    // let mask = generate_bishop_occupancy_mask(pos);
    // print_mask(mask);
}
