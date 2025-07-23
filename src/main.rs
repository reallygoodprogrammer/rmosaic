use clap::Parser;
use image::{ImageBuffer, open, RgbImage};
use rand::Rng;

#[derive(Parser)]
struct Args {
    #[arg(help = "input file for mosaic'ing")]
    input_file: String,
    #[arg(
        short,
        long,
        default_value_t = String::from("output.png"), 
        help = "output file name"
        )]
    output_file: String,
    #[arg(
        short,
        long,
        default_value_t = 0.5,
        help = "decrease chance by chance * argument per split"
        )]
    decrement: f32,
    #[arg(
        long,
        default_value_t = 5,
        help = "minimum width of mosaic piece"
        )]
    width_min: u32,
    #[arg(
        long,
        default_value_t = 5,
        help = "minimum height of mosaic piece"
        )]
    height_min: u32,
}

fn main() {
    let args = Args::parse();
    let mut in_file = open(args.input_file.clone())
        .expect("could not open file")
        .to_rgb8();
    let mut rng = rand::rng();

    in_file = split(
        in_file, 
        1.0, 
        &args,
        &mut rng,
    );
    in_file.save(args.output_file)
        .expect("failed to save file");
}

fn split<T: Rng>(mut in_file: RgbImage, chance: f32, args: &Args, rng: &mut T) -> RgbImage {
    let (width, height) = in_file.dimensions();
    if rng.random::<f32>() < chance && width > args.width_min {
        let w_split_at = rng.random::<u32>() % width;
        let first = split(
            image::imageops::crop_imm(&in_file, 0, 0, w_split_at, height).to_image(),
            chance * args.decrement,
            args,
            rng,
        );
        let second = split(
            image::imageops::crop_imm(&in_file, w_split_at, 0, width - w_split_at, height).to_image(),
            chance * args.decrement,
            args,
            rng,
        );
        let mut combined = ImageBuffer::new(width, height);
        image::imageops::replace(&mut combined, &second, 0, 0);
        image::imageops::replace(&mut combined, &first, (width - w_split_at).into(), 0);
        in_file = combined;
    } 

    if rng.random::<f32>() < chance && height > args.height_min {
        let h_split_at = rng.random::<u32>() % height;
        let first = split(
            image::imageops::crop_imm(&in_file, 0, 0, width, h_split_at).to_image(),
            chance * args.decrement,
            args,
            rng,
        );
        let second = split(
            image::imageops::crop_imm(&in_file, 0, h_split_at, width, height - h_split_at).to_image(),
            chance * args.decrement,
            args,
            rng,
        );
        let mut combined = ImageBuffer::new(width, height);
        image::imageops::replace(&mut combined, &second, 0, 0);
        image::imageops::replace(&mut combined, &first, 0, (height - h_split_at).into());
        in_file = combined;
    }
    in_file
}
