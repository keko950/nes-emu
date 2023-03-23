pub mod emu;

use crate::emu::cpu;


fn main() {
    let mut i : u8;
    let mut j : u8;
    i = 60;
    j = 50;
    
    let result = (i >> 1) + (j >> 1);
    let last_bit_i = i & 0b1000_0000;
    let last_bit_j = j & 0b1000_0000;
    println!("{}", ((result >> 7) + ((last_bit_i | last_bit_j) >> 7)) == 2);
}
