use crate::image_processing::filters::FilterType;
use crate::image_processing::image_utils::{decode_image_from_memory, encode_image_as_png};
use crate::types::{MyDataType, MyNodeData, MyValueType};
use crate::utils::*;
use egui_node_graph::*;

// Função para construir o nó ContrastFilter
pub fn build_node(graph: &mut Graph<MyNodeData, MyDataType, MyValueType>, node_id: NodeId) {
    graph.add_input_param(
        node_id,
        "image".to_string(),
        MyDataType::Image,
        MyValueType::default_image(),
        InputParamKind::ConnectionOrConstant,
        true,
    );

    graph.add_input_param(
        node_id,
        "value".to_string(),
        MyDataType::Scalar,
        MyValueType::Scalar { value: 0.0 },
        InputParamKind::ConnectionOrConstant,
        true,
    );

    graph.add_output_param(node_id, "out".to_string(), MyDataType::Image);
}

pub fn evaluate(evaluator: &mut Evaluator<'_>) -> anyhow::Result<MyValueType> {
    let image_value = evaluator.evaluate_input("image")?;
    let value = evaluator.evaluate_input("value")?;

    if let (MyValueType::Image { data, .. }, MyValueType::Scalar { value }) = (image_value, value) {
        let image = decode_image_from_memory(&data)?;

        let filter = FilterType::Contrast(value);

        let processed_image = FilterType::apply_filter(image, filter);

        // Processamento de imagem...

        // Converta a imagem processada para RGBA8
        let image_buffer = processed_image.to_rgba8();

        // Codifique o ImageBuffer como PNG
        let buffer = encode_image_as_png(&image_buffer)?;

        evaluator.populate_output(
            "out",
            MyValueType::Image {
                data: buffer,
                pending_image: None,
            },
        )
    } else {
        anyhow::bail!("Entradas inválidas: Esperado uma imagem e um valor escalar");
    }
}
