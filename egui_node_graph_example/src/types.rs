use crate::nodes;
use crate::utils::evaluate_node;
use crate::utils::Evaluator;
use derivative::Derivative;
use eframe::egui::{self, DragValue};
use eframe::web_sys::console;
use egui_node_graph::*;
use image::GenericImageView;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;
use wasm_bindgen_futures::spawn_local;

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
    Image,
}

/// Input parameters can optionally have a constant value.
#[derive(Derivative, Serialize, Deserialize)]
pub enum MyValueType {
    Vec2 {
        value: egui::Vec2,
    },
    Scalar {
        value: f32,
    },
    Image {
        data: Vec<u8>,
        #[serde(skip)]
        pending_image: Option<futures::channel::oneshot::Receiver<Vec<u8>>>,
    },
}

impl Clone for MyValueType {
    fn clone(&self) -> Self {
        match self {
            MyValueType::Vec2 { value } => MyValueType::Vec2 { value: *value },
            MyValueType::Scalar { value } => MyValueType::Scalar { value: *value },
            MyValueType::Image { data, .. } => MyValueType::Image {
                data: data.clone(),
                pending_image: None,
            },
        }
    }
}

impl PartialEq for MyValueType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (MyValueType::Vec2 { value: a }, MyValueType::Vec2 { value: b }) => a == b,
            (MyValueType::Scalar { value: a }, MyValueType::Scalar { value: b }) => a == b,
            (MyValueType::Image { data: a, .. }, MyValueType::Image { data: b, .. }) => a == b,
            _ => false,
        }
    }
}

impl std::fmt::Debug for MyValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyValueType::Vec2 { value } => f.debug_struct("Vec2").field("value", value).finish(),
            MyValueType::Scalar { value } => {
                f.debug_struct("Scalar").field("value", value).finish()
            }
            MyValueType::Image { data, .. } => {
                f.debug_struct("Image").field("data", &data.len()).finish()
            }
        }
    }
}

impl Default for MyValueType {
    fn default() -> Self {
        Self::Scalar { value: 0.0 }
    }
}

impl MyValueType {
    pub fn default_image() -> Self {
        Self::Image {
            data: vec![],
            pending_image: None,
        }
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
    ImageFilter,
    MakeImage,
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
    pub evaluate: fn(&mut Evaluator<'_>) -> anyhow::Result<MyValueType>,
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
            NodeDefinition {
                template: MyNodeTemplate::ImageFilter,
                build: nodes::image_filter::build_node, // Define this in your nodes module
                evaluate: nodes::image_filter::evaluate, // Define evaluation logic for image filtering
                label: "Image Filter",
                categories: &["Image"],
            },
            NodeDefinition {
                template: MyNodeTemplate::MakeImage,
                build: nodes::make_image::build_node,
                evaluate: nodes::make_image::evaluate,
                label: "Make Image",
                categories: &["Image"],
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
            MyDataType::Image => egui::Color32::from_rgb(255, 105, 180), // Assign a color for images
        }
    }

    fn name(&self) -> Cow<'_, str> {
        match self {
            MyDataType::Scalar => Cow::Borrowed("Scalar"),
            MyDataType::Vec2 => Cow::Borrowed("Vec2"),
            MyDataType::Image => Cow::Borrowed("Image"), // Name for the new data type
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
        _user_state: &mut Self::UserState,
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
            MyValueType::Image {
                data,
                pending_image,
            } => {
                ui.label(param_name);
                ui.label(format!("Image data length: {} bytes", data.len()));

                if ui.button("Load Image").clicked() {
                    let task = rfd::AsyncFileDialog::new()
                        .add_filter("Image files", &["jpg", "jpeg", "png"])
                        .pick_file();

                    // Cria um canal para receber os dados carregados
                    let (tx, rx) = futures::channel::oneshot::channel();

                    // Executa a tarefa assíncrona
                    spawn_local(async move {
                        if let Some(file) = task.await {
                            let loaded_data = file.read().await; // Carrega o conteúdo do arquivo
                            let _ = tx.send(loaded_data); // Envia os dados através do canal
                        } else {
                            let _ = tx.send(Vec::new()); // Envia um vetor vazio se nenhum arquivo for selecionado
                        }
                    });

                    // Armazena o receptor para verificar posteriormente
                    *pending_image = Some(rx);
                }

                // Verifica se recebemos dados no canal
                if let Some(rx) = pending_image {
                    match rx.try_recv() {
                        Ok(Some(loaded_data)) => {
                            *data = loaded_data;
                            *pending_image = None; // Limpa o receptor após receber os dados
                        }
                        Ok(None) => {
                            // Dados ainda não disponíveis, manter o receptor
                        }
                        Err(_) => {
                            // O canal foi fechado ou ocorreu um erro
                            *pending_image = None;
                        }
                    }
                }
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
        graph: &Graph<MyNodeData, MyDataType, MyValueType>,
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

        let mut outputs_cache = HashMap::new();

        // if _graph[node_id].user_data.template == MyNodeTemplate::MakeImage {
        if let Ok(output_value) = evaluate_node(graph, node_id, &mut outputs_cache) {
            if let MyValueType::Image { data, .. } = output_value {
                if !data.is_empty() {
                    // Carregue a imagem usando a biblioteca `image`
                    let image = image::load_from_memory(&data).expect("Failed to load image");
                    let (width, height) = image.dimensions();
                    let image_buffer = image.to_rgba8();
                    let pixels = image_buffer.into_vec();

                    // Verifique se os dados da imagem são válidos
                    if width * height * 4 == pixels.len() as u32 {
                        // Converta os dados da imagem para uma textura egui
                        let texture_id = ui.ctx().load_texture(
                            format!("node_image_{:?}", node_id),
                            egui::ColorImage::from_rgba_unmultiplied(
                                [width as usize, height as usize],
                                &pixels,
                            ),
                            Default::default(),
                        );

                        // Desenhe a imagem
                        ui.add(
                            egui::Image::new(&texture_id)
                                .max_width(300.0)
                                .rounding(10.0),
                        );
                    } else {
                        // Imprima os valores no console do navegador
                        console::log_1(
                            &format!(
                                "width: {}, height: {}, pixels.len(): {}",
                                width,
                                height,
                                pixels.len()
                            )
                            .into(),
                        );
                    }
                }
            }
        } else {
            // Imprima os valores no console do navegador
            console::log_1(&"Invalid input: Expected an image".into());
        }

        // }
        responses
    }
}

pub type MyGraph = Graph<MyNodeData, MyDataType, MyValueType>;
pub type MyEditorState =
    GraphEditorState<MyNodeData, MyDataType, MyValueType, MyNodeTemplate, MyGraphState>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_myvaluetype() {
        let default_value = MyValueType::default();
        assert_eq!(default_value, MyValueType::Scalar { value: 0.0 });
    }

    #[test]
    fn test_try_to_scalar() {
        let value = MyValueType::Scalar { value: 42.0 };
        let result = value.try_to_scalar();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42.0);

        let value = MyValueType::Vec2 {
            value: egui::Vec2::new(1.0, 2.0),
        };
        let result = value.try_to_scalar();
        assert!(result.is_err());
    }

