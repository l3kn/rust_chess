// Generate bitscan magics for 64 bit ints

fn hash(a: u64, b: u64, s: u64) -> u64 {
    ((a.wrapping_mul(b)) >> (64 - s))
}

pub fn find_bitscan_magic() {
    let mut magic: u64 = 0;
    let mut indices: Vec<u64> = Vec::new();
    let mut pointer = 0;

    loop {
        let v = hash(1 << (63 - pointer), magic, 6);
        println!("Indices: {:?}", indices);
        println!("Magic: 0b{:x}, Pointer: {}", magic, pointer);
        if indices.contains(&v) {
            println!("Collision at {}", pointer);
            let (new_magic, new_pointer) = wrong(magic, pointer);
            magic = new_magic;
            
            indices = Vec::new();
            for i in 0..new_pointer {
                indices.push(hash(1 << (63 - i), magic, 6));
            }
            pointer = new_pointer;
        } else {
            indices.push(v);
            if pointer == 63 {
                break
            } else {
                pointer += 1;
            }
        }
    }
}

pub fn generate_db(bitscan_magic: u64) {
    let mut db = [0; 64];

    for i in 0..64 {
        let h = hash(1 << i, bitscan_magic, 6);
        db[h as usize] = i;
    }

    for e in db.iter() {
        print!("{}, ", e);
    }
}

// Return: new magic & pointer
fn wrong(magic : u64, pointer : u64) -> (u64, u64) {
    // Check if the `pointer`th bit is 1 or 0
    if (magic >> pointer) & 1 == 0 {
        // If it is 0, just toggle it to 1
        (magic ^ (1 << pointer), pointer)
    } else {
        // If it is 1 and pointer == 0, there is no solution,
        // otherwisrust binary literale call wrong() recursively
        // with magic[pointer] = 0 and pointer - 1
        if pointer == 0 {
            panic!("No solution found");
        } else {
            wrong(magic ^ (1 << pointer), pointer - 1)
        }
    }
}

#[test]
fn wrong_test() {
    assert_eq!(wrong(0b0, 0), (0b1, 0));
    assert_eq!(wrong(0b10, 1), (0b1, 0));
    assert_eq!(wrong(0b110, 2), (0b1, 0));
    assert_eq!(wrong(0b100, 2), (0b10, 1));
    assert_eq!(wrong(0b000, 2), (0b100, 2));
}
