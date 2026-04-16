use clap::{Arg, Parser};
use image::{
    DynamicImage, GenericImageView, ImageError,
    imageops::FilterType::{self, Lanczos3},
    math,
};
use std::{env::args, ops::Index, path::Path, process::Output};
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
    #[arg[short,long]]
    inverted: bool,
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
        Err(e) => {
            println!("Could not load or find image (:<)");
            return;
        }
    };
    let mut x = 50;
    let mut y = 50;
    //scale image if only one of the arguments is present
    if args.height.is_some() && args.width.is_none() {
        let target_h = args.height.unwrap();
        let scale = target_h as f32 / img.height() as f32;
        x = (img.width() as f32 * scale).round() as u32;
        y = target_h;
    } else if args.height.is_none() && args.width.is_some() {
        let target_w = args.width.unwrap();
        let scale = target_w as f32 / img.width() as f32;
        x = target_w;
        y = (img.height() as f32 * scale).round() as u32;
    }
    x = x * 2;
    let workingimage = (img.resize_exact(x, y, FilterType::Lanczos3)).to_luma8();
    let ascii_array = ASCII_RAMP.chars().collect::<Vec<char>>();
    let mut to_ret = String::new();
    for y in 0..workingimage.height() {
        for x in 0..workingimage.width() {
            let pixel = workingimage.get_pixel(x, y);
            let brightness = pixel.0[0];
            let index: usize = if (args.inverted) {
                ((255 - brightness) as usize * ascii_array.len()) / 256
            } else {
                ((brightness) as usize * ascii_array.len()) / 256
            };
            to_ret.push(ascii_array[index.min(ascii_array.len() - 1)]);
        }
        to_ret.push('\n');
    }
    println!("\n{}", to_ret);
    if (args.output.is_some()) {
        let o_path = args.output.unwrap();
        std::fs::write(&o_path, &to_ret).expect("Unable to write to the outputfile");
        println!("ASCII art saved to {}", o_path);
    }
}
fn imageread(path: &str) -> Result<DynamicImage, ImageError> {
    return image::open(path);
}
