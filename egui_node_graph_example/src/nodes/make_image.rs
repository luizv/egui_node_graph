use crate::types::{MyDataType, MyNodeData, MyValueType};
use crate::utils::*;
use egui_node_graph::*;
use std::fs;

// Function to build the MakeImage node
pub fn build_node(graph: &mut Graph<MyNodeData, MyDataType, MyValueType>, node_id: NodeId) {
    // Add an input parameter that allows users to select an image file path
    graph.add_input_param(
        node_id,
        "image_path".to_string(),
        MyDataType::Image,
        MyValueType::Image { data: vec![] }, // Default empty image
        InputParamKind::ConnectionOrConstant,
        true,
    );

    // Add an output parameter for the image data
    graph.add_output_param(node_id, "out".to_string(), MyDataType::Image);
}

// Function to evaluate the MakeImage node
pub fn evaluate(evaluator: &mut Evaluator<'_>) -> anyhow::Result<MyValueType> {
    // Attempt to get the image path from the input parameter
    let image_path_value = evaluator.evaluate_input("image_path")?;

    // Check if the input is of expected type, e.g., MyValueType::Scalar representing a path string.
    if let MyValueType::Scalar { value } = image_path_value {
        // Assuming that the scalar value is used to represent the image path string (this needs clarification):
        let image_path = format!("{}", value); // Convert f32 value to string (for demonstration purposes)

        // Load the image data from the path or use a placeholder
        let image_data = load_image_from_path(&image_path);

        // Output the image data
        evaluator.populate_output("out", MyValueType::Image { data: image_data })
    } else {
        anyhow::bail!("Invalid input: Expected an image path as a string-like type");
    }
}

// Helper function to load image data from the provided path
fn load_image_from_path(image_path: &str) -> Vec<u8> {
    // Attempt to read the image file as bytes
    fs::read(image_path).unwrap_or_else(|_| vec![0u8; 1024]) // Placeholder for error handling
}
