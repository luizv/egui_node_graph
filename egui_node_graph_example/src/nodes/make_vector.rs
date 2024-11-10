use crate::types::*;
use crate::utils::*;
use egui_node_graph::*;

pub fn build_node(graph: &mut MyGraph, node_id: NodeId) {
    graph.add_input_param(
        node_id,
        "x".to_string(),
        MyDataType::Scalar,
        MyValueType::Scalar { value: 0.0 },
        InputParamKind::ConnectionOrConstant,
        true,
    );
    graph.add_input_param(
        node_id,
        "y".to_string(),
        MyDataType::Scalar,
        MyValueType::Scalar { value: 0.0 },
        InputParamKind::ConnectionOrConstant,
        true,
    );
    graph.add_output_param(node_id, "out".to_string(), MyDataType::Vec2);
}

pub fn evaluate(evaluator: &mut Evaluator<'_>) -> anyhow::Result<MyValueType> {
    let x = evaluator.input_scalar("x")?;
    let y = evaluator.input_scalar("y")?;
    evaluator.output_vector("out", egui::vec2(x, y))
}
