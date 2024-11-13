use crate::types::{MyDataType, MyNodeData, MyValueType};
use crate::utils::*;
use egui_node_graph::*;
use image::{ImageBuffer, Rgba};

// Function to build the MakeImage node
pub fn build_node(graph: &mut Graph<MyNodeData, MyDataType, MyValueType>, node_id: NodeId) {
    graph.add_input_param(
        node_id,
        "image".to_string(),
        MyDataType::Image,
        MyValueType::default_image(),
        InputParamKind::ConnectionOrConstant,
        true,
    );

    graph.add_output_param(node_id, "out".to_string(), MyDataType::Image);
}

pub fn evaluate(evaluator: &mut Evaluator<'_>) -> anyhow::Result<MyValueType> {
    let image_value = evaluator.evaluate_input("image")?;

    if let MyValueType::Image { data, .. } = image_value {
        // Carregue a imagem a partir dos dados
        let image = match image::load_from_memory(&data) {
            Ok(img) => img,
            Err(err) => {
                eprintln!("Failed to load image: {}", err);
                anyhow::bail!("Failed to load image");
            }
        };
        let image_buffer = image.to_rgba8();
        let (width, height) = image_buffer.dimensions();

        // Converta a imagem para preto e branco.
        let mut bw_data = ImageBuffer::new(width, height);

        for (x, y, pixel) in image_buffer.enumerate_pixels() {
            let [r, g, b, a] = pixel.0;
            let gray = ((r as u32 + g as u32 + b as u32) / 3) as u8;
            bw_data.put_pixel(x, y, Rgba([gray, gray, gray, a]));
        }

        // Codifique o ImageBuffer como PNG
        let mut buffer = Vec::new();
        let encoder = image::codecs::png::PngEncoder::new(&mut buffer);
        encoder.encode(&bw_data, width, height, image::ColorType::Rgba8)?;

        // Output the black and white image.
        evaluator.populate_output(
            "out",
            MyValueType::Image {
                data: buffer,
                pending_image: None,
            },
        )
    } else {
        anyhow::bail!("Invalid input: Expected an image");
    }
}
