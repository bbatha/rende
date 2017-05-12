use id_tree;
use std::collections::BTreeMap;
use ordermap;
use vnode::*;

struct NodeId(usize);
struct Key(u64);

type ParentId = NodeId;
type ChildId = NodeId;

enum VNode {
    Element(&'static str),
    Text(String),
}

struct VDocument {
    nodes: Vec<VNode>,
    key_to_index: BTreeMap<Key, NodeId>,
    children: BTreeMap<ParentId, ordermap::OrderMap<ChildId, ()>>,
}

render(&documnet) {
    let parent_id = document.create('div');
    let child_id = child.render(document);
    document.set_key(child_id, key);
    document.append(child_id, parent_id);

    return parent_id;
}

impl VDocument {
    fn create_element(&mut self, tag: &'static str) -> NodeId {
        let node = VNode::Element(tag);
        let next_index = self.nodes.len();
        self.nodes.push(node);
        
        NodeId(next_index)
    } 
    
    fn append_child(ParentId, ChildId) {
    }

    fn set_key(ChildId, Key) {
    }
}