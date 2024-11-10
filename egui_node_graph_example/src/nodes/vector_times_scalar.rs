// src/nodes/vector_times_scalar.rs

use crate::types::*;
use crate::utils::*;
use egui_node_graph::*;

pub fn build_node(graph: &mut MyGraph, node_id: NodeId) {
    graph.add_input_param(
        node_id,
        "vector".to_string(),
        MyDataType::Vec2,
        MyValueType::Vec2 {
            value: egui::Vec2::ZERO,
        },
        InputParamKind::ConnectionOrConstant,
        true,
    );
    graph.add_input_param(
        node_id,
        "scalar".to_string(),
        MyDataType::Scalar,
        MyValueType::Scalar { value: 1.0 },
        InputParamKind::ConnectionOrConstant,
        true,
    );
    graph.add_output_param(node_id, "out".to_string(), MyDataType::Vec2);
}

pub fn evaluate(evaluator: &mut Evaluator<'_>) -> anyhow::Result<MyValueType> {
    let vector = evaluator.input_vector("vector")?;
    let scalar = evaluator.input_scalar("scalar")?;
    evaluator.output_vector("out", vector * scalar)
}
