use bevy::prelude::*;

use bevy_node_editor::{
    node::{NodeIOTemplate, NodeTemplate},
    Node, NodeIO, NodeInput, NodeOutput, NodePlugins, NodeType, OutputNode,
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugins(NodePlugins::<MathNodes>::default())
        .add_startup_system(setup)
        .run();
}

#[derive(Clone, Copy)]
enum MathNodes {
    Add,
    Print,
    Value(f32),
}

impl Default for MathNodes {
    fn default() -> Self {
        Self::Value(0.0)
    }
}

impl NodeType for MathNodes {
    fn resolve(
        &self,
        entity: Entity,
        node: &Node<Self>,
        q_nodes: &Query<(Entity, &Node<Self>), Without<OutputNode>>,
        q_inputs: &Query<(&Parent, &NodeInput<Self>)>,
        q_outputs: &Query<(&Parent, &NodeOutput)>,
    ) -> NodeIO {
        let inputs = node.get_inputs(entity, q_nodes, q_inputs, q_outputs);

        match node.node_type {
            MathNodes::Add => {
                let a: f32 = inputs["a"].into();
                let b: f32 = inputs["b"].into();

                NodeIO::F32(a + b)
            }
            MathNodes::Value(value) => NodeIO::F32(value),
            MathNodes::Print => {
                println!("{:?}", inputs["value"]);
                NodeIO::None
            }
        }
    }
}

impl MathNodes {
    fn to_template(&self, position: Vec2) -> NodeTemplate<Self> {
        match self {
            Self::Add => NodeTemplate {
                position,
                title: "Add".to_string(),
                inputs: Some(vec![
                    NodeIOTemplate {
                        label: "a".to_string(),
                        ..default()
                    },
                    NodeIOTemplate {
                        label: "b".to_string(),
                        ..default()
                    },
                ]),
                output: Some(NodeIOTemplate {
                    label: "result".to_string(),
                    ..default()
                }),
                node_type: *self,
                ..default()
            },
            Self::Print => NodeTemplate {
                position,
                title: "Print".to_string(),
                inputs: Some(vec![NodeIOTemplate {
                    label: "value".to_string(),
                    ..Default::default()
                }]),
                node_type: *self,
                ..default()
            },
            Self::Value(_) => NodeTemplate {
                position,
                title: "Value".to_string(),
                output: Some(NodeIOTemplate {
                    label: "Value".to_string(),
                    ..Default::default()
                }),
                node_type: *self,
                ..Default::default()
            },
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());

    commands
        .spawn()
        .insert(MathNodes::Value(5.0).to_template(Vec2::new(-150.0, 100.0)));

    commands
        .spawn()
        .insert(MathNodes::Value(7.0).to_template(Vec2::new(-150.0, -100.0)));

    commands
        .spawn()
        .insert(MathNodes::Add.to_template(Vec2::new(150.0, 0.0)));

    commands
        .spawn()
        .insert(MathNodes::Print.to_template(Vec2::new(450.0, 0.0)));
}
