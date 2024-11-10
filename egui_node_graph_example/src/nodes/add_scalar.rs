use crate::types::{MyDataType, MyNodeData, MyValueType};
use crate::utils::Evaluator;
use egui_node_graph::*;

pub fn build_node(graph: &mut Graph<MyNodeData, MyDataType, MyValueType>, node_id: NodeId) {
    graph.add_input_param(
        node_id,
        "A".to_string(),
        MyDataType::Scalar,
        MyValueType::Scalar { value: 0.0 },
        InputParamKind::ConnectionOrConstant,
        true,
    );
    graph.add_input_param(
        node_id,
        "B".to_string(),
        MyDataType::Scalar,
        MyValueType::Scalar { value: 0.0 },
        InputParamKind::ConnectionOrConstant,
        true,
    );
    graph.add_output_param(node_id, "out".to_string(), MyDataType::Scalar);
}

pub fn evaluate(evaluator: &mut Evaluator) -> anyhow::Result<MyValueType> {
    let a = evaluator.input_scalar("A")?;
    let b = evaluator.input_scalar("B")?;
    evaluator.output_scalar("out", a + b)
}
