use std::collections::{BTreeMap, VecDeque};
use std::hash::{Hasher, Hash};
use std::collections::hash_map::DefaultHasher;

use ordermap;

use component::{self, Component};

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

#[derive(Debug, Eq, PartialEq)]
pub enum Patch {
    Reuse(NodeId, NodeId),
    Create(NodeId),
    Delete(NodeId),
    Append(),
}

type ParentId = NodeId;
type ChildId = NodeId;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Default)]
struct KeyMap {
    key_to_id: BTreeMap<Key, NodeId>,
    id_to_key: BTreeMap<NodeId, Key>,
}

impl KeyMap {
    fn insert(&mut self, id: NodeId, key: Key) {
        let a = self.key_to_id.insert(key, id);
        let b = self.id_to_key.insert(id, key);

        assert!(a.is_none() && b.is_none(), "Key already in use");
    }

    fn get_id(&self, key: Key) -> Option<NodeId> {
        self.key_to_id.get(&key).cloned()
    }

    fn get_key(&self, id: NodeId) -> Option<Key> {
        self.id_to_key.get(&id).cloned()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum VNode {
    Root,
    Element(&'static str),
    Text(String),
}

#[derive(Debug, Eq, PartialEq)]
pub struct VDocument {
    nodes: Vec<VNode>,
    keys: BTreeMap<ParentId, KeyMap>,
    children: BTreeMap<ParentId, ordermap::OrderMap<ChildId, ()>>,
}

impl Default for VDocument {
    fn default() -> Self {
        VDocument {
            nodes: vec![VNode::Root],
            keys: Default::default(),
            children: Default::default(),
        }
    }
}

const ROOT_ID: NodeId = NodeId(0);

impl VDocument {
    pub fn from_component<C: Component>(component: C) -> VDocument {
        let mut new_doc = VDocument::default();
        component
            .render(&mut new_doc)
            .and_then(|child| Some(new_doc.append_child(ROOT_ID, child)));
        new_doc
    }

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
        let mut children = self.children
            .entry(parent)
            .or_insert(ordermap::OrderMap::new());
        children.insert(child, ());
    }

    pub fn set_key<K: Into<Key>>(&mut self, node: NodeId, key: K, parent: NodeId) {
        let mut keys = self.keys.entry(parent).or_insert(Default::default());
        keys.insert(node, key.into());
    }

    pub fn get_root(&self) -> NodeId {
        ROOT_ID
    }

    fn diff(&self, new_document: &VDocument) -> Vec<Patch> {
        let mut patches = Vec::with_capacity(new_document.nodes.len());

        // used as default later
        let empty_children = ordermap::OrderMap::default();

        // simple bfs traversal of the dom tree.
        let mut q = VecDeque::new();
        q.push_back((self.get_root(), new_document.get_root()));
        loop {
            let (old_node_id, new_node_id) = if let Some(n) = q.pop_front() {
                n
            } else {
                break;
            };

            // TODO(bbatha): do property comparisions, etc.
            // assume rearrangement of nodes is the fastest way to patch the dom
            patches.push(Patch::Reuse(old_node_id, new_node_id));

            let old_children = self.children.get(&old_node_id).unwrap_or(&empty_children);
            let new_children = new_document
                .children
                .get(&new_node_id)
                .unwrap_or(&empty_children);

            let mut old_children_iter = old_children.iter();
            let mut new_children_iter = new_children.iter();
            loop {
                let old_child = old_children_iter.next().map(|(n, _)| *n);
                let new_child = new_children_iter.next().map(|(n, _)| *n);

                match (old_child, new_child) {
                    (None, Some(id)) => patches.push(Patch::Create(id)),
                    (Some(id), None) => patches.push(Patch::Delete(id)),
                    (Some(old_id), Some(new_id)) => {
                        q.push_back((old_id, new_id));
                    }
                    (None, None) => break,
                }
            }
        }

        patches
    }
}

#[test]
fn create_element() {
    let div = component::Div::<component::Empty>::new();
    let old_doc = VDocument::default();
    let new_doc = VDocument::from_component(div);

    let patches = old_doc.diff(&new_doc);
    let expected = vec![Patch::Reuse(ROOT_ID, ROOT_ID), Patch::Create(NodeId(1))];

    assert_eq!(patches, expected);
}

#[test]
fn recurse_children() {
    use component::*;
    let div = Div::with_children(vec!["test"]);
    let old_div = Div::with_children(vec![div.clone(), div.clone()]);
    let new_div = Div::with_children(vec![Div::<Empty>::new()]);
    let old_doc = VDocument::from_component(old_div);
    let new_doc = VDocument::from_component(new_div);

    let patches = old_doc.diff(&new_doc);
    let expected = vec![Patch::Reuse(ROOT_ID, ROOT_ID),
                        Patch::Reuse(NodeId(1), NodeId(1)),
                        Patch::Delete(NodeId(4)),
                        Patch::Reuse(NodeId(2), NodeId(2)),
                        Patch::Delete(NodeId(3))];

    assert_eq!(patches, expected);
}

#[test]
fn delete_element() {
    let div = component::Div::<component::Empty>::new();
    let old_doc = VDocument::from_component(div);
    let new_doc = VDocument::default();

    let patches = old_doc.diff(&new_doc);
    let expected = vec![Patch::Reuse(ROOT_ID, ROOT_ID), Patch::Delete(NodeId(1))];

    assert_eq!(patches, expected);
}

