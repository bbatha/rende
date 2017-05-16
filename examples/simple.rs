extern crate stdweb;
extern crate rende;

use stdweb::web::{document, INode};

fn main() {
    stdweb::initialize();

    let entry_node = document().query_selector("#rende-entry").unwrap();
    entry_node.set_text_content("rendered from rust!");
    stdweb::event_loop();
}
