// src/types.rs

use crate::nodes;
use crate::utils::Evaluator;
use eframe::egui::{self, DragValue};
use egui_node_graph::*;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// The NodeData holds a custom data struct inside each node.
#[derive(Debug, Serialize, Deserialize)]
pub struct MyNodeData {
    pub template: MyNodeTemplate,
}

/// `DataType`s define the possible range of connections when attaching two ports together.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MyDataType {
    Scalar,
    Vec2,
}

/// Input parameters can optionally have a constant value.
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum MyValueType {
    Vec2 { value: egui::Vec2 },
    Scalar { value: f32 },
}

impl Default for MyValueType {
    fn default() -> Self {
        Self::Scalar { value: 0.0 }
    }
}

impl MyValueType {
    /// Tries to downcast this value type to a vector
    pub fn try_to_vec2(self) -> anyhow::Result<egui::Vec2> {
        if let MyValueType::Vec2 { value } = self {
            Ok(value)
        } else {
            anyhow::bail!("Invalid cast from {:?} to Vec2", self)
        }
    }

    /// Tries to downcast this value type to a scalar
    pub fn try_to_scalar(self) -> anyhow::Result<f32> {
        if let MyValueType::Scalar { value } = self {
            Ok(value)
        } else {
            anyhow::bail!("Invalid cast from {:?} to Scalar", self)
        }
    }
}

/// NodeTemplate is a mechanism to define node templates.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum MyNodeTemplate {
    MakeScalar,
    AddScalar,
    SubtractScalar,
    MultiplyScalar,
    MakeVector,
    AddVector,
    SubtractVector,
    VectorTimesScalar,
}

/// The response type is used to encode side-effects produced when drawing a node in the graph.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MyResponse {
    SetActiveNode(NodeId),
    ClearActiveNode,
}

/// The graph 'global' state.
#[derive(Default, Serialize, Deserialize)]
pub struct MyGraphState {
    pub active_node: Option<NodeId>,
}

/// Define NodeDefinition struct
pub struct NodeDefinition {
    pub template: MyNodeTemplate,
    pub build: fn(&mut MyGraph, NodeId),
    pub evaluate: fn(&mut Evaluator) -> anyhow::Result<MyValueType>,
    pub label: &'static str,
    pub categories: &'static [&'static str],
}

impl NodeDefinition {
    pub fn all_definitions() -> &'static [NodeDefinition] {
        &[
            NodeDefinition {
                template: MyNodeTemplate::MakeScalar,
                build: nodes::make_scalar::build_node,
                evaluate: nodes::make_scalar::evaluate,
                label: "New Scalar",
                categories: &["Scalar"],
            },
            NodeDefinition {
                template: MyNodeTemplate::AddScalar,
                build: nodes::add_scalar::build_node,
                evaluate: nodes::add_scalar::evaluate,
                label: "Scalar Add",
                categories: &["Scalar"],
            },
            NodeDefinition {
                template: MyNodeTemplate::SubtractScalar,
                build: nodes::subtract_scalar::build_node,
                evaluate: nodes::subtract_scalar::evaluate,
                label: "Scalar Subtract",
                categories: &["Scalar"],
            },
            NodeDefinition {
                template: MyNodeTemplate::MultiplyScalar,
                build: nodes::multiply_scalar::build_node,
                evaluate: nodes::multiply_scalar::evaluate,
                label: "Scalar Multiply",
                categories: &["Scalar"],
            },
            NodeDefinition {
                template: MyNodeTemplate::MakeVector,
                build: nodes::make_vector::build_node,
                evaluate: nodes::make_vector::evaluate,
                label: "New Vector",
                categories: &["Vector"],
            },
            NodeDefinition {
                template: MyNodeTemplate::AddVector,
                build: nodes::add_vector::build_node,
                evaluate: nodes::add_vector::evaluate,
                label: "Vector Add",
                categories: &["Vector"],
            },
            NodeDefinition {
                template: MyNodeTemplate::SubtractVector,
                build: nodes::subtract_vector::build_node,
                evaluate: nodes::subtract_vector::evaluate,
                label: "Vector Subtract",
                categories: &["Vector"],
            },
            NodeDefinition {
                template: MyNodeTemplate::VectorTimesScalar,
                build: nodes::vector_times_scalar::build_node,
                evaluate: nodes::vector_times_scalar::evaluate,
                label: "Vector Times Scalar",
                categories: &["Vector", "Scalar"],
            },
        ]
    }
}

// Implement DataTypeTrait for MyDataType
impl DataTypeTrait<MyGraphState> for MyDataType {
    fn data_type_color(&self, _user_state: &mut MyGraphState) -> egui::Color32 {
        match self {
            MyDataType::Scalar => egui::Color32::from_rgb(38, 109, 211),
            MyDataType::Vec2 => egui::Color32::from_rgb(238, 207, 109),
        }
    }

