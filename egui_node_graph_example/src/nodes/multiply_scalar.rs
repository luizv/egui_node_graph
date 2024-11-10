// src/nodes/multiply_scalar.rs

use crate::types::*;
use crate::utils::*;
use egui_node_graph::*;

pub fn build_node(graph: &mut MyGraph, node_id: NodeId) {
    graph.add_input_param(
        node_id,
        "A".to_string(),
        MyDataType::Scalar,
        MyValueType::Scalar { value: 1.0 },
        InputParamKind::ConnectionOrConstant,
        true,
    );
    graph.add_input_param(
        node_id,
        "B".to_string(),
        MyDataType::Scalar,
        MyValueType::Scalar { value: 1.0 },
        InputParamKind::ConnectionOrConstant,
        true,
    );
    graph.add_output_param(node_id, "out".to_string(), MyDataType::Scalar);
}

pub fn evaluate(evaluator: &mut Evaluator) -> anyhow::Result<MyValueType> {
    let a = evaluator.input_scalar("A")?;
    let b = evaluator.input_scalar("B")?;
    evaluator.output_scalar("out", a * b)
}
