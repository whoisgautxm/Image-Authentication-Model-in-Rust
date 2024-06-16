extern crate image;

use image::{GenericImageView, ImageBuffer, Rgba};

pub fn extract_msb(img_path: &str) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    // Load the image from a file
    let img = image::open(img_path).expect("Failed to open image");

    // Get the dimensions of the image
    let (width, height) = img.dimensions();

    // Create a new image buffer to store the MSB extracted image
    let mut msb_img = ImageBuffer::new(width, height);

    // Iterate over each pixel
    for (x, y, pixel) in img.pixels() {
        // Extract the color channels (assuming the image is in RGBA format)
        let rgba = pixel.0;

        // Extract the MSB of each color channel
        let r_msb = (rgba[0] >> 7) & 0x01;
        let g_msb = (rgba[1] >> 7) & 0x01;
        let b_msb = (rgba[2] >> 7) & 0x01;

        // Create a new pixel with the MSBs (scaled up to 255 for visibility)
        let msb_pixel = Rgba([r_msb * 255, g_msb * 255, b_msb * 255, 255]);

        // Put the new pixel into the MSB image buffer
        msb_img.put_pixel(x, y, msb_pixel);
    }
    msb_img.save("../msb_image.jpeg");
    msb_img
}
