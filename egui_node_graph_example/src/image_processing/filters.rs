use image::DynamicImage;

pub enum FilterType {
    Grayscale,
    Blur(f32),
    // Adicione outros filtros conforme necessário
}

impl FilterType {
    pub fn apply_filter(image: DynamicImage, filter: FilterType) -> DynamicImage {
        match filter {
            FilterType::Grayscale => image.grayscale(),
            FilterType::Blur(sigma) => image.blur(sigma),
            // Implemente outros filtros aqui
        }
    }
}
