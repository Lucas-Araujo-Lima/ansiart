use image::{GenericImageView, ImageReader, Pixel, Rgba};

fn set_foreground(pixel: &<image::DynamicImage as GenericImageView>::Pixel) {
    let pixel = pixel.to_rgb();
    print!("\x1b[38;2;{};{};{}m", pixel.0[0], pixel.0[1], pixel.0[2]);
}

fn set_background(pixel: &<image::DynamicImage as GenericImageView>::Pixel) {
    let pixel = pixel.to_rgb();
    print!("\x1b[48;2;{};{};{}m", pixel.0[0], pixel.0[1], pixel.0[2]);
}

fn reset_colors() {
    print!("\x1b[0m");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if let None = args.get(1) {
        eprintln!("Please provide an image file");
        return;
    }

    let filepath = &args[1];

    let img = ImageReader::open(filepath).unwrap().decode().unwrap();
    let w = img.width();
    let h = img.height();

    for y in (0..h).step_by(2) {
        for x in 0..w {
            let upper_pixel = img.get_pixel(x, y);
            let mut lower_pixel: Rgba<u8> = Rgba([0, 0, 0, 255]);
            if img.in_bounds(x, y + 1) {
                lower_pixel = img.get_pixel(x, y + 1);
            }

            set_foreground(&upper_pixel);
            set_background(&lower_pixel);
            print!("▀");
        }
        reset_colors();
        println!();
    }
}
