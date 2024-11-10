use crate::types::{MyDataType, MyNodeData, MyValueType};
use crate::utils::*;

use egui_node_graph::*; // Import shared types

pub fn build_node(graph: &mut Graph<MyNodeData, MyDataType, MyValueType>, node_id: NodeId) {
    graph.add_input_param(
        node_id,
        "value".to_string(),
        MyDataType::Scalar,
        MyValueType::Scalar { value: 0.0 },
        InputParamKind::ConnectionOrConstant,
        true,
    );
    graph.add_output_param(node_id, "out".to_string(), MyDataType::Scalar);
}

pub fn evaluate(evaluator: &mut Evaluator<'_>) -> anyhow::Result<MyValueType> {
    let value = evaluator.input_scalar("value")?;
    evaluator.output_scalar("out", value)
}
