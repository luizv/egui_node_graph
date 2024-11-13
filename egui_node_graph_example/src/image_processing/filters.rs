use image::imageops;
use image::DynamicImage;

pub enum FilterType {
    Grayscale,
    Blur(f32),
    Invert,
    Brighten(i32),
    Contrast(f32),
    FlipHorizontal,
    FlipVertical,
    Rotate90,
    HueRotate(i32),
    Mix(DynamicImage, i32),
}

impl FilterType {
    pub fn apply_filter(image: DynamicImage, filter: FilterType) -> DynamicImage {
        match filter {
            FilterType::Grayscale => image.grayscale(),
            FilterType::Blur(sigma) => image.blur(sigma),
            FilterType::Invert => {
                let mut img = image.to_rgba8();
                imageops::invert(&mut img);
                DynamicImage::ImageRgba8(img)
            }
            FilterType::Brighten(value) => image.brighten(value),
            FilterType::Contrast(value) => image.adjust_contrast(value),
            FilterType::FlipHorizontal => image.fliph(),
            FilterType::FlipVertical => image.flipv(),
            FilterType::Rotate90 => image.rotate90(),
            FilterType::HueRotate(degrees) => image.huerotate(degrees),
            FilterType::Mix(other_image, factor) => {
                // Converta ambas as imagens para RGBA8
                let mut base_img = image.to_rgba8();
                let (base_width, base_height) = base_img.dimensions();
                let other_img = other_image.to_rgba8();

                // Redimensione a imagem secundária para cobrir a imagem principal
                let resized_other_img = imageops::resize(
                    &other_img,
                    base_width,
                    base_height,
                    imageops::FilterType::Lanczos3,
                );

                // Clampe o fator de mistura entre 0 e 100
                let factor = (factor as f32).clamp(0.0, 100.0) / 100.0;

                // Misture as duas imagens
                for (base_pixel, other_pixel) in
                    base_img.pixels_mut().zip(resized_other_img.pixels())
                {
                    let [r1, g1, b1, a1] = base_pixel.0;
                    let [r2, g2, b2, a2] = other_pixel.0;

                    base_pixel.0 = [
                        (r1 as f32 * (1.0 - factor) + r2 as f32 * factor) as u8,
                        (g1 as f32 * (1.0 - factor) + g2 as f32 * factor) as u8,
                        (b1 as f32 * (1.0 - factor) + b2 as f32 * factor) as u8,
                        (a1 as f32 * (1.0 - factor) + a2 as f32 * factor) as u8,
                    ];
                }

                DynamicImage::ImageRgba8(base_img)
            }
        }
    }
}
