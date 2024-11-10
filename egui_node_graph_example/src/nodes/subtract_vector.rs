// src/nodes/subtract_vector.rs

use crate::types::*;
use crate::utils::*;
use egui_node_graph::*;

pub fn build_node(graph: &mut MyGraph, node_id: NodeId) {
    graph.add_input_param(
        node_id,
        "v1".to_string(),
        MyDataType::Vec2,
        MyValueType::Vec2 {
            value: egui::Vec2::ZERO,
        },
        InputParamKind::ConnectionOrConstant,
        true,
    );
    graph.add_input_param(
        node_id,
        "v2".to_string(),
        MyDataType::Vec2,
        MyValueType::Vec2 {
            value: egui::Vec2::ZERO,
        },
        InputParamKind::ConnectionOrConstant,
        true,
    );
    graph.add_output_param(node_id, "out".to_string(), MyDataType::Vec2);
}

pub fn evaluate(evaluator: &mut Evaluator) -> anyhow::Result<MyValueType> {
    let v1 = evaluator.input_vector("v1")?;
    let v2 = evaluator.input_vector("v2")?;
    evaluator.output_vector("out", v1 - v2)
}
