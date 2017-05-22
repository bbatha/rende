use std::collections::{BTreeMap, VecDeque};
use std::hash::{Hasher, Hash};
use std::collections::hash_map::DefaultHasher;

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

impl VNode {
    fn diff(&self, other: &VNode) -> Option<Patch> {
        None
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct VDocument {
    nodes: Vec<VNode>,
    keys: BTreeMap<ParentId, KeyMap>,
    children: BTreeMap<ParentId, Vec<ChildId>>,
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
        let mut children = self.children.entry(parent).or_insert(Vec::new());
        children.push(child);
    }

    pub fn set_key<K: Into<Key>>(&mut self, node: NodeId, key: K, parent: NodeId) {
        let mut keys = self.keys.entry(parent).or_insert(Default::default());
        keys.insert(node, key.into());
    }

    pub fn root(&self) -> NodeId {
        ROOT_ID
    }

    fn diff<'a>(&'a self, new_doc: &'a VDocument) -> DiffIterator<'a> {
        let mut queue = VecDeque::new();
        queue.push_back((self.root(), new_doc.root()));
        DiffIterator {
            queue,
            old_doc: self,
            new_doc,
            parent: None,
            new_child_idx: 0,
            old_child_idx: 0,
            done: false,
        }
    }
}

struct DiffIterator<'a> {
    queue: VecDeque<(NodeId, NodeId)>,
    old_doc: &'a VDocument,
    new_doc: &'a VDocument,
    parent: Option<(NodeId, NodeId)>,
    new_child_idx: usize,
    old_child_idx: usize,
    done: bool,
}

impl<'a> Iterator for DiffIterator<'a> {
    type Item = Patch;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        if let Some((old_parent, new_parent)) = self.parent {
            let old_child = self.old_doc
                .children
                .get(&old_parent)
                .and_then(|children| children.get(self.old_child_idx).cloned());
            let new_child = self.new_doc
                .children
                .get(&new_parent)
                .and_then(|children| children.get(self.new_child_idx).cloned());

            // we are now comparing children. May be useful to split into another iterator and
            // flat_map it but the None, None case makes that hard.
            match (old_child, new_child) {
                (None, Some(id)) => {
                    self.new_child_idx += 1;
                    Some(Patch::Create(id))
                }
                (Some(id), None) => {
                    self.old_child_idx += 1;
                    Some(Patch::Delete(id))
                }
                (Some(old_id), Some(new_id)) => {
                    self.queue.push_back((old_id, new_id));
                    self.old_child_idx += 1;
                    self.new_child_idx += 1;

                    let old_node = &self.old_doc.nodes[old_id.0];
                    let new_node = &self.old_doc.nodes[new_id.0];

                    // because its pushed on the queue it will be returned in the iterator when
                    // popped
                    old_node.diff(new_node).or_else(|| self.next())
                },
                (None, None) => {
                    self.parent = None;
                    self.next()
                }
            }
        } else {
            // compare a local root
            if let Some((new_id, old_id)) = self.queue.pop_front() {
                self.parent = Some((old_id, new_id));
                self.old_child_idx = 0;
                self.new_child_idx = 0;

                Some(Patch::Reuse(new_id, old_id))
            } else {
                self.done = true;
                None
            }
        }
    }
}

#[test]
fn create_element() {
    let div = component::Div::<component::Empty>::new();
    let old_doc = VDocument::default();
    let new_doc = VDocument::from_component(div);

    let patches: Vec<_> = old_doc.diff(&new_doc).collect();
    let expected = vec![Patch::Reuse(ROOT_ID, ROOT_ID), Patch::Create(NodeId(1))];

    assert_eq!(patches, expected);
}


#[test]
fn delete_element() {
    let div = component::Div::<component::Empty>::new();
    let old_doc = VDocument::from_component(div);
    let new_doc = VDocument::default();

    let patches: Vec<_> = old_doc.diff(&new_doc).collect();
    let expected = vec![Patch::Reuse(ROOT_ID, ROOT_ID), Patch::Delete(NodeId(1))];

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

    let patches: Vec<_> = old_doc.diff(&new_doc).collect();
    let expected = vec![Patch::Reuse(ROOT_ID, ROOT_ID),
                        Patch::Reuse(NodeId(1), NodeId(1)),
                        Patch::Delete(NodeId(4)),
                        Patch::Reuse(NodeId(2), NodeId(2)),
                        Patch::Delete(NodeId(3))];

    assert_eq!(patches, expected);
}

