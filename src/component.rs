use document::{VDocument, NodeId, Key};
use std::hash;

pub trait Component {
    fn render(&self, doc: &mut VDocument) -> Option<NodeId>;
}

impl Component for &'static str {
    fn render(&self, doc: &mut VDocument) -> Option<NodeId> {
        Some(doc.create_text_node(*self))
    }
}

/// HTML Div element
#[derive(Debug, Default, Eq, PartialEq)]
pub struct Div<C> {
    children: Vec<C>
}

impl<C> Div<C> {
    pub fn new() -> Self {
        Div {
            children: Vec::new(),
        }
    }
}

impl<C: Component> Div<C> {
    pub fn with_children(children: Vec<C>) -> Self {
        Div {
            children
        }
    }
}

impl<C: Component> Component for Div<C> {
    fn render(&self, doc: &mut VDocument) -> Option<NodeId> {
        let div_id = doc.create_element("div");
        for child in self.children.iter() {
            child.render(doc).and_then(|child| Some(doc.append_child(div_id, child)));
        }
        Some(div_id)
    }
}

impl Component for Div<()> {
    fn render(&self, doc: &mut VDocument) -> Option<NodeId> {
        Some(doc.create_element("div"))
    }
}

/// Use this type to compose a component and assign it a user friendly Id key
#[derive(Debug, Default, Eq, PartialEq)]
pub struct KeyedComponent<C, K>(C, K, NodeId);

impl<K: Into<Key> + hash::Hash, C: Component> Component for KeyedComponent<K, C> {
    fn render(&self, doc: &mut VDocument) -> Option<NodeId> {
        self.1.render(doc).and_then(|id| {
            doc.set_key(id, &self.0, self.2);
            Some(id)
        })
    }
}