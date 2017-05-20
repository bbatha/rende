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

/// Composes a vector of child components
#[derive(Default, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
pub struct Parent<P, C> {
    parent: P,
    children: Vec<C>,
}

/// Implementors of this trait can be composed into `Parent` and have a basic implementation
/// of `ParentComponent` provided to you
pub trait ParentAllowed {}
impl<C: ParentComponent> ParentAllowed for C {}

impl<P: ParentAllowed + Component, C: Component> Parent<P, C> {
    pub fn new(parent: P) -> Self {
        Parent {
            parent,
            children: Vec::new(),
        }
    }

    pub fn with_children(parent: P, children: Vec<C>) -> Self {
        Parent { parent, children }
    }
}

impl<P: Component, C: Component> Component for Parent<P, C> {
    fn render(&self, doc: &mut VDocument) -> Option<NodeId> {
        if let Some(parent_id) = self.parent.render(doc) {
            for child in self.children.iter() {
                child
                    .render(doc)
                    .and_then(|child| Some(doc.append_child(parent_id, child)));
            }
            Some(parent_id)
        } else {
            None
        }
    }
}

impl<P: Component, C: Component> ParentComponent for Parent<P, C> {
    type Children = Vec<C>;

    fn children(&self) -> &Self::Children {
        &self.children
    }
    fn children_mut(&mut self) -> &mut Self::Children {
        &mut self.children
    }
}

impl<P: PropertyComponent, C: Component> PropertyComponent for Parent<P, C> {
    type Properties = P::Properties;

    fn properties(&self) -> &Self::Properties {
        self.parent.properties()
    }
    fn properties_mut(&mut self) -> &mut Self::Properties {
        self.parent.properties_mut()
    }
}

#[derive(Default, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
pub struct Propertied<C> {
    component: C,
    properties: BTreeMap<String, BTreeSet<String>>,
}

/// Implementors of this trait can be composed into `Propertied` and have a basic implementation
/// of `PropertyComponent` provided to you
pub trait PropertiesAllowed {}
impl<C: PropertyComponent> PropertiesAllowed for C {}

impl<C: PropertiesAllowed + Component> Propertied<C> {
    pub fn new(component: C) -> Self {
        Propertied {
            component,
            properties: BTreeMap::new(),
        }
    }

    // TODO(bbatha): don't expose that our properties are a BTree* in our public api
    pub fn with_properties(component: C, properties: BTreeMap<String, BTreeSet<String>>) -> Self {
        Propertied {
            component,
            properties,
        }
    }
}

impl<C: Component> Component for Propertied<C> {
    fn render(&self, doc: &mut VDocument) -> Option<NodeId> {
        if let Some(parent_id) = self.component.render(doc) {
            // TODO(bbatha): merge properties
            Some(parent_id)
        } else {
            None
        }
    }
}

impl<C: ParentComponent> ParentComponent for Propertied<C> {
    type Children = C::Children;

    fn children(&self) -> &Self::Children {
        &self.component.children()
    }
    fn children_mut(&mut self) -> &mut Self::Children {
        self.component.children_mut()
    }
}

impl<C: Component> PropertyComponent for Propertied<C> {
    type Properties = BTreeMap<String, BTreeSet<String>>;

    fn properties(&self) -> &Self::Properties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut Self::Properties {
        &mut self.properties
    }
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

/// HTML Div element with no children and no proprieties. These
/// can be composed with `PropertiedComponent` and `ParentComponent`
/// user key names can be added with `Keyed`.
#[derive(Default, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
pub struct EmptyDiv;

impl Component for EmptyDiv {
    fn render(&self, doc: &mut VDocument) -> Option<NodeId> {
        Some(doc.create_element("div"))
    }
}

impl PropertiesAllowed for EmptyDiv {}
impl ParentAllowed for EmptyDiv {}

/// HTML div with children and properties
#[derive(Default, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
pub struct Div<C>(Parent<Propertied<EmptyDiv>, C>);

impl<C: PropertiesAllowed + ParentAllowed + Component> Div<C> {
    pub fn new() -> Self {
        Div(Parent {
                parent: Propertied::new(EmptyDiv),
                children: Vec::new(),
            })
    }

    pub fn with_children(children: Vec<C>) -> Self {
        Div(Parent {
                parent: Propertied::new(EmptyDiv),
                children,
            })
    }

    pub fn with_properties(props: BTreeMap<String, BTreeSet<String>>) -> Self {
        Div(Parent {
                parent: Propertied::with_properties(EmptyDiv, props),
                children: Vec::new(),
            })
    }
}

impl<C: Component> Component for Div<C> {
    fn render(&self, doc: &mut VDocument) -> Option<NodeId> {
        self.0.render(doc)
    }
}

impl<C: Component> ParentComponent for Div<C> {
    type Children = Vec<C>;

    fn children(&self) -> &Self::Children {
        self.0.children()
    }

    fn children_mut(&mut self) -> &mut Self::Children {
        self.0.children_mut()
    }
}

impl<C: Component> PropertyComponent for Div<C> {
    type Properties = BTreeMap<String, BTreeSet<String>>;

    fn properties(&self) -> &Self::Properties {
        self.0.properties()
    }

    fn properties_mut(&mut self) -> &mut Self::Properties {
        self.0.properties_mut()
    }
}

#[test]
fn smoke() {
    let mut div = Div::with_children(vec![EmptyDiv]);
    div.children_mut().push(EmptyDiv);
    div.properties_mut().insert("test".into(), BTreeSet::new());
}
