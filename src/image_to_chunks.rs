extern crate image;

use image::{ImageBuffer, Rgba};

pub fn slice_image_into_blocks(image: &ImageBuffer<Rgba<u8>, Vec<u8>>, block_size: u32) -> Vec<ImageBuffer<Rgba<u8>, Vec<u8>>> {
    let (width, height) = image.dimensions();
    let mut blocks = Vec::new();

    for y in (0..height).step_by(block_size as usize) {
        for x in (0..width).step_by(block_size as usize) {
            let mut block = ImageBuffer::new(block_size, block_size);

            for by in 0..block_size {
                for bx in 0..block_size {
                    if x + bx < width && y + by < height {
                        let pixel = image.get_pixel(x + bx, y + by);
                        block.put_pixel(bx, by, *pixel);
                    }
                }
            }

            blocks.push(block);
        }
    }

    blocks
}

pub fn save_blocks(blocks:Vec<ImageBuffer<Rgba<u8>, Vec<u8>>>,prefix:&str){
    for (i,block) in blocks.iter().enumerate(){
        block.save(format!("{}{}.jpg",prefix,i)).unwrap();
    }
}
