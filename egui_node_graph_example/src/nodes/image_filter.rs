use crate::types::{MyGraph, MyValueType};
use crate::utils::Evaluator;
use egui_node_graph::*;

pub fn build_node(graph: &mut MyGraph, node_id: NodeId) {
    graph.add_input_param(
        node_id,
        "input_image".to_string(),
        crate::types::MyDataType::Image,
        MyValueType::Image { data: vec![] },
        InputParamKind::ConnectionOrConstant,
        true,
    );

    graph.add_output_param(
        node_id,
        "filtered_image".to_string(),
        crate::types::MyDataType::Image,
    );
}

/// Function to evaluate an Image Filter node.
pub fn evaluate(evaluator: &mut Evaluator<'_>) -> anyhow::Result<MyValueType> {
    // Fetch the input image value.
    let input_image = evaluator.evaluate_input("input_image")?;

    // Ensure the input is of type `Image`.
    let image_data = if let MyValueType::Image { data } = input_image {
        data
    } else {
        anyhow::bail!("Expected input of type Image, got {:?}", input_image);
    };

    // Here, you would perform the image filtering logic.
    // For demonstration purposes, let's assume it inverts the image bytes.
    let filtered_data: Vec<u8> = image_data.iter().map(|&byte| 255 - byte).collect();

    // Output the filtered image.
    evaluator.populate_output(
        "filtered_image",
        MyValueType::Image {
            data: filtered_data,
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{MyGraphState, MyNodeData, MyNodeTemplate};
    use crate::utils::{evaluate_node, OutputsCache};

    #[test]
    fn test_image_filter_node() {
        let mut graph = MyGraph::new();
        let node_id = graph.add_node(
            "ImageFilter".to_string(),
            MyNodeData {
                template: MyNodeTemplate::ImageFilter, // Ensure this is defined in `MyNodeTemplate`.
            },
            |_, _| {},
        );

        build_node(&mut graph, node_id);

        // Set input image data.
        let input_id = graph[node_id].get_input("input_image").unwrap();
        graph[input_id].value = MyValueType::Image {
            data: vec![0, 128, 255],
        };

        let mut outputs_cache = OutputsCache::new();

        // Evaluate the node.
        let result = evaluate_node(&graph, node_id, &mut outputs_cache).unwrap();

        // Check the output value.
        if let MyValueType::Image { data } = result {
            assert_eq!(data, vec![255, 127, 0]); // Expected inverted values.
        } else {
            panic!("Expected output of type Image, got {:?}", result);
        }
    }
}
