use crate::types::{MyGraph, MyValueType, NodeDefinition};
use anyhow::Result;
use egui_node_graph::*;
use std::collections::HashMap;

pub type OutputsCache = HashMap<OutputId, MyValueType>;

pub struct Evaluator<'a> {
    pub graph: &'a MyGraph,
    pub outputs_cache: &'a mut OutputsCache,
    pub node_id: NodeId,
}

impl<'a> Evaluator<'a> {
    pub fn new(graph: &'a MyGraph, outputs_cache: &'a mut OutputsCache, node_id: NodeId) -> Self {
        Self {
            graph,
            outputs_cache,
            node_id,
        }
    }

    pub fn evaluate_input(&mut self, name: &str) -> Result<MyValueType> {
        evaluate_input(self.graph, self.node_id, name, self.outputs_cache)
    }

    pub fn populate_output(&mut self, name: &str, value: MyValueType) -> Result<MyValueType> {
        populate_output(self.graph, self.outputs_cache, self.node_id, name, value)
    }

    pub fn input_vector(&mut self, name: &str) -> Result<egui::Vec2> {
        self.evaluate_input(name)?.try_to_vec2()
    }

    pub fn input_scalar(&mut self, name: &str) -> Result<f32> {
        self.evaluate_input(name)?.try_to_scalar()
    }

    pub fn output_vector(&mut self, name: &str, value: egui::Vec2) -> Result<MyValueType> {
        self.populate_output(name, MyValueType::Vec2 { value })
    }

    pub fn output_scalar(&mut self, name: &str, value: f32) -> Result<MyValueType> {
        self.populate_output(name, MyValueType::Scalar { value })
    }
}

pub fn evaluate_node(
    graph: &MyGraph,
    node_id: NodeId,
    outputs_cache: &mut OutputsCache,
) -> Result<MyValueType> {
    let node = &graph[node_id];
    let mut evaluator = Evaluator::new(graph, outputs_cache, node_id);
    let template = node.user_data.template;

    if let Some(definition) = NodeDefinition::all_definitions()
        .iter()
        .find(|def| def.template == template)
    {
        (definition.evaluate)(&mut evaluator)
    } else {
        anyhow::bail!("Unknown node template {:?}", template)
    }
}

fn populate_output(
    graph: &MyGraph,
    outputs_cache: &mut OutputsCache,
    node_id: NodeId,
    param_name: &str,
    value: MyValueType,
) -> Result<MyValueType> {
    let output_id = graph[node_id].get_output(param_name)?;
    outputs_cache.insert(output_id, value.clone());
    Ok(value)
}

