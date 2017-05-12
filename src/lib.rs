extern crate ordermap;
extern crate snowflake;

use std::hash;

#[derive(Debug, Eq)]
pub struct VNode {
    ty: VNodeType,
    key: u64,
} 

#[derive(Debug, Eq, PartialEq)]
enum VNodeType {
    Element {
        tag: &'static str,
        children: ordermap::OrderMap<u64, VNode>,
    },
    Text(String),
    Empty,
}

impl hash::Hash for VNode {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.key.hash(state)
    }
}

impl PartialEq for VNode {
    fn eq(&self, other: &VNode) -> bool {
        self.key == other.key
    }
}

impl VNode {
    fn new_text<I: Into<String>>(text: I) -> VNode {
        let mut vn = VNode {
            key: 0,
            ty: VNodeType::Text(text.into()),
        };
        vn.set_key(&snowflake::ProcessUniqueId::new());
        vn
    }

    fn new_element<I: std::iter::IntoIterator<Item=VNode>>(tag: &'static str, children: I) -> VNode {
        let children = children.into_iter().map(|n| (n.key, n)).collect();
        let mut vn: VNode = VNode {
            key: 0,
            ty: VNodeType::Element{ tag, children }
        };
        vn.set_key(&snowflake::ProcessUniqueId::new());
        vn
    }

    fn new_empty() -> VNode {
        let mut vn = VNode {
            key: 0,
            ty: VNodeType::Empty,
        };
        vn.set_key(&snowflake::ProcessUniqueId::new());
        vn
    }

    fn set_key<H: hash::Hash>(&mut self, key: &H) {
        use std::hash::Hasher;
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        key.hash(&mut hasher);
        self.key = hasher.finish();
    }
}

pub trait Component {
    fn render(&self) -> VNode;
}

impl Component for &'static str {
    fn render(&self) -> VNode {
        VNode::new_text(self.to_string())
    }
}

#[derive(Default, Eq, PartialEq)]
pub struct Div<C> {
    children: Vec<C>
}

impl<C: Component> Div<C> {
    pub fn new() -> Self {
        Div {
            children: Vec::new(),
        }
    }

    pub fn with_children(children: Vec<C>) -> Self {
        Div {
            children
        }
    }
}

impl<C: Component> Component for Div<C> {
    fn render(&self) -> VNode {
        VNode::new_element("div", self.children.iter().map(Component::render))
    }
}

impl Component for () {
    fn render(&self) -> VNode {
        VNode::new_empty()
    }
}

pub struct KeyedComponent<K, C>(K, C);

impl<K: hash::Hash, C: Component> Component for KeyedComponent<K, C> {
    fn render(&self) -> VNode {
        let mut vnode = self.1.render();
        vnode.set_key(&self.0);
        vnode
    }
}

#[test]
fn render_keyed() {
    let key = 3;
    let div: Div<()> = Div::new();
    let keyed = KeyedComponent(key, div);
    let mut actual = VNode::new_element("div", Vec::new());
    actual.set_key(&key);
    assert_eq!(actual, keyed.render());
}