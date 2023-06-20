use rand::{Rng, distributions::Uniform};
// use delaunator::{Point, Triangulation};
use image::{GrayImage, ImageBuffer, DynamicImage};

fn main() {
    const PICTURE_FILE_PATH: &str = "C:\\Users\\James\\Documents\\Projects\\low_poly_art\\galaxy.jpg";
    let opened_image = image::open(PICTURE_FILE_PATH).unwrap();
    let greyscale_image = opened_image.grayscale();
    greyscale_image.save("images\\test.jpg").unwrap();
    let x = greyscale_image.blur(2.0);
    let x2 = greyscale_image.blur(30.0);    
    highlight_details(x.to_luma8(), x2.to_luma8())
    sample_verticies_from_image
}

fn highlight_details(image_one: GrayImage, image_two: GrayImage) {
    let (width, height) = image_one.dimensions();
    let mut new_image_buffer = ImageBuffer::new(width, height);
    let mut final_image_buffer = ImageBuffer::new(width, height);
    let mut max_pixel_diff = 0.0;

    for (x, y, pixel) in new_image_buffer.enumerate_pixels_mut() {
        let pixel1 = image_one.get_pixel(x, y)[0];
        let pixel2 = image_two.get_pixel(x, y)[0];

        let mut diff = (pixel1 as f64 - pixel2 as f64) as f64;

        if diff < 0.0 {
            diff *= 0.1
        }

        let abs_diff = diff.abs();

        if abs_diff > max_pixel_diff {max_pixel_diff = abs_diff} ;
        
        *pixel = image::Luma([abs_diff]);
    }

    for (x, y, _) in new_image_buffer.enumerate_pixels() {
        let pixel_value = new_image_buffer.get_pixel(x, y)[0];
        let normalised_pixel = (((pixel_value / max_pixel_diff).sqrt()) * 255.0) as u8;
        final_image_buffer.put_pixel(x, y, image::Luma([normalised_pixel]))
    }

    final_image_buffer.save("images\\diff_image.jpg").unwrap();

}

fn sample_verticies_from_image(image: DynamicImage) {
    let mut n = 1000000;
    let mut rng = rand::thread_rng();
    let (width, height) = image.dimensions();




    let xs = rng.gen_range(0..width);
    let ys = rng.gen_range(0..height);
}
 