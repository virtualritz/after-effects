use after_effects::Error;
use image::{ImageBuffer, Rgba};
use log;

pub fn save_frame_as_png(
    data: &[u8],
    width: u32,
    height: u32,
    stride: u32,
    bit_depth: i32,
    file_path: &std::path::Path,
) -> Result<(), Error> {
    if bit_depth != 8 {
        log::error!(
            "Unsupported bit depth: {}. Only 8-bit images are supported",
            bit_depth
        );
        return Err(Error::Generic);
    }

    let mut img_buffer = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let idx = (y * stride + x * 4) as usize;

            let a = data[idx];
            let r = data[idx + 1];
            let g = data[idx + 2];
            let b = data[idx + 3];

            img_buffer.put_pixel(x, y, Rgba([r, g, b, a]));
        }
    }

    img_buffer.save(file_path).map_err(|_| Error::Generic)?;
    log::info!("Successfully saved frame to {:?}", file_path);

    Ok(())
}
