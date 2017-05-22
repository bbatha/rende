extern crate ordermap;
extern crate stdweb;

mod document;

pub mod component;
pub use document::vdocument::{VDocument, Key, NodeId};

use stdweb::web::{self, INode};
use document::rendered_document::RenderedDocument;

pub fn rende<C: component::Component>(id: &'static str, component: C) {
    stdweb::initialize();

    let entry_node = web::document().query_selector(&format!("#{}", id)).unwrap();
    entry_node.set_text_content("rendered from rust!");

    let initial = RenderedDocument::from_dom(entry_node);
    // loop {
    let user = VDocument::from_component(component);
    let _initial = initial.patch(user);
    // TODO(bbatha): figure out how to compose our event loop with stdweb's
    // also add events...
    // }

    stdweb::event_loop();
}