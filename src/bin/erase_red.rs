use clap::Parser;
use image::{GenericImageView, ImageBuffer, ImageReader};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input file path of image
    #[arg(short, long)]
    input: String,
    /// Output file path of image
    #[arg(short, long)]
    output: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let img = ImageReader::open(args.input)?.decode()?;
    let (width, height) = img.dimensions();
    let mut imgbuf = ImageBuffer::new(width, height);

    let (start, end) = (125, 255);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let mut rgba = img.get_pixel(width - x - 1, height - y - 1);
        if (rgba.0[0] >= start && rgba.0[0] <= end)
            || (rgba.0[1] >= start && rgba.0[1] <= end)
            || (rgba.0[2] >= start && rgba.0[2] <= end)
        {
            rgba.0[0] = 255;
            rgba.0[1] = 255;
            rgba.0[2] = 255;
        }
        *pixel = rgba;
    }
    imgbuf.save(&args.output)?;
    println!("make a new file {}", args.output);
    Ok(())
}