fn evaluate_input(
    graph: &MyGraph,
    node_id: NodeId,
    param_name: &str,
    outputs_cache: &mut OutputsCache,
) -> Result<MyValueType> {
    let input_id = graph[node_id].get_input(param_name)?;

    if let Some(other_output_id) = graph.connection(input_id) {
        if let Some(other_value) = outputs_cache.get(&other_output_id) {
            Ok(other_value.clone())
        } else {
            evaluate_node(graph, graph[other_output_id].node, outputs_cache)?;
            Ok(outputs_cache
                .get(&other_output_id)
                .expect("Cache should be populated")
                .clone())
        }
    } else {
        Ok(graph[input_id].value.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nodes;
    use crate::types::*;

    #[test]
    fn test_evaluate_make_scalar_node() {
        let mut graph = MyGraph::new();

        let node_id = graph.add_node(
            "MakeScalar".to_string(),
            MyNodeData {
                template: MyNodeTemplate::MakeScalar,
            },
            |_, _| {},
        );

        nodes::make_scalar::build_node(&mut graph, node_id);

        let input_id = graph[node_id].get_input("value").unwrap();
        graph[input_id].value = MyValueType::Scalar { value: 42.0 };

        let mut outputs_cache = OutputsCache::new();

        let result = evaluate_node(&graph, node_id, &mut outputs_cache).unwrap();

        assert_eq!(result, MyValueType::Scalar { value: 42.0 });
    }

    #[test]
    fn test_evaluate_add_scalar_node() {
        let mut graph = MyGraph::new();

        let node_id1 = graph.add_node(
            "MakeScalar".to_string(),
            MyNodeData {
                template: MyNodeTemplate::MakeScalar,
            },
            |_, _| {},
        );
        nodes::make_scalar::build_node(&mut graph, node_id1);
        let input_id = graph[node_id1].get_input("value").unwrap();
        graph[input_id].value = MyValueType::Scalar { value: 10.0 };

        let node_id2 = graph.add_node(
            "MakeScalar".to_string(),
            MyNodeData {
                template: MyNodeTemplate::MakeScalar,
            },
            |_, _| {},
        );
        nodes::make_scalar::build_node(&mut graph, node_id2);
        let input_id = graph[node_id2].get_input("value").unwrap();
        graph[input_id].value = MyValueType::Scalar { value: 32.0 };

        let add_node_id = graph.add_node(
            "AddScalar".to_string(),
            MyNodeData {
                template: MyNodeTemplate::AddScalar,
            },
            |_, _| {},
        );
        nodes::add_scalar::build_node(&mut graph, add_node_id);

        graph.add_connection(
            graph[node_id1].get_output("out").unwrap(),
            graph[add_node_id].get_input("A").unwrap(),
        );

        graph.add_connection(
            graph[node_id2].get_output("out").unwrap(),
            graph[add_node_id].get_input("B").unwrap(),
        );

        let mut outputs_cache = OutputsCache::new();

        let result = evaluate_node(&graph, add_node_id, &mut outputs_cache).unwrap();

        assert_eq!(result, MyValueType::Scalar { value: 42.0 });
    }

    #[test]
    fn test_evaluate_vector_times_scalar_node() {
        let mut graph = MyGraph::new();

        let vector_node_id = graph.add_node(
            "MakeVector".to_string(),
            MyNodeData {
                template: MyNodeTemplate::MakeVector,
            },
            |_, _| {},
        );
        nodes::make_vector::build_node(&mut graph, vector_node_id);
        let input_x = graph[vector_node_id].get_input("x").unwrap();
        let input_y = graph[vector_node_id].get_input("y").unwrap();

        graph[input_x].value = MyValueType::Scalar { value: 2.0 };
        graph[input_y].value = MyValueType::Scalar { value: 3.0 };

        let scalar_node_id = graph.add_node(
            "MakeScalar".to_string(),
            MyNodeData {
                template: MyNodeTemplate::MakeScalar,
            },
            |_, _| {},
        );
        nodes::make_scalar::build_node(&mut graph, scalar_node_id);
        let input_id = graph[scalar_node_id].get_input("value").unwrap();
        graph[input_id].value = MyValueType::Scalar { value: 4.0 };

        let vector_times_scalar_node_id = graph.add_node(
            "VectorTimesScalar".to_string(),
            MyNodeData {
                template: MyNodeTemplate::VectorTimesScalar,
            },
            |_, _| {},
        );
        nodes::vector_times_scalar::build_node(&mut graph, vector_times_scalar_node_id);

        graph.add_connection(
            graph[vector_node_id].get_output("out").unwrap(),
            graph[vector_times_scalar_node_id]
                .get_input("vector")
                .unwrap(),
        );

        graph.add_connection(
            graph[scalar_node_id].get_output("out").unwrap(),
            graph[vector_times_scalar_node_id]
                .get_input("scalar")
                .unwrap(),
        );

        let mut outputs_cache = OutputsCache::new();

        let result =
            evaluate_node(&graph, vector_times_scalar_node_id, &mut outputs_cache).unwrap();

        assert_eq!(
            result,
            MyValueType::Vec2 {
                value: egui::Vec2::new(8.0, 12.0)
            }
        );
    }

    #[test]
    fn test_evaluate_node_with_missing_input() {
        let mut graph = MyGraph::default();
        let mut outputs_cache = HashMap::new();

        let node_id = graph.add_node(
            "Test Node".to_string(),
            MyNodeData {
                template: MyNodeTemplate::AddScalar,
            },
            |_, _| {}, // Replace `None` with an empty closure
        );

        // We intentionally do NOT connect or set a value for an expected input
        let result = evaluate_node(&graph, node_id, &mut outputs_cache);

        // Check if an error is returned
        assert!(
            result.is_err(),
            "Expected an error due to missing input, but got {:?}",
            result
        );
    }
}
