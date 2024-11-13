use crate::types::{MyDataType, MyNodeData, MyValueType};
use crate::utils::*;
use egui_node_graph::*;
use image::ImageBuffer;

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
        // Simply pass the image data through to the output
        evaluator.populate_output(
            "out",
            MyValueType::Image {
                data,
                pending_image: None,
            },
        )
    } else {
        anyhow::bail!("Invalid input: Expected an image");
    }
}
