use clap::{Arg, Parser};
use image::{
    DynamicImage, GenericImageView, ImageError,
    imageops::FilterType::{self, Lanczos3},
    math,
};
use std::{env::args, ops::Index, path::Path, process::Output};
// Raw string with r#"..."# - no escaping needed
const ASCII_RAMP: &str =
    r#" .'`^",:;Il!i><~+_-?][}{1)(|\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$"#;
/// My Rust practice and your cli tools to convert images into ACII text!
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    ///path or name of the image file
    file_name: String,

    /// ASCII is greyscale by default but you can add color using this flag!
    #[arg(short, long)]
    color: bool,

    /// width of output
    #[arg(short = 'x', long)]
    width: Option<u32>,
    /// height of output
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
    let mut x = 200;
    let mut y = 100;
    /*if args.height.is_some() && !args.width.is_some() {
        x = img.width() * (args.height.unwrap() / img.height());
    } else if !args.height.is_some() && args.width.is_some() {
        y = img.height() * (args.width.unwrap() / img.width());
    }*/
    let workingimage = (img.resize_exact(x, y, FilterType::Lanczos3)).to_luma8();
    let ascii_array = ASCII_RAMP.chars().collect::<Vec<char>>();
    let mut to_ret = String::new();
    for y in 0..workingimage.height() {
        for x in 0..workingimage.width() {
            let pixel = workingimage.get_pixel(x, y);
            let brigthness = pixel.0[0];
            let index = (brigthness as usize * ascii_array.len()) / 256;
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
