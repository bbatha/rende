#[derive(Debug, Eq, PartialEq)]
pub enum VNode {
    Element {
        tag: &'static str,
        children: Vec<VNode>,
        key: Option<Key>,
    },
    Text(String),
    Empty,
}

impl VNode {
    fn set_key(&mut self, key: Key) {
        match *self {
            VNode::Element{ key: ref mut k, .. } => *k = Some(key),
            _ => (),
        }
    }
}

pub trait Component {
    fn render(&self) -> VNode;
}

impl Component for &'static str {
    fn render(&self) -> VNode {
        VNode::Text(self.to_string())
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
        VNode::Element {
            tag: "div",
            children: self.children.iter().map(Component::render).collect(),
            key: None,
        }
    }
}

impl Component for () {
    fn render(&self) -> VNode {
        VNode::Empty
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Default, Hash, Clone)]
pub struct Key(i32);

pub struct KeyedComponent<C>(Key, C);

impl<C: Component> Component for KeyedComponent<C> {
    fn render(&self) -> VNode {
        let mut vnode = self.1.render();
        vnode.set_key(self.0.clone());
        vnode
    }
}

#[test]
fn render_div() {
    let div: Div<()> = Div::new();
    assert_eq!(VNode::Element{ tag: "div", children: vec![], key: None}, div.render());
}

#[test]
fn render_text_div() {
    let div = Div::with_children(vec!["test"]);
    assert_eq!(VNode::Element{ tag: "div", children: vec![VNode::Text("test".to_string())], key: None}, div.render())
}

#[test]
fn render_keyed() {
    let div: Div<()> = Div::new();
    let keyed = KeyedComponent(Key(3), div);
    assert_eq!(VNode::Element{ tag: "div", children: vec![], key: Some(Key(3))}, keyed.render());
}