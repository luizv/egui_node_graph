// src/utils.rs

use crate::types::{MyGraph, MyNodeTemplate, MyValueType, NodeDefinition};
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
    outputs_cache.insert(output_id, value);
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
            Ok(*other_value)
        } else {
            evaluate_node(graph, graph[other_output_id].node, outputs_cache)?;
            Ok(*outputs_cache
                .get(&other_output_id)
                .expect("Cache should be populated"))
        }
    } else {
        Ok(graph[input_id].value)
    }
}