    fn name(&self) -> Cow<'_, str> {
        match self {
            MyDataType::Scalar => Cow::Borrowed("Scalar"),
            MyDataType::Vec2 => Cow::Borrowed("Vec2"),
        }
    }
}

// Implement NodeTemplateTrait for MyNodeTemplate
impl NodeTemplateTrait for MyNodeTemplate {
    type NodeData = MyNodeData;
    type DataType = MyDataType;
    type ValueType = MyValueType;
    type UserState = MyGraphState;
    type CategoryType = &'static str;

    fn node_finder_label(&self, _user_state: &mut Self::UserState) -> Cow<'_, str> {
        if let Some(def) = NodeDefinition::all_definitions()
            .iter()
            .find(|def| def.template == *self)
        {
            Cow::Borrowed(def.label)
        } else {
            Cow::Borrowed("Unknown")
        }
    }

    fn node_finder_categories(&self, _user_state: &mut Self::UserState) -> Vec<&'static str> {
        if let Some(def) = NodeDefinition::all_definitions()
            .iter()
            .find(|def| def.template == *self)
        {
            def.categories.to_vec()
        } else {
            vec![]
        }
    }

    fn node_graph_label(&self, user_state: &mut Self::UserState) -> String {
        self.node_finder_label(user_state).into()
    }

    fn user_data(&self, _user_state: &mut Self::UserState) -> Self::NodeData {
        MyNodeData { template: *self }
    }

    fn build_node(
        &self,
        graph: &mut Graph<Self::NodeData, Self::DataType, Self::ValueType>,
        user_state: &mut Self::UserState,
        node_id: NodeId,
    ) {
        if let Some(def) = NodeDefinition::all_definitions()
            .iter()
            .find(|def| def.template == *self)
        {
            (def.build)(graph, node_id);
        } else {
            panic!("Unknown node template: {:?}", self);
        }
    }
}

pub struct RegisteredNodeTypes;

impl Default for RegisteredNodeTypes {
    fn default() -> Self {
        Self
    }
}

impl NodeTemplateIter for &RegisteredNodeTypes {
    type Item = MyNodeTemplate;

    fn all_kinds(&self) -> Vec<Self::Item> {
        NodeDefinition::all_definitions()
            .iter()
            .map(|def| def.template)
            .collect()
    }
}

// Implement WidgetValueTrait for MyValueType
impl WidgetValueTrait for MyValueType {
    type Response = MyResponse;
    type UserState = MyGraphState;
    type NodeData = MyNodeData;
    fn value_widget(
        &mut self,
        param_name: &str,
        _node_id: NodeId,
        ui: &mut egui::Ui,
        _user_state: &mut MyGraphState,
        _node_data: &MyNodeData,
    ) -> Vec<MyResponse> {
        match self {
            MyValueType::Vec2 { value } => {
                ui.label(param_name);
                ui.horizontal(|ui| {
                    ui.label("x");
                    ui.add(DragValue::new(&mut value.x));
                    ui.label("y");
                    ui.add(DragValue::new(&mut value.y));
                });
            }
            MyValueType::Scalar { value } => {
                ui.horizontal(|ui| {
                    ui.label(param_name);
                    ui.add(DragValue::new(value));
                });
            }
        }
        Vec::new()
    }
}

impl UserResponseTrait for MyResponse {}

// Implement NodeDataTrait for MyNodeData
impl NodeDataTrait for MyNodeData {
    type Response = MyResponse;
    type UserState = MyGraphState;
    type DataType = MyDataType;
    type ValueType = MyValueType;

    fn bottom_ui(
        &self,
        ui: &mut egui::Ui,
        node_id: NodeId,
        _graph: &Graph<MyNodeData, MyDataType, MyValueType>,
        user_state: &mut Self::UserState,
    ) -> Vec<NodeResponse<MyResponse, MyNodeData>> {
        let mut responses = vec![];
        let is_active = user_state
            .active_node
            .map(|id| id == node_id)
            .unwrap_or(false);

        if !is_active {
            if ui.button("👁 Set active").clicked() {
                responses.push(NodeResponse::User(MyResponse::SetActiveNode(node_id)));
            }
        } else {
            let button =
                egui::Button::new(egui::RichText::new("👁 Active").color(egui::Color32::BLACK))
                    .fill(egui::Color32::GOLD);
            if ui.add(button).clicked() {
                responses.push(NodeResponse::User(MyResponse::ClearActiveNode));
            }
        }

        responses
    }
}

pub type MyGraph = Graph<MyNodeData, MyDataType, MyValueType>;
pub type MyEditorState =
    GraphEditorState<MyNodeData, MyDataType, MyValueType, MyNodeTemplate, MyGraphState>;
