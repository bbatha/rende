use std::hash::{self, Hasher};
use std::{iter, collections};
use ordermap;
use snowflake;

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
    pub(crate) fn new_text<I: Into<String>>(text: I) -> VNode {
        let mut vn = VNode {
            key: 0,
            ty: VNodeType::Text(text.into()),
        };
        vn.set_key(&snowflake::ProcessUniqueId::new());
        vn
    }

    pub(crate) fn new_element<I: iter::IntoIterator<Item=VNode>>(tag: &'static str, children: I) -> VNode {
        let children = children.into_iter().map(|n| (n.key, n)).collect();
        let mut vn: VNode = VNode {
            key: 0,
            ty: VNodeType::Element{ tag, children }
        };
        vn.set_key(&snowflake::ProcessUniqueId::new());
        vn
    }

    pub(crate) fn new_empty() -> VNode {
        let mut vn = VNode {
            key: 0,
            ty: VNodeType::Empty,
        };
        vn.set_key(&snowflake::ProcessUniqueId::new());
        vn
    }

    pub(crate) fn set_key<H: hash::Hash>(&mut self, key: &H) {
        let mut hasher = collections::hash_map::DefaultHasher::new();
        key.hash(&mut hasher);
        self.key = hasher.finish();
    }
}