use std::path::PathBuf;

use clap::{command, Parser};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ImageFormat {
    PNG,
    PPM,
}

impl clap::ValueEnum for ImageFormat {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::PNG, Self::PPM]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            Self::PNG => Some(clap::builder::PossibleValue::new("png")),
            Self::PPM => Some(clap::builder::PossibleValue::new("ppm")),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Image {
    Clock,
    Circle,
    Sphere,
    SphereRayon,
}

impl clap::ValueEnum for Image {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Clock, Self::Circle, Self::Sphere, Self::SphereRayon]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            Self::Clock => Some(clap::builder::PossibleValue::new("clock")),
            Self::Circle => Some(clap::builder::PossibleValue::new("circle")),
            Self::Sphere => Some(clap::builder::PossibleValue::new("sphere")),
            Self::SphereRayon => Some(clap::builder::PossibleValue::new("sphere-rayon")),
        }
    }
}

#[derive(Parser, Debug, Clone)]
#[command(name = "macroquad-raytracer", about = "A simple raytracer")]
pub struct AppOptions {
    /// Path to directory to save images
    #[arg(short, long, default_value = ".")]
    pub directory: PathBuf,

    /// Generate this image
    #[arg(short, long, default_value = "sphere-rayon")]
    pub image: Image,

    /// Save output as an image of this type
    #[arg(short = 'F', long)]
    pub format: Option<ImageFormat>,

    /// Basename (without extension) to save image as
    #[arg(short, long, default_value = "canvas")]
    pub filename: String,

    /// Don't show the generated image in a window
    #[arg(short = 'H', long)]
    pub hide: bool,

    /// Display elapsed time on stdout
    #[arg(short = 't', long)]
    pub time: bool,

    /// Size of canvas in pixels for both width and height
    #[arg(short = 's', long, default_value = "100")]
    pub size: usize,
}

impl AppOptions {
    pub fn image_path(&self) -> Option<PathBuf> {
        match self.format {
            Some(ImageFormat::PNG) => Some(self.directory_path("png")),
            Some(ImageFormat::PPM) => Some(self.directory_path("ppm")),
            None => None,
        }
    }

    fn directory_path(&self, extension: &str) -> PathBuf {
        let mut path = self.directory.clone();
        path.push(&self.filename);
        path.set_extension(extension);
        path
    }
}
