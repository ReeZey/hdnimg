use bitvec::{field::BitField, prelude::Lsb0, slice::BitSlice, vec::BitVec};
use image::{io::Reader as ImageReader, RgbaImage};

const BIT_COUNT: usize = 1;

fn main() {
    println!("brrr");
    let mut img: RgbaImage = ImageReader::open("input.png").unwrap().decode().unwrap().into_rgba8();

    let str_input = "test";

    let mut input_buffer: Vec<u8> = vec![];
    input_buffer.extend(str_input.bytes());

    let mut buffer = vec![];
    buffer.extend((input_buffer.len() as u32).to_le_bytes());
    buffer.extend(input_buffer);

    let bitbuffer: BitVec<u8, Lsb0> = BitVec::from_vec(buffer);
    
    let image_width = img.width();
    let image_height = img.height();

    let total_bytes = image_height * image_width * 3 * BIT_COUNT as u32;

    println!("{}", total_bytes);

    let mut chunks = bitbuffer.chunks(3 * BIT_COUNT);

    println!("chunks: {}", chunks.len());

    for pixel in img.pixels_mut() {
        if chunks.len() == 0 {
            break;
        }

        let mut chunk = chunks.next().unwrap().chunks(BIT_COUNT);

        let r = chunk.next().unwrap_or(BitSlice::from_element(&0)).load::<u8>();
        pixel[0] >>= 8 - r.leading_zeros();
        pixel[0] <<= (8 - r.leading_zeros()) as u8 | r;

        let g = chunk.next().unwrap_or(BitSlice::from_element(&0)).load::<u8>();
        pixel[0] >>= 8 - g.leading_zeros();
        pixel[0] <<= (8 - g.leading_zeros()) as u8 | g;

        let b = chunk.next().unwrap_or(BitSlice::from_element(&0)).load::<u8>();
        pixel[0] >>= 8 - b.leading_zeros();
        pixel[0] <<= (8 - b.leading_zeros()) as u8 | b;
    }

    img.save("output.png").unwrap();

    drop(img);

    let img: RgbaImage = ImageReader::open("output.png").unwrap().decode().unwrap().into_rgba8();
    
    let mut bitvec: BitVec<u8, Lsb0> = BitVec::new();

    for pixel in img.pixels() {
        let mut r = pixel[0] << 8 - BIT_COUNT;
        r = r >> 8 - BIT_COUNT;
        bitvec.extend(BitSlice::<u8, Lsb0>::from_element(&r));

        let mut g = pixel[0] << 8 - BIT_COUNT;
        g = g >> 8 - BIT_COUNT;
        bitvec.extend(BitSlice::<u8, Lsb0>::from_element(&g));

        let mut b = pixel[0] << 8 - BIT_COUNT;
        b = b >> 8 - BIT_COUNT;
        bitvec.extend(BitSlice::<u8, Lsb0>::from_element(&b));
    }

    println!("{:?}", bitvec[0..32].to_bitvec());

    let len = bitvec[0..32].load::<u32>();

    println!("{:?}", len);

    println!("don");
}
