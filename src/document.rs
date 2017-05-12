use std::collections::BTreeMap;
use ordermap;
use std::hash::{Hasher, Hash};
use std::collections::hash_map::DefaultHasher;

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Default, Debug, Copy, Clone)]
pub struct NodeId(usize);

#[derive(Ord, PartialOrd, Eq, PartialEq, Default, Debug, Copy, Clone)]
pub struct Key(u64);

impl<H: Hash> From<H> for Key {
    fn from(h: H) -> Key {
        let mut hasher = DefaultHasher::new();
        h.hash(&mut hasher);

        Key(hasher.finish())
    }
}

type ParentId = NodeId;
type ChildId = NodeId;

enum VNode {
    Element(&'static str),
    Text(String),
}

pub struct VDocument {
    nodes: Vec<VNode>,
    key_to_index: BTreeMap<Key, NodeId>,
    children: BTreeMap<ParentId, ordermap::OrderMap<ChildId, ()>>,
}

impl VDocument {
    pub fn create_element(&mut self, tag: &'static str) -> NodeId {
        let node = VNode::Element(tag);
        let next_index = self.nodes.len();
        self.nodes.push(node);
        
        NodeId(next_index)
    } 

    pub fn create_text_node<I: Into<String>>(&mut self, content: I) -> NodeId {
        let node = VNode::Text(content.into());
        let next_index = self.nodes.len();
        self.nodes.push(node);

        NodeId(next_index)
    }
    
    pub fn append_child(&mut self, parent: NodeId, child: NodeId) {
        let mut children = self.children.entry(parent).or_insert(ordermap::OrderMap::new());
        children.insert(child, ());
    }

    pub fn set_key<K: Into<Key>>(&mut self, node: NodeId, key: K) {
        let previous_id = self.key_to_index.insert(key.into(), node);
        assert!(previous_id.is_none(), "Key already in use");
    }
}