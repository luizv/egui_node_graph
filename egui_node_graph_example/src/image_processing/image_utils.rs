use anyhow::Result;
use image::DynamicImage;
use image::{ImageBuffer, Rgba};

pub fn encode_image_as_png(image_buffer: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> Result<Vec<u8>> {
    let (width, height) = image_buffer.dimensions();
    let mut buffer = Vec::new();
    {
        // O escopo é importante para garantir que o encoder finalize a escrita no buffer
        let encoder = image::codecs::png::PngEncoder::new(&mut buffer);
        encoder.encode(image_buffer, width, height, image::ColorType::Rgba8)?;
    }
    Ok(buffer)
}

pub fn decode_image_from_memory(data: &[u8]) -> Result<DynamicImage> {
    let image = match image::load_from_memory(data) {
        Ok(img) => img,
        Err(err) => {
            eprintln!("Failed to load image: {}", err);
            anyhow::bail!("Failed to load image");
        }
    };
    // Converta a imagem para RGBA8 e retorne o ImageBuffer
    Ok(image)
}
