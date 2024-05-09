pub fn clear_bits(mut num: u8, amount: u8) -> u8 {
    for idx in 0..amount {
        num &= !(1 << idx)
    }
    return num;
}

pub fn get_bits(num: u8, amount: u8) -> u8 {
    let mask = 255 >> (8 - amount);

    return num & mask;
}