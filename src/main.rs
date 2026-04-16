use clap::Parser;
use image::{DynamicImage, GrayImage, ImageError, Rgba, RgbaImage, imageops::FilterType};
// Raw string with r#"..."# - no escaping needed
const ASCII_RAMP: &str = "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'. ";
/// My Rust practice and your cli tools to convert images into ACII text!
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    ///path or name of the image file
    file_name: String,
    /// Output is greyscale by default but you can add color using this flag!
    #[arg(short, long)]
    color: bool,
    /// inverts ASCII ramp sequence to make white spots sparse and black spots dense.
    #[arg(short, long)]
    inverted: bool,
    ///increases the image contrast to achieve better quality ASCII with lower dimentions.
    #[arg(short, long)]
    quality: bool,
    /// width of output if not present, scales according to height if present, set to 200 if not
    #[arg(short = 'x', long)]
    width: Option<u32>,
    /// height of outputif not present, scales according to width if present, set to 200 if not
    #[arg(short = 'y', long)]
    height: Option<u32>,
    ///output file name
    #[arg(short, long)]
    output: Option<String>,
}
fn main() {
    let args = Args::parse();
    println!("Reading image file....");
    let img = match imageread(&args.file_name) {
        Ok(img) => img,
        Err(_e) => {
            println!("Could not load or find image (:<)");
            return;
        }
    };
    //scale image if only one of the arguments is present
    let (x, y) = calculate_dimensions(&args.width, &args.height, &img);
    let scaled_image = img.resize_exact(x, y, FilterType::Lanczos3).to_rgba8();
    let workingimage = if args.quality {
        let scaled_image_cn = adjust_image_contrast(scaled_image.clone(), 1.5); // clone here
        DynamicImage::ImageRgba8(scaled_image_cn).to_luma8()
    } else {
        DynamicImage::ImageRgba8(scaled_image.clone()).to_luma8()
    };

    let to_ret = make_ascii(&workingimage, &scaled_image, &args.color, &args.inverted);
    println!("\n{}", to_ret);
    if args.output.is_some() {
        let o_path = args.output.unwrap();
        std::fs::write(&o_path, &to_ret).expect("Unable to write to the outputfile");
        println!("ASCII art saved to {}", o_path);
    }
}
fn imageread(path: &str) -> Result<DynamicImage, ImageError> {
    return image::open(path);
}
fn calculate_dimensions(x: &Option<u32>, y: &Option<u32>, img: &DynamicImage) -> (u32, u32) {
    if y.is_some() && x.is_none() {
        let target_h = y.unwrap();
        let scale = target_h as f32 / img.height() as f32;
        return ((img.width() as f32 * scale).round() as u32 * 2, target_h);
    } else if y.is_none() && x.is_some() {
        let target_w = x.unwrap(); // fixed: args.width → x
        let scale = target_w as f32 / img.width() as f32;
        return (target_w * 2, (img.height() as f32 * scale).round() as u32);
    }
    (100, 50) // default when both None or both Some
}
fn make_ascii(workingimage: &GrayImage, color_image: &RgbaImage, c: &bool, i: &bool) -> String {
    let ascii_array = ASCII_RAMP.chars().collect::<Vec<char>>();
    let mut to_ret = String::new();
    for y in 0..workingimage.height() {
        for x in 0..workingimage.width() {
            let pixel = workingimage.get_pixel(x, y);
            let brightness = pixel.0[0];
            let index: usize = if *i {
                ((brightness) as usize * ascii_array.len()) / 256
            } else {
                ((255 - brightness) as usize * ascii_array.len()) / 256
            };
            let rgb = color_image.get_pixel(x, y).0;
            let mut to_push = String::from(ascii_array[index.min(ascii_array.len() - 1)]);

            if *c {
                to_push = format!(
                    "\x1b[38;2;{};{};{}m{}\x1b[0m", //\x1b escape character  [38;2 rgb color mode] m to stop  \x1b[0m to reset all formatting
                    rgb[0], rgb[1], rgb[2], to_push
                );
            }
            to_ret.push_str(&to_push); // Use push_str for String
        }
        to_ret.push('\n');
    }
    return to_ret;
}
fn adjust_image_contrast(img: RgbaImage, factor: f32) -> RgbaImage {
    let mut new_img = img.clone();
    for (x, y, pixel) in img.enumerate_pixels() {
        let r = adjust_contrast(pixel[0], factor);
        let g = adjust_contrast(pixel[1], factor);
        let b = adjust_contrast(pixel[2], factor);
        let a = pixel[3]; // Keep alpha unchanged
        new_img.put_pixel(x, y, Rgba([r, g, b, a]));
    }
    return new_img;
}
fn adjust_contrast(value: u8, factor: f32) -> u8 {
    let normalized = value as f32 / 255.0;
    let adjusted = (normalized - 0.5) * factor + 0.5;
    (adjusted.clamp(0.0, 1.0) * 255.0) as u8
}
