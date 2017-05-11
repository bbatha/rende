#[derive(Debug, Eq, PartialEq)]
pub enum VNode {
    Element {
        tag: &'static str,
        children: Vec<VNode>,
    },
    Text(String),
    Empty,
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
        }
    }
}

impl Component for () {
    fn render(&self) -> VNode {
        VNode::Empty
    }
}

#[test]
fn render_div() {
    let div: Div<()> = Div::new();
    assert_eq!(VNode::Element{ tag: "div", children: vec![]}, div.render());
}

#[test]
fn render_text_div() {
    let div = Div::with_children(vec!["test"]);
    assert_eq!(VNode::Element{ tag: "div", children: vec![VNode::Text("test".to_string())]}, div.render())
}