use std::hash;
use vnode::VNode;

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