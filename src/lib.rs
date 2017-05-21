extern crate ordermap;
extern crate stdweb;

pub mod component;
pub mod document;

use stdweb::web::{self, INode};

pub fn rende<C: component::Component>(id: &'static str, component: C) {
    stdweb::initialize();

    let entry_node = web::document().query_selector(&format!("#{}", id)).unwrap();
    entry_node.set_text_content("rendered from rust!");

    let initial = document::RenderedDocument::from_dom(entry_node);
    // loop {
    let user = document::VDocument::from_component(component);
    let _initial = initial.patch(user);
    // TODO(bbatha): figure out how to compose our event loop with stdweb's
    // also add events...
    // }

    stdweb::event_loop();
}