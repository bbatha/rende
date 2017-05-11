extern crate id_tree;

use id_tree::{Tree, Node, NodeId, NodeIdError, InsertBehavior};

struct VNode {
    tag: String,
    children: Vec<VNode>,
};

impl VNode {
    fn new_root() -> VNode {
        VNode()
    }
}

trait Render {
    fn render(&self, tree: &mut VTree);
}



impl VTree {
    fn new() -> VTree { 
        let mut tree = Tree::new();
        let root = VNode::new_root();
        let root_id = tree.insert(Node::new(root), InsertBehavior::AsRoot).expect("fresh");
        VTree {
            tree: tree,
            cursor: root_id.clone()
         }
    }

    fn insert_vnode(&mut self, vnode: VNode) -> Result<NodeId, NodeIdError> {
        self.tree.insert(Node::new(vnode), InsertBehavior::UnderNode(&self.cursor))
    }

    fn insert_children<R: Render>(&mut self, parent: NodeId, children: &[R]) {
        let prev_cursor = self.cursor.clone();
        self.cursor = parent;

        for renderable in children {
            renderable.render(self);
        }

        self.cursor = prev_cursor;
    }

    fn create_patch_set(&self, old_tree: &VTree) {
    }
}

impl Div {
    fn new<R: Render>(children: &[R]) {
    }
    
    fn render() -> VNode {
        let childVNodes = Vec::new();
        
        for child in this.children {
            childVNodes.push(generateTree(child));
        }

        return VNode {
            tag: 'div',
            children: childVNodes,
        };
    }
}


impl Component {
    fn render() -> VNode {
        div = Div::new();
        child = OtherComponent::new();
        div.appendChild(OtherComponent);
        return div.render();


        div.appendChild(OtherComponent.render());
    }
}

let lastTree = EMPTY_TREE;
loop {
    let component = MyComponent::new(currentState);
    h(component)
    let newTree = MyComponent;
    let patchSet = patch(lastTree, newTree);
    applyPatch(patchSet, domElement);
    lastTree = newTee;
}