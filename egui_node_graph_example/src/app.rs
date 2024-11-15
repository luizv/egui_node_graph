// Import the nodes module
use crate::types::*;
use crate::utils::evaluate_node;
use eframe::egui::{self, TextStyle};
use egui_node_graph::*;
use std::collections::HashMap;

/// The main application struct
#[derive(Default)]
pub struct NodeGraphExample {
    // The state of the graph editor
    state: MyEditorState,
    // The registered node types
    node_types: RegisteredNodeTypes,
    // The user-defined state
    user_state: MyGraphState,
}

impl NodeGraphExample {
    /// Constructor for the application
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        #[cfg(feature = "persistence")]
        {
            // Load previous state if persistence is enabled
            let state = cc
                .storage
                .and_then(|storage| eframe::get_value(storage, PERSISTENCE_KEY))
                .unwrap_or_default();
            Self {
                state,
                node_types: RegisteredNodeTypes::default(),
                user_state: MyGraphState::default(),
            }
        }
        #[cfg(not(feature = "persistence"))]
        {
            // Initialize default state
            Self {
                state: MyEditorState::default(),
                node_types: RegisteredNodeTypes::default(),
                user_state: MyGraphState::default(),
            }
        }
    }
}

#[cfg(feature = "persistence")]
const PERSISTENCE_KEY: &str = "egui_node_graph";

impl eframe::App for NodeGraphExample {
    #[cfg(feature = "persistence")]
    /// Save the application state when persistence is enabled
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, PERSISTENCE_KEY, &self.state);
    }

    /// The main update function called by the framework
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Draw the top panel with a theme switcher
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_dark_light_mode_switch(ui);
            });
        });

        // Draw the graph editor in the central panel
        let graph_response = egui::CentralPanel::default()
            .show(ctx, |ui| {
                self.state.draw_graph_editor(
                    ui,
                    &self.node_types,
                    &mut self.user_state,
                    Vec::default(),
                )
            })
            .inner;

        // Process responses from the graph editor
        for node_response in graph_response.node_responses {
            if let NodeResponse::User(user_event) = node_response {
                match user_event {
                    MyResponse::SetActiveNode(node) => self.user_state.active_node = Some(node),
                    MyResponse::ClearActiveNode => self.user_state.active_node = None,
                }
            }
        }

        // If an active node is set, evaluate and display its result
        if let Some(node) = self.user_state.active_node {
            if self.state.graph.nodes.contains_key(node) {
                let text = match evaluate_node(&self.state.graph, node, &mut HashMap::new()) {
                    Ok(value) => format!("The result is: {:?}", value),
                    Err(err) => format!("Execution error: {}", err),
                };
                ctx.debug_painter().text(
                    egui::pos2(10.0, 35.0),
                    egui::Align2::LEFT_TOP,
                    text,
                    TextStyle::Button.resolve(&ctx.style()),
                    egui::Color32::WHITE,
                );
            } else {
                // Clear the active node if it was deleted
                self.user_state.active_node = None;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nodes;
    use std::collections::HashMap;

    #[test]
    fn test_node_graph_example_initialization() {
        let app = NodeGraphExample::default();
        assert!(app.state.graph.nodes.is_empty());
        assert!((&app.node_types).all_kinds().len() > 0);
        assert!(app.user_state.active_node.is_none());
    }

    #[test]
    fn test_set_active_node() {
        let mut app = NodeGraphExample::default();
        let node_id = app.state.graph.add_node(
            "Test Node".to_string(),
            MyNodeData {
                template: MyNodeTemplate::MakeScalar,
            },
            |_, _| {},
        );
        app.user_state.active_node = Some(node_id);
        assert_eq!(app.user_state.active_node, Some(node_id));
    }

    #[test]
    fn test_clear_active_node() {
        let mut app = NodeGraphExample::default();
        let node_id = app.state.graph.add_node(
            "Test Node".to_string(),
            MyNodeData {
                template: MyNodeTemplate::MakeScalar,
            },
            |_, _| {},
        );
        app.user_state.active_node = Some(node_id);
        app.user_state.active_node = None;
        assert!(app.user_state.active_node.is_none());
    }

    #[test]
    fn test_evaluate_node() {
        let mut app = NodeGraphExample::default();
        let node_id = app.state.graph.add_node(
            "MakeScalar".to_string(),
            MyNodeData {
                template: MyNodeTemplate::MakeScalar,
            },
            |_, _| {},
        );
        nodes::make_scalar::build_node(&mut app.state.graph, node_id);
        let input_id = app.state.graph[node_id].get_input("value").unwrap();
        app.state.graph[input_id].value = MyValueType::Scalar { value: 42.0 };
        let mut outputs_cache = HashMap::new();
        let result = evaluate_node(&app.state.graph, node_id, &mut outputs_cache).unwrap();
        assert_eq!(result, MyValueType::Scalar { value: 42.0 });
    }
}
