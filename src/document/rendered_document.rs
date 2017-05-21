use std::collections::BTreeMap;

use stdweb::web::INode;

use document::vdocument::{VDocument, NodeId};

#[derive(Debug, Default)]
pub struct RenderedDocument<I> {
    vdoc: VDocument,
    dom_nodes: BTreeMap<NodeId, I>,
}

impl<I: INode> RenderedDocument<I> {
    pub fn from_dom(node: I) -> Self {
        let vdoc = VDocument::default();
        let mut dom_nodes = BTreeMap::new();
        dom_nodes.insert(vdoc.get_root(), node);
        RenderedDocument { vdoc, dom_nodes }
    }

    pub fn patch(self, _new_document: VDocument) -> Self {
        unimplemented!();
    }

    fn associate(&mut self, id: NodeId, node: I) -> Option<I> {
        self.dom_nodes.insert(id, node)
    }
}