    #[test]
    fn test_try_to_vec2() {
        let value = MyValueType::Vec2 {
            value: egui::Vec2::new(3.0, 4.0),
        };
        let result = value.try_to_vec2();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), egui::Vec2::new(3.0, 4.0));

        let value = MyValueType::Scalar { value: 5.0 };
        let result = value.try_to_vec2();
        assert!(result.is_err());
    }

    #[test]
    fn test_node_definition_retrieval() {
        let definitions = NodeDefinition::all_definitions();
        assert!(definitions.len() > 0);
        assert!(definitions
            .iter()
            .any(|def| def.template == MyNodeTemplate::MakeScalar));
    }

    #[test]
    fn test_node_template_label() {
        let template = MyNodeTemplate::MakeScalar;
        let label = template.node_finder_label(&mut MyGraphState::default());
        assert_eq!(label, Cow::Borrowed("New Scalar"));

        let unknown_template = MyNodeTemplate::MultiplyScalar; // Assuming MultiplyScalar is known
        let label = unknown_template.node_finder_label(&mut MyGraphState::default());
        assert!(label != Cow::Borrowed("Unknown"));
    }

    #[test]
    fn test_build_node() {
        let mut graph = MyGraph::default();
        let node_id = graph.add_node(
            "Test Node".to_string(),
            MyNodeData {
                template: MyNodeTemplate::MakeScalar,
            },
            |_, _| {},
        );

        MyNodeTemplate::MakeScalar.build_node(&mut graph, &mut MyGraphState::default(), node_id);
        let input_exists = graph[node_id].get_input("value").is_ok();
        assert!(input_exists);
    }

    #[test]
    fn test_data_type_color() {
        let scalar_color = MyDataType::Scalar.data_type_color(&mut MyGraphState::default());
        let vec2_color = MyDataType::Vec2.data_type_color(&mut MyGraphState::default());
        assert_ne!(scalar_color, vec2_color);
    }

    // #[test]
    // fn test_widget_value_trait_scalar() {
    //     let mut scalar_value = MyValueType::Scalar { value: 10.0 };
    //     // Skipping `Ui` mock creation as it's complex; you may need integration testing here.
    //     // This part assumes you have a way to test UI interactions separately.
    // }

    // #[test]
    // fn test_widget_value_trait_vec2() {
    //     let mut vec2_value = MyValueType::Vec2 {
    //         value: egui::Vec2::new(1.0, 2.0),
    //     };
    //     // Skipping `Ui` mock creation; similar reasoning as above.
    // }

    #[test]
    fn test_graph_state_default() {
        let state = MyGraphState::default();
        assert!(state.active_node.is_none());
    }
}
