use bevy::app::{PluginGroup, PluginGroupBuilder};
use std::marker::PhantomData;

pub mod camera;
pub mod connection;
pub mod cursor;
pub mod interactions;
pub mod menu;
pub mod node;
pub mod widget;
pub mod widgets;

pub use crate::{
    camera::PanCameraPlugin,
    menu::{NodeMenu, NodeMenuPlugin},
    node::{Node, NodeEvent, NodeInput, NodeOutput, NodeSet, NodeSlot, NodeTemplate},
    widget::{Widget, WidgetPlugin},
};

#[derive(Default)]
pub struct NodePlugins<N: NodeSet>(PhantomData<N>);

impl<N: NodeSet> PluginGroup for NodePlugins<N> {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(connection::ConnectionPlugin::<N>::default())
            .add(cursor::CursorPlugin)
            .add(interactions::InteractionPlugin)
            .add(node::NodePlugin::<N>::default())
    }
}
