use std::path::PathBuf;

use anyhow::{Error, Ok};

use bitvec::{field::BitField, order::Lsb0, vec::BitVec, view::BitViewSized};
use image::{io::Reader as ImageReader, RgbaImage};

use crate::utils::get_bits;

pub fn decode(input_file: &PathBuf, bit_count: &usize) -> anyhow::Result<Vec<u8>, Error> {
    let img: RgbaImage = ImageReader::open(input_file)?.decode()?.into_rgba8();

    let mut bit_vec: BitVec<u8, Lsb0> = BitVec::new();

    for pixel in img.pixels() {
        //this is shit but it works? idk
        bit_vec.extend(&get_bits(pixel[0], *bit_count as u8).into_bitarray::<Lsb0>()[0..*bit_count]);
        bit_vec.extend(&get_bits(pixel[1], *bit_count as u8).into_bitarray::<Lsb0>()[0..*bit_count]);
        bit_vec.extend(&get_bits(pixel[2], *bit_count as u8).into_bitarray::<Lsb0>()[0..*bit_count]);
    }
    
    let len = bit_vec[0..32].load::<u32>() as usize;

    let mut buffer = vec![];
    for idx in 0..len {
        buffer.push(bit_vec[32+(8*idx)..32 + 8 + (8*idx)].load::<u8>());
    }
    Ok(buffer)
}