extern crate rende;

use rende::component::{Component, Div};
use rende::rende;

fn main() {
    let content = "Rende!";
    // TODO(bbatha): need equalivalent of "innerText" for div
    let div = Div::with_children(vec![content]);
    rende("rende-entry", div);
}

