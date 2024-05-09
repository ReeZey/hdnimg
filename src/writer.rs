use std::path::PathBuf;

use anyhow::{Error, Ok};

use bitvec::{field::BitField, order::Lsb0, slice::BitSlice, vec::BitVec};
use image::{io::Reader as ImageReader, RgbaImage};
use crate::utils::clear_bits;

pub fn encode(input: Vec<u8>, input_image: &PathBuf, output_image: &PathBuf, bit_count: &usize) -> anyhow::Result<(), Error> {
    let mut img: RgbaImage = ImageReader::open(input_image)?.decode()?.into_rgba8();

    let image_width = img.width();
    let image_height = img.height();

    let total_bytes = image_height * image_width * 3 * *bit_count as u32;
    
    if input.len() > total_bytes as usize {
        return Err(Error::msg(format!("input is larger than image, higher image resolution or bit_count required [{} input buffer > {} image-dimentions * bit_depth]", input.len(), total_bytes)));
    }
    
    println!("size used: {}/{}", input.len(), total_bytes);

    let mut buffer = vec![];
    buffer.extend((input.len() as u32).to_le_bytes());
    buffer.extend(input);

    let bitbuffer: BitVec<u8, Lsb0> = BitVec::from_vec(buffer);
    let mut chunks = bitbuffer.chunks(3 * bit_count);
    
    for pixel in img.pixels_mut() {
        if chunks.len() == 0 {
            break;
        }
        
        let mut chunk = chunks.next().expect("this should never happen (writer code 1)").chunks(*bit_count);

        // this could be rewritten, this was the easiest way to do it

        pixel[0] = clear_bits(pixel[0], *bit_count as u8);
        pixel[0] |= chunk.next().unwrap_or(BitSlice::from_element(&1)).load::<u8>();

        pixel[1] = clear_bits(pixel[1], *bit_count as u8);
        pixel[1] |= chunk.next().unwrap_or(BitSlice::from_element(&1)).load::<u8>();

        pixel[2] = clear_bits(pixel[2], *bit_count as u8);
        pixel[2] |= chunk.next().unwrap_or(BitSlice::from_element(&1)).load::<u8>();
    }

    img.save(output_image)?;

    Ok(())
}