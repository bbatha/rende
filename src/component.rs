use document::{VDocument, NodeId, Key};

use std::hash;
use std::collections::{BTreeSet, BTreeMap};

pub trait Component {
    fn render(&self, doc: &mut VDocument) -> Option<NodeId>;
}

pub trait PropertyComponent: Component {
    type Properties;

    fn properties(&self) -> &Self::Properties;
    fn properties_mut(&mut self) -> &mut Self::Properties;
}

pub trait ParentComponent: Component {
    type Children;

    fn children(&self) -> &Self::Children;
    fn children_mut(&mut self) -> &mut Self::Children;
}

/// Use this type to compose a component and assign it a user friendly Id key
#[derive(Default, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
pub struct Keyed<C, K> {
    component: C,
    key: K,
    parent_id: NodeId,
}

impl<K: Into<Key> + hash::Hash, C: Component> Component for Keyed<C, K> {
    fn render(&self, doc: &mut VDocument) -> Option<NodeId> {
        self.component
            .render(doc)
            .and_then(|id| {
                          doc.set_key(id, &self.key, self.parent_id);
                          Some(id)
                      })
    }
}

impl<K: Into<Key> + hash::Hash, C: ParentComponent> ParentComponent for Keyed<C, K> {
    type Children = C::Children;

    fn children(&self) -> &Self::Children {
        self.component.children()
    }
    fn children_mut(&mut self) -> &mut Self::Children {
        self.component.children_mut()
    }
}

impl<K: Into<Key> + hash::Hash, C: PropertyComponent> PropertyComponent for Keyed<C, K> {
    type Properties = C::Properties;

    fn properties(&self) -> &Self::Properties {
        self.component.properties()
    }
    fn properties_mut(&mut self) -> &mut Self::Properties {
        self.component.properties_mut()
    }
}

impl Component for &'static str {
    fn render(&self, doc: &mut VDocument) -> Option<NodeId> {
        Some(doc.create_text_node(*self))
    }
}

/// Empty type to use in place of `()` for `Div`s with no children. This
/// type is needed until specialization is stabilized so that `render`
/// can properly handle no children.
#[derive(Default, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
pub struct Empty;

impl Component for Empty {
    fn render(&self, doc: &mut VDocument) -> Option<NodeId> {
        None
    }
}

/// HTML div with children and properties
#[derive(Default, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
pub struct Div<C = Empty> {
    children: Vec<C>,
    properties: BTreeMap<String, BTreeSet<String>>,
}

impl<C> Div<C> {
    pub fn new() -> Self {
        Div {
            properties: BTreeMap::new(),
            children: Vec::new(),
        }
    }
    pub fn with_properties(properties: BTreeMap<String, BTreeSet<String>>) -> Self {
        Div {
            properties,
            children: Vec::new(),
        }
    }
}

impl<C: Component> Div<C> {
    pub fn with_children(children: Vec<C>) -> Self {
        Div {
            properties: BTreeMap::new(),
            children,
        }
    }
}

impl<C: Component> Component for Div<C> {
    fn render(&self, doc: &mut VDocument) -> Option<NodeId> {
        let parent = doc.create_element("div");
        for child in self.children.iter() {
            child
                .render(doc)
                .and_then(|child| {
                              doc.append_child(parent, child);
                              Some(child)
                          });
        }
        Some(parent)
    }
}

impl<C: Component> ParentComponent for Div<C> {
    type Children = Vec<C>;

    fn children(&self) -> &Self::Children {
        &self.children
    }

    fn children_mut(&mut self) -> &mut Self::Children {
        &mut self.children
    }
}

impl<C: Component> PropertyComponent for Div<C> {
    type Properties = BTreeMap<String, BTreeSet<String>>;

    fn properties(&self) -> &Self::Properties {
        &self.properties
    }

    fn properties_mut(&mut self) -> &mut Self::Properties {
        &mut self.properties
    }
}

#[test]
fn smoke() {
    let mut div = Div::with_children(vec![Empty]);
    div.children_mut().push(Empty);
    div.properties_mut().insert("test".into(), BTreeSet::new());
}

