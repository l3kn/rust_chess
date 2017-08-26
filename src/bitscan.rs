// 64-bit magic bitscan

const bitscan_magic: u64 = 0x7edd5e59a4e28c2;
const bitscan_db: [u64; 64] = [
    63, 0, 58, 1, 59, 47, 53, 2, 60, 39, 48, 27, 54, 33, 42, 3,
    61, 51, 37, 40, 49, 18, 28, 20, 55, 30, 34, 11, 43, 14, 22, 4,
    62, 57, 46, 52, 38, 26, 32, 41, 50, 36, 17, 19, 29, 10, 13, 21,
    56, 45, 25, 31, 35, 16, 9, 12, 44, 24, 15, 8, 23, 7, 6, 5,
];

pub fn bitscan(i: u64) -> u64 {
    let index = i.wrapping_mul(bitscan_magic) >> (64 - 6);
    bitscan_db[index as usize]
}

#[test]
fn bitscan_test() {
    for i in 0..64 {
        assert_eq!(bitscan(1 << i), i);
    }
}
