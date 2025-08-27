pub mod palette;

use std::env;
use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView, ImageError, ImageReader, Rgb};

use crate::palette::{get_palette, Palette};

/// Pixelate an image by resizing it to a smaller size
fn pixelate(image: DynamicImage, pixel_size: u32) -> Result<DynamicImage, ImageError> {
    let image = image.resize(
        image.width() / pixel_size,
        image.height() / pixel_size,
        FilterType::Nearest,
    );
    return Ok(image);
}

/// Euclidean distance between two colors
fn euclidean_distance(a: Rgb<u8>, b: Rgb<u8>) -> u32 {
    a.0.iter()
        .zip(b.0.iter())
        .map(|(x, y)| (*x as i32 - *y as i32).pow(2) as u32)
        .sum()
}

/// Find the closest color in a palette to a given color
fn closest_color(color: Rgb<u8>, palette: &Palette) -> Rgb<u8> {
    let mut closest: Rgb<u8> = *palette.get_color(0).unwrap();
    let mut min_distance = u32::MAX;
    for palette_color in palette.colors.iter() {
        let distance = euclidean_distance(color, *palette_color);
        if distance < min_distance {
            min_distance = distance;
            closest = palette_color.clone();
        }
    }
    closest
}

/// Colorize an image using a palette
fn colorize(image: DynamicImage, palette: &Palette) -> Result<DynamicImage, ImageError> {
    let mut imagebuf = image.to_rgb8();
    for x in 0..imagebuf.width() {
        for y in 0..imagebuf.height() {
            let pixel = imagebuf.get_pixel(x, y);
            let color = closest_color(*pixel, palette);
            imagebuf.put_pixel(x, y, color);
        }
    }
    Ok(DynamicImage::from(imagebuf))
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check for help argument
    if args.len() > 1 && (args[1] == "--help" || args[1] == "-h") {
        println!("Usage: {} <image_path> [pixel_size] [palette]", args[0]);
        println!("  image_path  - Path to the input image (mandatory)");
        println!("  pixel_size  - Size for pixelation (optional, default: 0)");
        println!("  palette     - Color palette to use (optional, default: pico8)");
        println!("  --help, -h  - Show this help message");
        return;
    }

    if args.len() < 2 || args.len() > 4 {
        eprintln!("Usage: {} <image_path> [pixel_size] [palette]", args[0]);
        eprintln!("Use --help for more information");
        std::process::exit(1);
    }

    let image_path = &args[1];

    let pixel_size = if args.len() >= 3 {
        match args[2].parse::<u32>() {
            Ok(size) => size,
            Err(_) => {
                eprintln!("Error: pixel_size must be a valid number");
                std::process::exit(1);
            }
        }
    } else {
        0
    };

    let palette_name = if args.len() == 4 {
        &args[3]
    } else {
        "pico8"
    };


    let palette = get_palette(palette_name).expect("Palette not found");
    let image = ImageReader::open(image_path)
        .map_err(|e| format!("Failed to open image: {}", e))
        .unwrap()
        .decode()
        .map_err(|e| format!("Failed to decode image: {}", e))
        .unwrap();
    let (ogwidth, ogheight) = image.dimensions();
    let image = pixelate(image, pixel_size).unwrap();
    let image = colorize(image, &palette).unwrap();
    let image = image.resize(ogwidth, ogheight, FilterType::Lanczos3);
    image.save("output.png").expect("Failed to save image");
    println!("image saved as output.png");
}
