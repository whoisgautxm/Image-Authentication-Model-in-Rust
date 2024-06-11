extern crate image;
#[macro_use]
extern crate lazy_static;

use image::{GenericImageView, ImageBuffer, RgbImage, Rgba,open};
use std::sync::Mutex;

// Define a global mutable variable using lazy_static and Mutex for thread safety
lazy_static! {
    static ref MSB_IMG: Mutex<Option<RgbImage>> = Mutex::new(None);
}

pub fn extract_msb() {
    // Load the image from a file
    let img = image::open("../rdr.png").expect("Failed to open image");

    // Get the dimensions of the image
    let (width, height) = img.dimensions();

    // Create a new image buffer to store the MSB extracted image
    let mut msb_img: RgbImage = ImageBuffer::new(width, height);

    // Iterate over each pixel
    for (x, y, pixel) in img.pixels() {
        // Extract the color channels (assuming the image is in RGB format)
        let channels = pixel.0;

        // Extract the MSB of each color channel
        let r_msb = (channels[0]  >> 7)& 0x01;
        let g_msb = (channels[1]  >> 7)& 0x01;
        let b_msb = (channels[2]  >> 7)& 0x01;
        // Print out the channel values for debugging
        println!("Pixel at ({}, {}): R={}, G={}, B={}", x, y, r_msb, g_msb, b_msb);

        // Create a new pixel with the MSBs (scaled up to 255 for visibility)
        let msb_pixel = Rgba([r_msb * 255, g_msb * 255, b_msb * 255, 255]);

        println!("{:?}",msb_pixel);

        // Put the new pixel into the MSB image buffer
        msb_img.put_pixel(x, y, image::Rgb([msb_pixel.0[0], msb_pixel.0[1], msb_pixel.0[2]]));
    }

    // Save the msb_img to the global variable
    {
        let mut global_msb_img = MSB_IMG.lock().unwrap();
        *global_msb_img = Some(msb_img);
    }

    // Optionally save the MSB extracted image to a file
    {
        let global_msb_img = MSB_IMG.lock().unwrap();
        if let Some(ref img) = *global_msb_img {
            img.save("../msb_image1.png").expect("Failed to save image");
        }
    }
}