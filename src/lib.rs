pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use image::{GenericImageView, ImageBuffer, ImageReader};

    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn copy_image_pixel_one_by_one() -> anyhow::Result<()> {
        let _ = tracing_subscriber::fmt().with_line_number(true).try_init();
        let img = ImageReader::open("./data/scan_raw.jpeg")?.decode()?;
        let (width, height) = img.dimensions();
        let mut imgbuf = ImageBuffer::new(width, height);
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let rgba = img.get_pixel(width - x - 1, height - y - 1);
            *pixel = rgba;
        }
        imgbuf.save("./data/scan_output.png")?;
        Ok(())
    }
}
