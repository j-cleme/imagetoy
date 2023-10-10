extern crate photon_rs;
use photon_rs::{ monochrome, transform };
use photon_rs::native::{ open_image, save_image };
use clap::{ Parser, ValueEnum };

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Image to be processed
    image: String,
    /// Transform to apply to image
    #[arg(long)]
    transform: Option<Transformation>,
    /// Correction to apply to image
    #[arg(long)]
    correction: Option<Correction>,
    /// Channel to apply to image
    #[arg(long)]
    channel: Option<Channel>,
    /// Monochrome to apply to image
    #[arg(long)]
    monochrome: Option<Monochrome>,
    /// Color to apply to image
    #[arg(long)]
    color: Option<Color>,
    /// Filter to apply to image
    #[arg(long)]
    filter: Option<Filter>,
    /// Text to apply to image
    #[arg(long)]
    text: Option<Text>,
    /// Watermark to apply to image
    #[arg(long)]
    watermark: Option<Watermark>,
    /// Blend to apply to image
    #[arg(long)]
    blend: Option<Blend>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Transformation {
    /// Compression
    Compress,
    /// Crop an image.
    Crop,
    /// Flip an image horizontally.
    Fliph,
    /// Flip an image vertically.
    Flipv,
    /// Apply padding on the left side of the PhotonImage A padded PhotonImage is returned.
    PaddingBottom,
    /// Apply padding on the left side of the PhotonImage A padded PhotonImage is returned.
    PaddingLeft,
    /// Apply padding on the left side of the PhotonImage A padded PhotonImage is returned.
    PaddingRight,
    /// Apply padding on the left side of the PhotonImage A padded PhotonImage is returned.
    PaddingTop,
    /// Apply uniform padding around the PhotonImage A padded PhotonImage is returned.
    PaddingUniform,
    /// Resample the PhotonImage.
    Resample,
    /// Resize an image.
    Resize,
    /// Rotate the PhotonImage on an arbitrary angle A rotated PhotonImage is returned.
    Rotate,
    /// Resize image using seam carver. Resize only if new dimensions are smaller, than original image.
    SeamCarve,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Correction {
    Sepia,
    Grayscale,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Channel {
    Sepia,
    Grayscale,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Monochrome {
    /// doc todo
    BGrayscale,
    /// doc todo
    DecomposeMax,
    /// doc todo
    DecomposeMin,
    /// doc todo
    Desaturate,
    /// doc todo
    GGrayscale,
    /// doc todo
    Grayscale,
    /// doc todo
    GrayscaleHuman,
    /// doc todo
    GrayscaleShades,
    /// doc todo
    Monochrome,
    /// doc todo
    RGrayscale,
    /// doc todo
    Sepia,
    /// doc todo
    SingleChannelGrayscale,
    /// doc todo
    Threshold,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Color {
    Sepia,
    Grayscale,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Filter {
    Sepia,
    Grayscale,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Text {
    Sepia,
    Grayscale,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Watermark {
    Sepia,
    Grayscale,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Blend {
    Sepia,
    Grayscale,
}

fn main() {
    let args = Args::parse();
    let target_image = args.image;
    let transform = args.transform;
    let correction = args.correction;
    let channel = args.channel;
    let monochrome = args.monochrome;

    let mut img = open_image(&target_image).expect("File should open");

    match transform {
        Some(Transformation::Compress) => todo!(),
        Some(Transformation::Crop) => todo!(),
        Some(Transformation::Fliph) => transform::fliph(&mut img),
        Some(Transformation::Flipv) => transform::flipv(&mut img),
        Some(Transformation::PaddingBottom) => todo!(),
        Some(Transformation::PaddingLeft) => todo!(),
        Some(Transformation::PaddingRight) => todo!(),
        Some(Transformation::PaddingTop) => todo!(),
        Some(Transformation::PaddingUniform) => todo!(),
        Some(Transformation::Resample) => todo!(),
        Some(Transformation::Resize) => todo!(),
        Some(Transformation::Rotate) => todo!(),
        Some(Transformation::SeamCarve) => todo!(),
        None => (),
    }

    match monochrome {
        Some(Monochrome::BGrayscale) => monochrome::b_grayscale(&mut img),
        Some(Monochrome::DecomposeMax) => monochrome::decompose_max(&mut img),
        Some(Monochrome::DecomposeMin) => monochrome::decompose_min(&mut img),
        Some(Monochrome::Desaturate) => monochrome::desaturate(&mut img),
        Some(Monochrome::GGrayscale) => monochrome::g_grayscale(&mut img),
        Some(Monochrome::Grayscale) => monochrome::grayscale(&mut img),
        Some(Monochrome::GrayscaleHuman) => monochrome::grayscale_human_corrected(&mut img),
        Some(Monochrome::GrayscaleShades) => {
            todo!();
            // monochrome::grayscale_shades(&mut img, num_shades);
        }
        Some(Monochrome::Monochrome) => {
            todo!();
            // monochrome::monochrome(&mut img, r_offset, g_offset, b_offset);
        }
        Some(Monochrome::RGrayscale) => monochrome::r_grayscale(&mut img),
        Some(Monochrome::Sepia) => monochrome::sepia(&mut img),
        Some(Monochrome::SingleChannelGrayscale) => {
            todo!();
            // monochrome::single_channel_grayscale(&mut img, channel);
        }
        Some(Monochrome::Threshold) => {
            todo!();
            // monochrome::threshold(&mut img, threshold);
        }
        None => (),
    }

    save_image(img, &format!("processed_{}.jpg", &target_image)).expect("File should be saved");
}
