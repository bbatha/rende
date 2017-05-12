use document::{VDocument, NodeId, Key};
use std::hash;

pub trait Component {
    fn render(&self, doc: &mut VDocument) -> NodeId;
}

impl Component for &'static str {
    fn render(&self, doc: &mut VDocument) -> NodeId {
        doc.create_text_node(*self)
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
    fn render(&self, doc: &mut VDocument) -> NodeId {
        let div_id = doc.create_element("div");
        for child in self.children.iter() {
            let child_id = child.render(doc);
            doc.append_child(div_id, child_id);
        }
        div_id
    }
}

impl Component for Div<()> {
    fn render(&self, doc: &mut VDocument) -> NodeId {
        doc.create_element("div")
    }
}

/// Use this type to compose a component and assign it a user friendly Id key
#[derive(Debug, Default, Eq, PartialEq)]
pub struct KeyedComponent<C, K>(C, K, NodeId);

impl<K: Into<Key> + hash::Hash, C: Component> Component for KeyedComponent<K, C> {
    fn render(&self, doc: &mut VDocument) -> NodeId {
        let id = self.1.render(doc);
        doc.set_key(id, &self.0, self.2);
        id
    }
}